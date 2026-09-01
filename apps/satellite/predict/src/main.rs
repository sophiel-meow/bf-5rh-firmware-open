#![no_std]
#![no_main]

use bf5rh_abi::{Api, AppEvent, AppResult};
use sgp4mini::{
    days_since_j2000_from_utc_secs, gmst_rad, look, Look, Observer, Record,
};

#[path = "../../common.rs"]
mod common;
use common::*;

static mut API_PTR: *const Api = core::ptr::null();
/// One buffer for two jobs that never overlap: the raw satellite record while
/// it is being decoded, then the sample chunks on their way to flash. It lives
/// in `.bss` rather than on the stack because the stack under an oversized
/// segment is the scarcer of the two.
static mut SCRATCH: [u8; RECORD_SIZE] = [0; RECORD_SIZE];

const COARSE_STEP: u32 = 120;

const HORIZON_S: u32 = 24 * 3600;

const MIN_MAX_EL_CDEG: i16 = 500;

const BISECT_ROUNDS: u32 = 10;

const DEG_PER_RAD_E2: f64 = 5729.577951308232;

const RAD_PER_COORD_UNIT: f64 = core::f64::consts::PI / 180.0 / 100_000.0;

const FLUSH_SAMPLES: usize = 16;

struct Ctx<'a> {
    rec: &'a Record,
    obs: Observer,
}

impl Ctx<'_> {
    /// `None` when the propagator refuses the epoch: a decayed orbit, or a
    /// record this build cannot integrate.
    fn look_at(&self, t: u32) -> Option<Look> {
        let days = days_since_j2000_from_utc_secs(t);
        let state = self
            .rec
            .constants
            .propagate(self.rec.minutes_since_epoch(days))
            .ok()?;
        Some(look(
            &self.obs,
            gmst_rad(days),
            state.position,
            state.velocity,
        ))
    }

    fn elevation(&self, t: u32) -> Option<f64> {
        self.look_at(t).map(|l| l.elevation)
    }

    /// The instant the horizon is crossed, to within a fraction of a second.
    /// `rising` says which side of the bracket is above the horizon; the
    /// returned bound is always the first instant on the far side.
    fn cross(&self, lo: u32, hi: u32, rising: bool) -> u32 {
        let (mut lo, mut hi) = (lo, hi);
        for _ in 0..BISECT_ROUNDS {
            let mid = lo + (hi - lo) / 2;
            if mid == lo {
                break;
            }
            match self.elevation(mid) {
                Some(e) if (e < 0.0) != rising => hi = mid,
                Some(_) => lo = mid,
                None => break,
            }
        }
        hi
    }
}

#[no_mangle]
pub extern "C" fn app_entry(api: &Api, ev: AppEvent) -> AppResult {
    unsafe {
        API_PTR = api as *const Api;
    }
    match ev {
        AppEvent::Enter => enter(api),
        // Unreachable: the loader faults an oversized segment that is still
        // resident when anything but `Enter` arrives.
        _ => AppResult::Chain(SEG_LIST),
    }
}

fn fail(api: &Api, h: &Handoff, status: u8) -> AppResult {
    Handoff {
        status,
        page: PAGE_PASSES,
        ..*h
    }
    .store(api);
    AppResult::Chain(SEG_LIST)
}

fn enter(api: &Api) -> AppResult {
    let h = Handoff::load(api).unwrap_or(Handoff::BLANK);

    let now = (api.utc_get)();
    if now == bf5rh_abi::UTC_NOT_SET {
        return fail(api, &h, ST_NO_UTC);
    }
    let Some((lat_e5, lon_e5)) = observer(api) else {
        return fail(api, &h, ST_NO_POS);
    };

    // The observer's height above the ellipsoid is not asked for: a few
    // hundred metres moves the range by less than the elements do.
    let obs = Observer::new(
        lat_e5 as f64 * RAD_PER_COORD_UNIT,
        lon_e5 as f64 * RAD_PER_COORD_UNIT,
        0.0,
    );

    let outcome = match h.job {
        JOB_CALIBRATE => {
            calibrate(api, &obs, h.sat_index, h.pass_index, h.aos, h.los)
                .map(|_| h.pass_count)
        }
        _ => search_all(api, &obs, now),
    };

    match outcome {
        Err(status) => fail(api, &h, status),
        Ok(count) => {
            Handoff {
                pass_count: count,
                status: ST_OK,
                page: PAGE_PASSES,
                ..h
            }
            .store(api);
            if h.job == JOB_CALIBRATE {
                AppResult::Chain(SEG_TRACK)
            } else {
                AppResult::Chain(SEG_LIST)
            }
        }
    }
}

fn decode_record(api: &Api, sat: u8) -> Option<Record> {
    let raw = unsafe { &mut *core::ptr::addr_of_mut!(SCRATCH) };
    (api.nor_read)(
        record_addr(sat as usize),
        raw.as_mut_ptr(),
        RECORD_SIZE as u32,
    );
    Record::decode(raw)
}

fn search_all(api: &Api, obs: &Observer, now: u32) -> Result<u8, u8> {
    (api.nor_erase_sector)(RESULT_ADDR);

    let mut total = 0u8;
    for sat in 0..MAX_SATELLITES as u8 {
        let decoded = decode_record(api, sat);
        let Some(rec) = decoded.as_ref() else {
            continue;
        };
        let ctx = Ctx { obs: *obs, rec };
        let Ok(n) = scan(api, &ctx, sat, now, total) else {
            continue;
        };
        total = total.saturating_add(n);
        if total as usize >= MAX_TOTAL_PASSES {
            break;
        }
    }

    if total == 0 {
        return Err(ST_NO_PASS);
    }

    let mut head = [0u8; PASS_HEADER_SIZE];
    put_le32(&mut head, 0, PASS_MAGIC);
    head[4] = total;
    (api.nor_write)(RESULT_ADDR, head.as_ptr(), head.len() as u32);
    Ok(total)
}

fn calibrate(
    api: &Api,
    obs: &Observer,
    sat_index: u8,
    pass_index: u8,
    aos: u32,
    los: u32,
) -> Result<(), u8> {
    let decoded = decode_record(api, sat_index);
    let Some(rec) = decoded.as_ref() else {
        return Err(ST_BAD_RECORD);
    };
    (api.nor_erase_sector)(CALIB_ADDR);
    let ctx = Ctx { obs: *obs, rec };
    sample(api, &ctx, aos, los, Some((sat_index, pass_index)))?;
    Ok(())
}

fn scan(
    api: &Api,
    ctx: &Ctx,
    sat_index: u8,
    now: u32,
    base: u8,
) -> Result<u8, u8> {
    let mut count = 0u8;
    let mut prev = ctx.elevation(now).ok_or(ST_DECAYED)?;
    let mut aos = (prev > 0.0).then_some(now);
    let end = now + HORIZON_S;
    let mut t = now;

    while t + COARSE_STEP <= end && count < MAX_PASSES as u8 {
        let t2 = t + COARSE_STEP;
        let e2 = ctx.elevation(t2).ok_or(ST_DECAYED)?;
        if prev < 0.0 && !(e2 < 0.0) {
            aos = Some(ctx.cross(t, t2, true));
        } else if !(prev < 0.0) && e2 < 0.0 {
            if let Some(a) = aos.take() {
                let los = ctx.cross(t, t2, false);
                let max_el = sample(api, ctx, a, los, None)?;
                if max_el >= MIN_MAX_EL_CDEG {
                    write_pass_entry(
                        api,
                        base + count,
                        sat_index,
                        count,
                        a,
                        los,
                        max_el,
                    );
                    count += 1;
                }
            }
        }
        t = t2;
        prev = e2;
    }
    Ok(count)
}

fn write_pass_entry(
    api: &Api,
    slot: u8,
    sat_index: u8,
    pass_index: u8,
    aos: u32,
    los: u32,
    max_el: i16,
) {
    let mut b = [0u8; PASS_ENTRY_SIZE];
    b[0] = sat_index;
    b[1] = pass_index;
    put_le32(&mut b, 2, aos);
    put_le32(&mut b, 6, los);
    put_le16(&mut b, 10, max_el as u16);
    (api.nor_write)(
        RESULT_ADDR
            + (PASS_HEADER_SIZE + slot as usize * PASS_ENTRY_SIZE) as u32,
        b.as_ptr(),
        b.len() as u32,
    );
}

fn sample(
    api: &Api,
    ctx: &Ctx,
    aos: u32,
    los: u32,
    write: Option<(u8, u8)>,
) -> Result<i16, u8> {
    let t0 = aos.saturating_sub(CALIB_STEP_S);
    let span = los.saturating_add(CALIB_STEP_S).saturating_sub(t0);
    let count =
        (span / CALIB_STEP_S + 1).clamp(2, MAX_CALIB_SAMPLES as u32) as u16;

    let buf = unsafe { &mut *core::ptr::addr_of_mut!(SCRATCH) };
    let mut held = 0usize;
    let mut written = 0u32;
    let mut max_el_cdeg = i16::MIN;

    for i in 0..count as usize {
        let l = ctx
            .look_at(t0 + i as u32 * CALIB_STEP_S)
            .ok_or(ST_DECAYED)?;
        let el = to_cdeg(l.elevation).clamp(-9000, 9000) as i16;
        let az = to_cdeg(l.azimuth).clamp(0, 35_999) as u16;
        if el > max_el_cdeg {
            max_el_cdeg = el;
        }

        if write.is_some() {
            let rate = sgp4mini::math::round_to_i32(
                l.range_rate_km_s * (1000.0 * RATE_UNITS_PER_M_S as f64),
            );
            encode_sample(
                &Sample {
                    rate_q: rate.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
                    az_cdeg: az,
                    el_cdeg: el,
                },
                &mut buf[held * CALIB_SAMPLE_SIZE..],
            );
            held += 1;
            if held == FLUSH_SAMPLES {
                written = flush(api, &buf[..held * CALIB_SAMPLE_SIZE], written);
                held = 0;
            }
        }
    }

    if let Some((sat_index, pass_index)) = write {
        if held > 0 {
            flush(api, &buf[..held * CALIB_SAMPLE_SIZE], written);
        }
        let mut head = [0u8; CALIB_HEADER_SIZE];
        CalibHeader {
            sat_index,
            pass_index,
            count,
            t0,
            aos,
            los,
            step_s: CALIB_STEP_S as u16,
        }
        .encode(&mut head);
        (api.nor_write)(CALIB_ADDR, head.as_ptr(), head.len() as u32);
    }

    Ok(max_el_cdeg)
}

fn flush(api: &Api, chunk: &[u8], at: u32) -> u32 {
    (api.nor_write)(
        CALIB_ADDR + CALIB_HEADER_SIZE as u32 + at,
        chunk.as_ptr(),
        chunk.len() as u32,
    );
    at + chunk.len() as u32
}

fn to_cdeg(rad: f64) -> i32 {
    sgp4mini::math::round_to_i32(rad * DEG_PER_RAD_E2)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let ptr = API_PTR;
        if !ptr.is_null() {
            ((*ptr).app_fault)(0);
        }
    }
    loop {}
}
