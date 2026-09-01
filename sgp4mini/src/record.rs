//! The on-flash satellite record: 256 bytes of *already-propagated-ready*
//! constants, plus the radio's frequencies and tones.
//!
//! ```text
//! 0x00  10  name, ASCII, NUL-padded
//! 0x0A   1  version
//! 0x0B   1  flags: bit0 high_altitude, bit1 elliptic
//! 0x0C   4  rx_freq_hz     u32, downlink
//! 0x10   4  tx_freq_hz     u32, uplink, 0 = receive only
//! 0x14   2  rx_tone        u16, 5RH subaudio code
//! 0x16   2  tx_tone        u16
//! 0x18   8  epoch_days_j2000  f64
//! 0x20 224  28 × f64, in the order of `FIELDS` below
//! ```

use crate::sgp4::Constants;

pub const VERSION: u8 = 1;
pub const RECORD_SIZE: usize = 256;
pub const MAX_SATELLITES: usize = 20;
pub const TABLE_BYTES: usize = RECORD_SIZE * MAX_SATELLITES;

const NAME_LEN: usize = 10;
const CONSTANTS_OFF: usize = 0x20;
const CONSTANTS_COUNT: usize = 28;

const FLAG_HIGH_ALTITUDE: u8 = 1 << 0;
const FLAG_ELLIPTIC: u8 = 1 << 1;

/// One satellite as the segments see it.
#[derive(Debug, Clone, Copy)]
pub struct Record {
    pub name: [u8; NAME_LEN],
    /// Hz, downlink
    pub rx_freq_hz: u32,
    /// Hz, uplink; 0 means receive only
    pub tx_freq_hz: u32,
    pub rx_tone: u16,
    pub tx_tone: u16,
    /// days since J2000, the reference for `propagate`'s `t`
    pub epoch_days_j2000: f64,
    pub constants: Constants,
}

impl Record {
    pub fn name_str(&self) -> &str {
        let end = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());
        core::str::from_utf8(&self.name[..end]).unwrap_or("?")
    }

    /// Minutes since the elements' epoch, the argument `propagate` wants.
    pub fn minutes_since_epoch(&self, days_j2000: f64) -> f64 {
        (days_j2000 - self.epoch_days_j2000) * 1440.0
    }

    /// `None` for an erased slot (all `0xFF`), a version we do not know, or a
    /// buffer that is too short.
    pub fn decode(buf: &[u8]) -> Option<Record> {
        if buf.len() < RECORD_SIZE || buf[0x0A] != VERSION {
            return None;
        }
        let flags = buf[0x0B];
        let mut name = [0u8; NAME_LEN];
        for (slot, &b) in name.iter_mut().zip(buf) {
            *slot = b;
        }

        let mut f = [0.0f64; CONSTANTS_COUNT];
        for (i, slot) in f.iter_mut().enumerate() {
            *slot = f64_at(buf, CONSTANTS_OFF + i * 8);
        }

        Some(Record {
            name,
            rx_freq_hz: u32_at(buf, 0x0C),
            tx_freq_hz: u32_at(buf, 0x10),
            rx_tone: u16_at(buf, 0x14),
            tx_tone: u16_at(buf, 0x16),
            epoch_days_j2000: f64_at(buf, 0x18),
            constants: Constants {
                inclination_0: f[0],
                right_ascension_0: f[1],
                eccentricity_0: f[2],
                argument_of_perigee_0: f[3],
                mean_anomaly_0: f[4],
                mean_motion_0: f[5],
                right_ascension_dot: f[6],
                argument_of_perigee_dot: f[7],
                mean_anomaly_dot: f[8],
                c1: f[9],
                c4: f[10],
                k0: f[11],
                k1: f[12],
                a0: f[13],
                k2: f[14],
                k5: f[15],
                high_altitude: flags & FLAG_HIGH_ALTITUDE != 0,
                c5: f[16],
                d2: f[17],
                d3: f[18],
                d4: f[19],
                eta: f[20],
                k7: f[21],
                k8: f[22],
                k9: f[23],
                k10: f[24],
                elliptic: flags & FLAG_ELLIPTIC != 0,
                k11: f[25],
                k12: f[26],
                k13: f[27],
            },
        })
    }

    /// Writes exactly [`RECORD_SIZE`] bytes. Used by the host packer; kept in
    /// this crate so encode and decode can never drift apart.
    pub fn encode(&self, buf: &mut [u8]) {
        let buf = &mut buf[..RECORD_SIZE];
        buf.fill(0);
        buf[..NAME_LEN].copy_from_slice(&self.name);
        buf[0x0A] = VERSION;
        let c = &self.constants;
        buf[0x0B] = if c.high_altitude {
            FLAG_HIGH_ALTITUDE
        } else {
            0
        } | if c.elliptic { FLAG_ELLIPTIC } else { 0 };
        buf[0x0C..0x10].copy_from_slice(&self.rx_freq_hz.to_le_bytes());
        buf[0x10..0x14].copy_from_slice(&self.tx_freq_hz.to_le_bytes());
        buf[0x14..0x16].copy_from_slice(&self.rx_tone.to_le_bytes());
        buf[0x16..0x18].copy_from_slice(&self.tx_tone.to_le_bytes());
        buf[0x18..0x20].copy_from_slice(&self.epoch_days_j2000.to_le_bytes());

        let f = [
            c.inclination_0,
            c.right_ascension_0,
            c.eccentricity_0,
            c.argument_of_perigee_0,
            c.mean_anomaly_0,
            c.mean_motion_0,
            c.right_ascension_dot,
            c.argument_of_perigee_dot,
            c.mean_anomaly_dot,
            c.c1,
            c.c4,
            c.k0,
            c.k1,
            c.a0,
            c.k2,
            c.k5,
            c.c5,
            c.d2,
            c.d3,
            c.d4,
            c.eta,
            c.k7,
            c.k8,
            c.k9,
            c.k10,
            c.k11,
            c.k12,
            c.k13,
        ];
        for (i, v) in f.iter().enumerate() {
            let at = CONSTANTS_OFF + i * 8;
            buf[at..at + 8].copy_from_slice(&v.to_le_bytes());
        }
    }
}

fn u16_at(buf: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([buf[at], buf[at + 1]])
}

fn u32_at(buf: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([buf[at], buf[at + 1], buf[at + 2], buf[at + 3]])
}

/// Two unaligned word loads rather than `copy_from_slice` or eight indexed
/// bytes. `copy_from_slice` is outlined at `opt-level = "z"` into `core`'s
/// generic helper, which drags the big general-purpose `memcpy` in with it;
/// the byte-at-a-time form assembles the word by hand and costs 172 B. Both
/// hosts this crate runs on -- Cortex-M4 and x86-64 -- are little-endian and
/// take unaligned word loads in stride.
fn f64_at(buf: &[u8], at: usize) -> f64 {
    let p = buf[at..at + 8].as_ptr();
    // SAFETY: the slice index above proved eight readable bytes at `p`, and
    // `read_unaligned` imposes no alignment requirement.
    let (lo, hi) = unsafe {
        (
            core::ptr::read_unaligned(p as *const u32),
            core::ptr::read_unaligned(p.add(4) as *const u32),
        )
    };
    f64::from_bits(((hi as u64) << 32) | lo as u64)
}
