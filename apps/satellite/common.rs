#![allow(dead_code)]

use bf5rh_abi::Api;

pub const KIND_SINGLE: u8 = 2;
pub const KIND_LONG: u8 = 3;
pub const KIND_REPEAT: u8 = 4;
pub const KEY_DIGIT0: u8 = 0;
pub const KEY_ASTERISK: u8 = 10;
pub const KEY_POUND: u8 = 11;
pub const KEY_MENU: u8 = 12;
pub const KEY_EXIT: u8 = 13;
pub const KEY_UP: u8 = 14;
pub const KEY_DOWN: u8 = 15;
pub const KEY_BAND: u8 = 18;

pub const BLACK: u16 = 0x0000;
pub const WHITE: u16 = 0xFFFF;
pub const F5: u8 = 0;
pub const F6: u8 = 1;
pub const F9: u8 = 2;
pub const SCREEN_W: i16 = 160;
pub const SCREEN_H: i16 = 128;

pub const VISIBLE_ROWS: usize = 8;

pub const SEG_LIST: u8 = 0;
pub const SEG_PREDICT: u8 = 1;
pub const SEG_TRACK: u8 = 2;

// flash layout

/// Satellite records, written by the host.
/// 5120 B, so this spans **two** 4 KB sectors. the next free sector is
/// 0x16E000, not 0x16D000.
pub const SAT_TABLE_ADDR: u32 = 0x16C000;
pub const RECORD_SIZE: usize = 256;
pub const MAX_SATELLITES: usize = 20;

pub const RECORD_VERSION: u8 = 1;

/// Persistent app state: the UTC date
pub const STATE_ADDR: u32 = 0x16E000;
/// Prediction output: the combined pass table, one entry per kept pass across
/// every satellite.
pub const RESULT_ADDR: u32 = 0x16F000;
pub const CALIB_ADDR: u32 = 0x170000;

pub const SECTOR_SIZE: u32 = 4096;

// little-endian packing

pub fn put_le16(b: &mut [u8], at: usize, v: u16) {
    b[at] = v as u8;
    b[at + 1] = (v >> 8) as u8;
}

pub fn put_le32(b: &mut [u8], at: usize, v: u32) {
    b[at] = v as u8;
    b[at + 1] = (v >> 8) as u8;
    b[at + 2] = (v >> 16) as u8;
    b[at + 3] = (v >> 24) as u8;
}

pub fn le16(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

pub fn le32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

// satellite record, header only

#[derive(Clone, Copy)]
pub struct RecordHead {
    pub name: [u8; 10],
    pub rx_freq_hz: u32,
    /// 0 = receive only
    pub tx_freq_hz: u32,
    pub rx_tone: u16,
    pub tx_tone: u16,
}

impl RecordHead {
    pub const BLANK: RecordHead = RecordHead {
        name: [0; 10],
        rx_freq_hz: 0,
        tx_freq_hz: 0,
        rx_tone: 0,
        tx_tone: 0,
    };

    /// `None` for an erased slot or a record this firmware cannot read.
    pub fn decode(buf: &[u8]) -> Option<RecordHead> {
        if buf.len() < 24 || buf[10] != RECORD_VERSION {
            return None;
        }
        let mut name = [0u8; 10];
        for (slot, &b) in name.iter_mut().zip(buf) {
            *slot = b;
        }
        Some(RecordHead {
            name,
            rx_freq_hz: le32(buf, 0x0C),
            tx_freq_hz: le32(buf, 0x10),
            rx_tone: le16(buf, 0x14),
            tx_tone: le16(buf, 0x16),
        })
    }

    pub fn name_len(&self) -> usize {
        self.name
            .iter()
            .position(|&b| b == 0 || b == 0xFF)
            .unwrap_or(self.name.len())
    }
}

pub fn record_addr(idx: usize) -> u32 {
    SAT_TABLE_ADDR + (idx * RECORD_SIZE) as u32
}

pub fn read_head(api: &Api, idx: usize) -> Option<RecordHead> {
    let mut raw = [0u8; 24];
    (api.nor_read)(record_addr(idx), raw.as_mut_ptr(), raw.len() as u32);
    RecordHead::decode(&raw)
}

/// Bit `i` set = slot `i` holds a readable record.
pub fn scan_occupancy(api: &Api) -> u32 {
    let mut mask = 0u32;
    for i in 0..MAX_SATELLITES {
        if read_head(api, i).is_some() {
            mask |= 1 << i;
        }
    }
    mask
}

// pass table

pub const PASS_MAGIC: u32 = u32::from_le_bytes(*b"PASS");
/// Kept passes per satellite.
pub const MAX_PASSES: usize = 4;
/// Total entries the table can hold, over every satellite.
pub const MAX_TOTAL_PASSES: usize = MAX_SATELLITES * MAX_PASSES;
pub const PASS_HEADER_SIZE: usize = 8;
pub const PASS_ENTRY_SIZE: usize = 12;

#[derive(Clone, Copy)]
pub struct PassEntry {
    pub sat_index: u8,
    pub pass_index: u8,
    pub aos: u32,
    pub los: u32,
    /// hundredths of a degree
    pub max_el_cdeg: i16,
}

impl PassEntry {
    pub const BLANK: PassEntry = PassEntry {
        sat_index: 0xFF,
        pass_index: 0,
        aos: 0,
        los: 0,
        max_el_cdeg: 0,
    };
}

pub fn encode_pass_entry(b: &mut [u8], at: usize, e: &PassEntry) {
    b[at] = e.sat_index;
    b[at + 1] = e.pass_index;
    put_le32(b, at + 2, e.aos);
    put_le32(b, at + 6, e.los);
    put_le16(b, at + 10, e.max_el_cdeg as u16);
}

pub fn decode_pass_entry(b: &[u8]) -> PassEntry {
    PassEntry {
        sat_index: b[0],
        pass_index: b[1],
        aos: le32(b, 2),
        los: le32(b, 6),
        max_el_cdeg: le16(b, 10) as i16,
    }
}

/// Header only: magic + count. `None` when nothing has been written yet (an
/// erased sector) or the count is out of range.
pub fn read_pass_count(api: &Api) -> Option<u8> {
    let mut raw = [0u8; PASS_HEADER_SIZE];
    (api.nor_read)(RESULT_ADDR, raw.as_mut_ptr(), raw.len() as u32);
    if le32(&raw, 0) != PASS_MAGIC {
        return None;
    }
    let count = raw[4];
    if count as usize > MAX_TOTAL_PASSES {
        return None;
    }
    Some(count)
}

/// Reads one entry into `out`; returns `false` when the flash read failed.
pub fn read_pass_entry(
    api: &Api,
    slot: usize,
    out: &mut [u8; PASS_ENTRY_SIZE],
) -> bool {
    (api.nor_read)(
        RESULT_ADDR + (PASS_HEADER_SIZE + slot * PASS_ENTRY_SIZE) as u32,
        out.as_mut_ptr(),
        PASS_ENTRY_SIZE as u32,
    )
}

// calibration table

pub const CALIB_MAGIC: u32 = u32::from_le_bytes(*b"CALB");
pub const CALIB_HEADER_SIZE: usize = 24;
pub const CALIB_SAMPLE_SIZE: usize = 6;
pub const CALIB_STEP_S: u32 = 10;
pub const MAX_CALIB_SAMPLES: usize = 128;
pub const CALIB_TABLE_BYTES: usize =
    CALIB_HEADER_SIZE + MAX_CALIB_SAMPLES * CALIB_SAMPLE_SIZE;

pub const RATE_UNITS_PER_M_S: i32 = 4;

#[derive(Clone, Copy)]
pub struct Sample {
    pub rate_q: i16,
    pub az_cdeg: u16,
    pub el_cdeg: i16,
}

#[derive(Clone, Copy)]
pub struct CalibHeader {
    pub sat_index: u8,
    pub pass_index: u8,
    pub count: u16,
    /// UTC seconds of sample 0
    pub t0: u32,
    pub aos: u32,
    pub los: u32,
    pub step_s: u16,
}

impl CalibHeader {
    pub fn encode(&self, out: &mut [u8; CALIB_HEADER_SIZE]) {
        put_le32(out, 0, CALIB_MAGIC);
        out[4] = self.sat_index;
        out[5] = self.pass_index;
        put_le16(out, 6, self.count);
        put_le32(out, 8, self.t0);
        put_le32(out, 12, self.aos);
        put_le32(out, 16, self.los);
        put_le16(out, 20, self.step_s);
        out[22] = 0;
        out[23] = 0;
    }

    pub fn decode(b: &[u8]) -> Option<CalibHeader> {
        if b.len() < CALIB_HEADER_SIZE || le32(b, 0) != CALIB_MAGIC {
            return None;
        }
        let count = le16(b, 6);
        let step_s = le16(b, 20);
        if count as usize > MAX_CALIB_SAMPLES || count < 2 || step_s == 0 {
            return None;
        }
        Some(CalibHeader {
            sat_index: b[4],
            pass_index: b[5],
            count,
            t0: le32(b, 8),
            aos: le32(b, 12),
            los: le32(b, 16),
            step_s,
        })
    }
}

pub fn encode_sample(s: &Sample, out: &mut [u8]) {
    put_le16(out, 0, s.rate_q as u16);
    put_le16(out, 2, s.az_cdeg);
    put_le16(out, 4, s.el_cdeg as u16);
}

pub fn decode_sample(b: &[u8]) -> Sample {
    Sample {
        rate_q: le16(b, 0) as i16,
        az_cdeg: le16(b, 2),
        el_cdeg: le16(b, 4) as i16,
    }
}

pub fn read_calib_header(api: &Api) -> Option<CalibHeader> {
    let mut raw = [0u8; CALIB_HEADER_SIZE];
    (api.nor_read)(CALIB_ADDR, raw.as_mut_ptr(), raw.len() as u32);
    CalibHeader::decode(&raw)
}

/// Doppler shift in Hz for a carrier at `freq_hz`, from a range rate in
/// quarter-metres per second. Negative range rate (closing) shifts up.
///
/// All integer: `freq * rate` reaches 3.8e12, so it is done in `i64`, which
/// costs a compiler intrinsic but no soft-float
pub fn doppler_hz(freq_hz: u32, rate_q: i16) -> i32 {
    const C_M_S: i64 = 299_792_458;
    let n = -(freq_hz as i64) * rate_q as i64;
    (n / (C_M_S * RATE_UNITS_PER_M_S as i64)) as i32
}

// ersistent app state

pub const STATE_MAGIC: u32 = u32::from_le_bytes(*b"SATD");
pub const STATE_BYTES: usize = 8;

pub fn load_saved_date(api: &Api) -> Option<i32> {
    let mut raw = [0u8; STATE_BYTES];
    (api.nor_read)(STATE_ADDR, raw.as_mut_ptr(), raw.len() as u32);
    if le32(&raw, 0) != STATE_MAGIC {
        return None;
    }
    let days = le16(&raw, 4);
    // stored twice, once inverted, so a half-written record is rejected
    if le16(&raw, 6) != !days {
        return None;
    }
    Some(days as i32)
}

pub fn save_date(api: &Api, days: i32) {
    if !(0..=u16::MAX as i32).contains(&days) {
        return;
    }
    let days = days as u16;
    let mut raw = [0u8; STATE_BYTES];
    put_le32(&mut raw, 0, STATE_MAGIC);
    put_le16(&mut raw, 4, days);
    put_le16(&mut raw, 6, !days);
    (api.nor_erase_sector)(STATE_ADDR);
    (api.nor_write)(STATE_ADDR, raw.as_ptr(), raw.len() as u32);
}

// observer position

pub const SETTING_LAT: u16 = 5;
pub const SETTING_LON: u16 = 6;

/// Latitude and longitude in units of 1e-5 degrees, or `None` when not set
pub fn observer(api: &Api) -> Option<(i32, i32)> {
    let lat = (api.settings_get)(SETTING_LAT) as i32;
    let lon = (api.settings_get)(SETTING_LON) as i32;
    if lat == bf5rh_abi::SETTING_COORD_NOT_SET
        || lon == bf5rh_abi::SETTING_COORD_NOT_SET
    {
        return None;
    }
    Some((lat, lon))
}

// segment handoff

pub const HANDOFF_LEN: u16 = 16;

// `job`
pub const JOB_SEARCH: u8 = 0;
pub const JOB_CALIBRATE: u8 = 1;
// `status`, predict -> list
pub const ST_OK: u8 = 0;
pub const ST_NO_UTC: u8 = 1;
pub const ST_NO_POS: u8 = 2;
pub const ST_BAD_RECORD: u8 = 3;
pub const ST_NO_PASS: u8 = 4;
pub const ST_DECAYED: u8 = 5;
// `page`, which screen the list segment should come up on
pub const PAGE_SATS: u8 = 0;
pub const PAGE_PASSES: u8 = 1;

#[derive(Clone, Copy)]
pub struct Handoff {
    pub job: u8,
    pub sat_index: u8,
    pub list_row: u8,
    pub pass_index: u8,
    pub pass_count: u8,
    pub status: u8,
    pub page: u8,
    /// Age of the elements at prediction time, saturating at 255 days. Stale
    /// elements are the quiet failure mode of any tracker, so it is shown.
    pub tle_age_days: u8,
    pub aos: u32,
    pub los: u32,
}

impl Handoff {
    pub const BLANK: Handoff = Handoff {
        job: JOB_SEARCH,
        sat_index: 0,
        list_row: 0,
        pass_index: 0,
        pass_count: 0,
        status: ST_OK,
        page: PAGE_SATS,
        tle_age_days: 0,
        aos: 0,
        los: 0,
    };

    pub fn store(&self, api: &Api) {
        let mut buf = [0u8; HANDOFF_LEN as usize];
        buf[0] = self.job;
        buf[1] = self.sat_index;
        buf[2] = self.list_row;
        buf[3] = self.pass_index;
        buf[4] = self.pass_count;
        buf[5] = self.status;
        buf[6] = self.page;
        buf[7] = self.tle_age_days;
        put_le32(&mut buf, 8, self.aos);
        put_le32(&mut buf, 12, self.los);
        (api.handoff_write)(buf.as_ptr(), HANDOFF_LEN);
    }

    /// `None` means the segment was entered fresh from the launcher rather
    /// than chained into from a sibling segment.
    pub fn load(api: &Api) -> Option<Handoff> {
        let mut b = [0u8; HANDOFF_LEN as usize];
        if (api.handoff_read)(b.as_mut_ptr(), HANDOFF_LEN) != HANDOFF_LEN {
            return None;
        }
        Some(Handoff {
            job: b[0],
            sat_index: b[1],
            list_row: b[2],
            pass_index: b[3],
            pass_count: b[4],
            status: b[5],
            page: b[6],
            tle_age_days: b[7],
            aos: le32(&b, 8),
            los: le32(&b, 12),
        })
    }
}

// text helpers

pub fn write_str(out: &mut [u8], s: &str) {
    write_bytes(out, s.as_bytes());
}

pub fn write_bytes(out: &mut [u8], s: &[u8]) {
    let n = s.len().min(out.len());
    for (slot, &b) in out.iter_mut().zip(s) {
        *slot = b;
    }
    for b in out[n..].iter_mut() {
        *b = 0;
    }
}

/// Decimal, right-aligned into `width` if `width > 0`, else natural width.
pub fn put_u32(out: &mut [u8], at: usize, v: u32, width: usize) -> usize {
    let mut digits = [0u8; 10];
    let mut n = 0;
    let mut v = v;
    loop {
        digits[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
        if v == 0 {
            break;
        }
    }
    let mut i = at;
    for _ in n..width {
        if i < out.len() {
            out[i] = b'0';
            i += 1;
        }
    }
    while n > 0 {
        n -= 1;
        if i < out.len() {
            out[i] = digits[n];
            i += 1;
        }
    }
    i
}

pub fn put_i32(out: &mut [u8], at: usize, v: i32, width: usize) -> usize {
    let mut i = at;
    if v < 0 && i < out.len() {
        out[i] = b'-';
        i += 1;
    }
    put_u32(out, i, v.unsigned_abs(), width)
}

pub fn put_char(out: &mut [u8], at: usize, c: u8) -> usize {
    if at < out.len() {
        out[at] = c;
        return at + 1;
    }
    at
}

/// `xxx.xxx` MHz
pub fn put_freq_mhz(out: &mut [u8], at: usize, hz: u32) -> usize {
    let i = put_u32(out, at, hz / 1_000_000, 0);
    let i = put_char(out, i, b'.');
    put_u32(out, i, (hz % 1_000_000) / 1000, 3)
}

/// `xxx.xxx.xxx` Hz
pub fn put_freq_hz(out: &mut [u8], at: usize, hz: u32) -> usize {
    let i = put_u32(out, at, hz / 1_000_000, 0);
    let i = put_char(out, i, b'.');
    let i = put_u32(out, i, (hz / 1000) % 1000, 3);
    let i = put_char(out, i, b'.');
    put_u32(out, i, hz % 1000, 3)
}

/// Right-aligns the `len` bytes at the front of `out` into a `width` field, so
/// the last digit stays put as the number's length changes.
pub fn right_align(out: &mut [u8], len: usize, width: usize) {
    if len >= width || width > out.len() {
        return;
    }
    let shift = width - len;
    for i in (0..len).rev() {
        out[i + shift] = out[i];
    }
    for b in out[..shift].iter_mut() {
        *b = b' ';
    }
}

/// `HH:MM:SS` from seconds past midnight
pub fn put_hms(out: &mut [u8], at: usize, secs_of_day: u32) -> usize {
    let i = put_u32(out, at, secs_of_day / 3600, 2);
    let i = put_char(out, i, b':');
    let i = put_u32(out, i, (secs_of_day / 60) % 60, 2);
    let i = put_char(out, i, b':');
    put_u32(out, i, secs_of_day % 60, 2)
}

/// `HH:MM` from seconds past midnight
pub fn put_hm(out: &mut [u8], at: usize, secs_of_day: u32) -> usize {
    let i = put_u32(out, at, secs_of_day / 3600, 2);
    let i = put_char(out, i, b':');
    put_u32(out, i, (secs_of_day / 60) % 60, 2)
}

/// `MM-DD` for a day count since 2000-01-01
pub fn put_md(out: &mut [u8], at: usize, days: i32) -> usize {
    let (_, m, d) = sgp4mini::civil_from_days(days);
    let i = put_u32(out, at, m as u32, 2);
    let i = put_char(out, i, b'-');
    put_u32(out, i, d as u32, 2)
}

/// `M:SS` or `H:MM:SS`, for a countdown
pub fn put_duration(out: &mut [u8], at: usize, secs: u32) -> usize {
    let mut i = at;
    if secs >= 3600 {
        i = put_u32(out, i, secs / 3600, 0);
        i = put_char(out, i, b':');
        i = put_u32(out, i, (secs / 60) % 60, 2);
    } else {
        i = put_u32(out, i, secs / 60, 0);
    }
    let i = put_char(out, i, b':');
    put_u32(out, i, secs % 60, 2)
}

/// latitude and a longitude fit side by side in title.
pub fn put_coord(out: &mut [u8], at: usize, v: i32, is_lat: bool) -> usize {
    let av = v.unsigned_abs();
    let i = put_u32(out, at, av / 100_000, 0);
    let i = put_char(out, i, b'.');
    let i = put_u32(out, i, (av % 100_000) / 1000, 2);
    put_char(
        out,
        i,
        match (is_lat, v < 0) {
            (true, false) => b'N',
            (true, true) => b'S',
            (false, false) => b'E',
            (false, true) => b'W',
        },
    )
}

pub fn write_at(out: &mut [u8], at: usize, s: &[u8]) -> usize {
    let n = s.len().min(out.len().saturating_sub(at));
    for (slot, &b) in out[at..at + n].iter_mut().zip(s) {
        *slot = b;
    }
    at + n
}

pub fn octant(az_cdeg: u16) -> &'static [u8] {
    const NAMES: [&[u8]; 8] =
        [b"N", b"NE", b"E", b"SE", b"S", b"SW", b"W", b"NW"];
    NAMES[(((az_cdeg as u32 + 2250) / 4500) % 8) as usize]
}

pub fn digit_value(key: u8) -> Option<u8> {
    if key <= 9 {
        Some(key)
    } else {
        None
    }
}

pub fn scroll_top(total: usize, selected: usize) -> usize {
    if total <= VISIBLE_ROWS {
        0
    } else {
        selected
            .saturating_sub(VISIBLE_ROWS / 2)
            .min(total - VISIBLE_ROWS)
    }
}

pub const SECS_PER_DAY: u32 = 86400;
