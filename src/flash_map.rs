#![allow(dead_code)]
pub mod addr {

    pub const PA_TABLE_BASE_HIGH: u32 = 0xF000;
    pub const PA_TABLE_BASE_MID: u32 = 0xF040;
    pub const PA_TABLE_BASE_LOW: u32 = 0xF080;
    pub const PA_TABLE_ENTRY_LEN: usize = 16;

    pub const TH_SQL_TAB_ADDR: u32 = 0xF0C0;
    pub const TH_SQL_TAB_MUTE_ADDR: u32 = 0xF0D0;
    pub const TH_SQL_TAB_LEN: usize = 10;

    pub const OFFSET_SQL_U_400_ADDR: u32 = 0xF0E0;
    pub const OFFSET_SQL_V_136_ADDR: u32 = 0xF0F0;
    pub const OFFSET_SQL_V_200_ADDR: u32 = 0xF100;
    pub const OFFSET_SQL_U_350_ADDR: u32 = 0xF110;
    pub const OFFSET_SQL_LEN: usize = 16;

    pub const DEV_BATT_ADDR: u32 = 0xF200;
    pub const DEV_BATT_LEN: usize = 6;

    pub const MODULATION_CAL_ADDR: u32 =
        crate::drivers::norflash::CAL_BLOCK_ADDR;

    pub const BAND_ADDR: u32 = 0xF230;
    pub const BAND_LEN: usize = 21;

    pub const MODEL_ADDR: u32 = 0xF255;

    // Copied from K6's flash_map.rs, same-lineage assumption

    pub const CHAN_ADDR: u32 = 0x0000;
    pub const CHAN_SIZE: u32 = 32;
    pub const NAME_SIZE: usize = 12;

    pub const VFO_INFO_ADDR: u32 = 0x8000;
    pub const VFO_SIZE: u32 = 32;

    pub const RADIO_IMFOS_ADDR: u32 = 0x9000;

    pub const DTMFINFOR_ADDR: u32 = 0xA000;
    pub const DTMF_CODE_ADDR: u32 = 0xA020;
    pub const CONTACT_SIZE: u32 = 16;
    pub const CONTACT_COUNT: usize = 20;

    pub const FIRST_BOOT_MARKER_ADDR: u32 = 0xB000;

    pub const FM_ADDR: u32 = 0xC000;

    pub const RMW_SCRATCH_ADDR: u32 = 0xD000;

    pub const SYSTEMRAN_ADDR: u32 = 0xE000;

    /// external app overlay: 720KB total
    /// after voice prompt
    pub const OVERLAY_APP_ADDR: u32 = 0x14C000;

    /// 32KB each app package: up to `abi::MAX_SEGMENTS` (4) sub-programs
    /// of 8KB (`ARENA_SIZE`) each
    pub const OVERLAY_SLOT_SIZE: u32 = 32 * 1024;
    pub const OVERLAY_SLOT_COUNT: u8 = 6;

    pub const fn overlay_slot_addr(slot: u8) -> u32 {
        OVERLAY_APP_ADDR + slot as u32 * OVERLAY_SLOT_SIZE
    }

    /// Everything past the slots is app-private data. Apps reach it through
    /// `nor_read`/`nor_write`/`nor_erase_sector`, which refuse anything below
    /// `OVERLAY_APP_ADDR`, so firmware data can never be clobbered from an
    /// app. Firmware does not interpret this region.
    pub const OVERLAY_DATA_ADDR: u32 =
        OVERLAY_APP_ADDR + OVERLAY_SLOT_SIZE * OVERLAY_SLOT_COUNT as u32;
}

pub const FIRST_BOOT_MAGIC: [u8; 4] = *b"AURA";

/// A coordinate outside its legal range is not a coordinate. Erased flash
/// reads back as `-1`, which would otherwise look like a position 0.00001
/// south of the equator.
const fn coord_or_unset(v: i32, limit: i32) -> i32 {
    if v >= -limit && v <= limit {
        v
    } else {
        COORD_NOT_SET
    }
}

fn digits_to_deci_hz(digits: &[u8]) -> u32 {
    digits.iter().fold(0u32, |acc, &d| acc * 10 + d as u32)
}

fn deci_hz_to_digits(mut deci_hz: u32, out: &mut [u8]) {
    for slot in out.iter_mut().rev() {
        *slot = (deci_hz % 10) as u8;
        deci_hz /= 10;
    }
}

fn bcd_byte_to_decimal(b: u8) -> u32 {
    ((b >> 4) * 10 + (b & 0x0f)) as u32
}

fn decimal_to_bcd_byte(v: u32) -> u8 {
    (((v / 10) << 4) | (v % 10)) as u8
}

fn bcd4_to_deci_hz(bcd: [u8; 4]) -> u32 {
    bcd_byte_to_decimal(bcd[3]) * 1_000_000
        + bcd_byte_to_decimal(bcd[2]) * 10_000
        + bcd_byte_to_decimal(bcd[1]) * 100
        + bcd_byte_to_decimal(bcd[0])
}

fn deci_hz_to_bcd4(deci_hz: u32) -> [u8; 4] {
    [
        decimal_to_bcd_byte(deci_hz % 100),
        decimal_to_bcd_byte((deci_hz / 100) % 100),
        decimal_to_bcd_byte((deci_hz / 10_000) % 100),
        decimal_to_bcd_byte(deci_hz / 1_000_000),
    ]
}

pub const FM_CHANNEL_COUNT: usize = 30;
pub const FM_CHANNEL_EMPTY: u16 = 0xFFFF;

pub enum SubaudioCode {
    None,
    DcsNormal(u16),
    DcsInverted(u16),
    Ctcss(u16),
}

impl SubaudioCode {
    pub fn decode(raw: u16) -> SubaudioCode {
        if raw > 250 {
            SubaudioCode::Ctcss(raw)
        } else if raw >= 1 {
            if raw <= 105 {
                SubaudioCode::DcsNormal(raw)
            } else {
                SubaudioCode::DcsInverted(raw)
            }
        } else {
            SubaudioCode::None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Channel {
    rx_freq_bcd: [u8; 4],
    tx_freq_bcd: [u8; 4],
    pub rx_dcs_cts_num: u16,
    pub tx_dcs_cts_num: u16,
    pub dtmf_group: u8,
    pub ptt_id: u8,
    pub tx_power: u8,
    flags: u8,
    pub decoder_code: u32,
    pub name: [u8; addr::NAME_SIZE],
}

impl Channel {
    pub fn from_bytes(buf: &[u8; addr::CHAN_SIZE as usize]) -> Channel {
        Channel {
            rx_freq_bcd: [buf[0], buf[1], buf[2], buf[3]],
            tx_freq_bcd: [buf[4], buf[5], buf[6], buf[7]],
            rx_dcs_cts_num: u16::from_le_bytes([buf[8], buf[9]]),
            tx_dcs_cts_num: u16::from_le_bytes([buf[10], buf[11]]),
            dtmf_group: buf[12],
            ptt_id: buf[13],
            tx_power: buf[14],
            flags: buf[15],
            decoder_code: u32::from_le_bytes([
                buf[16], buf[17], buf[18], buf[19],
            ]),
            name: buf[20..32].try_into().unwrap(),
        }
    }

    pub fn to_bytes(self) -> [u8; addr::CHAN_SIZE as usize] {
        let mut buf = [0u8; addr::CHAN_SIZE as usize];
        buf[0..4].copy_from_slice(&self.rx_freq_bcd);
        buf[4..8].copy_from_slice(&self.tx_freq_bcd);
        buf[8..10].copy_from_slice(&self.rx_dcs_cts_num.to_le_bytes());
        buf[10..12].copy_from_slice(&self.tx_dcs_cts_num.to_le_bytes());
        buf[12] = self.dtmf_group;
        buf[13] = self.ptt_id;
        buf[14] = self.tx_power;
        buf[15] = self.flags;
        buf[16..20].copy_from_slice(&self.decoder_code.to_le_bytes());
        buf[20..32].copy_from_slice(&self.name);
        buf
    }

    pub fn rx_freq_deci_hz(&self) -> u32 {
        bcd4_to_deci_hz(self.rx_freq_bcd)
    }

    pub fn set_rx_freq_deci_hz(&mut self, deci_hz: u32) {
        self.rx_freq_bcd = deci_hz_to_bcd4(deci_hz);
    }

    pub fn tx_freq_deci_hz(&self) -> u32 {
        bcd4_to_deci_hz(self.tx_freq_bcd)
    }

    pub fn set_tx_freq_deci_hz(&mut self, deci_hz: u32) {
        self.tx_freq_bcd = deci_hz_to_bcd4(deci_hz);
    }

    pub fn wide_narrow(&self) -> bool {
        self.flags & 0x40 != 0
    }

    pub fn set_wide_narrow(&mut self, narrow: bool) {
        if narrow {
            self.flags |= 0x40;
        } else {
            self.flags &= !0x40;
        }
    }

    pub fn busy_lock(&self) -> bool {
        self.flags & 0x08 != 0
    }

    pub fn set_busy_lock(&mut self, on: bool) {
        if on {
            self.flags |= 0x08;
        } else {
            self.flags &= !0x08;
        }
    }

    pub fn scan_add(&self) -> bool {
        self.flags & 0x04 != 0
    }

    pub fn set_scan_add(&mut self, on: bool) {
        if on {
            self.flags |= 0x04;
        } else {
            self.flags &= !0x04;
        }
    }

    pub fn ani_target(&self) -> Option<u8> {
        let idx = self.dtmf_group & 0x1f;
        if (idx as usize) < addr::CONTACT_COUNT {
            Some(idx)
        } else {
            None
        }
    }

    pub fn set_ani_target(&mut self, target: Option<u8>) {
        let idx = target.map_or(0x1f, |t| t.min(0x1f));
        self.dtmf_group = idx;
    }
}

#[derive(Clone, Copy)]
pub struct Contact {
    id_raw: [u8; 5],
    pub name: [u8; 11],
}

impl Contact {
    pub const BLANK: Contact = Contact {
        id_raw: [0xFF; 5],
        name: [0xFF; 11],
    };

    pub fn from_bytes(buf: &[u8; addr::CONTACT_SIZE as usize]) -> Contact {
        Contact {
            id_raw: buf[0..5].try_into().unwrap(),
            name: buf[5..16].try_into().unwrap(),
        }
    }

    pub fn to_bytes(self) -> [u8; addr::CONTACT_SIZE as usize] {
        let mut buf = [0u8; addr::CONTACT_SIZE as usize];
        buf[0..5].copy_from_slice(&self.id_raw);
        buf[5..16].copy_from_slice(&self.name);
        buf
    }

    pub fn id(&self) -> [u8; 3] {
        [self.id_raw[0], self.id_raw[1], self.id_raw[2]]
    }

    pub fn set_id(&mut self, id: [u8; 3]) {
        self.id_raw[0] = id[0];
        self.id_raw[1] = id[1];
        self.id_raw[2] = id[2];
    }

    pub fn is_empty(&self) -> bool {
        self.id_raw.iter().all(|&b| b == 0xFF)
    }
}

#[derive(Clone, Copy)]
pub struct VfoMode {
    freq_digits: [u8; 8],
    pub rx_dcs_cts_num: u16,
    pub tx_dcs_cts_num: u16,
    pub dtmf_group: u8,
    pub ani: u8,
    pub tx_power: u8,
    vfo_flags: u8,
    pub step: u8,
    offset_digits: [u8; 7],
    pub sp_mute: u8,
    pub decoder_code: u32,
}

impl VfoMode {
    pub fn from_bytes(buf: &[u8; addr::VFO_SIZE as usize]) -> VfoMode {
        VfoMode {
            freq_digits: buf[0..8].try_into().unwrap(),
            rx_dcs_cts_num: u16::from_le_bytes([buf[8], buf[9]]),
            tx_dcs_cts_num: u16::from_le_bytes([buf[10], buf[11]]),
            dtmf_group: buf[14],
            ani: buf[15],
            tx_power: buf[16],
            vfo_flags: buf[17],
            step: buf[19],
            offset_digits: buf[20..27].try_into().unwrap(),
            sp_mute: buf[27],
            decoder_code: u32::from_le_bytes([
                buf[28], buf[29], buf[30], buf[31],
            ]),
        }
    }

    pub fn to_bytes(self) -> [u8; addr::VFO_SIZE as usize] {
        let mut buf = [0u8; addr::VFO_SIZE as usize];
        buf[0..8].copy_from_slice(&self.freq_digits);
        buf[8..10].copy_from_slice(&self.rx_dcs_cts_num.to_le_bytes());
        buf[10..12].copy_from_slice(&self.tx_dcs_cts_num.to_le_bytes());
        buf[14] = self.dtmf_group;
        buf[15] = self.ani;
        buf[16] = self.tx_power;
        buf[17] = self.vfo_flags;
        buf[19] = self.step;
        buf[20..27].copy_from_slice(&self.offset_digits);
        buf[27] = self.sp_mute;
        buf[28..32].copy_from_slice(&self.decoder_code.to_le_bytes());
        buf
    }

    pub fn freq_deci_hz(&self) -> u32 {
        digits_to_deci_hz(&self.freq_digits)
    }

    pub fn set_freq_deci_hz(&mut self, deci_hz: u32) {
        deci_hz_to_digits(deci_hz, &mut self.freq_digits);
    }

    pub fn offset_deci_hz(&self) -> u32 {
        digits_to_deci_hz(&self.offset_digits) * 10
    }

    pub fn set_offset_deci_hz(&mut self, deci_hz: u32) {
        deci_hz_to_digits(deci_hz / 10, &mut self.offset_digits);
    }

    pub fn freq_dir(&self) -> u8 {
        self.dtmf_group >> 5
    }

    pub fn set_freq_dir(&mut self, dir: u8) {
        self.dtmf_group = (dir << 5) | (self.dtmf_group & 0x1f);
    }

    pub fn ani_target(&self) -> Option<u8> {
        let idx = self.dtmf_group & 0x1f;
        if (idx as usize) < addr::CONTACT_COUNT {
            Some(idx)
        } else {
            None
        }
    }

    pub fn set_ani_target(&mut self, target: Option<u8>) {
        let idx = target.map_or(0x1f, |t| t.min(0x1f));
        self.dtmf_group = (self.dtmf_group & 0xe0) | idx;
    }

    pub fn wide_narrow(&self) -> bool {
        self.vfo_flags & 0x40 != 0
    }

    pub fn set_wide_narrow(&mut self, narrow: bool) {
        if narrow {
            self.vfo_flags |= 0x40;
        } else {
            self.vfo_flags &= !0x40;
        }
    }
}

#[derive(Clone, Copy)]
pub struct Settings {
    /// `sqlLevel`, 0-9.
    pub sql_level: u8,
    /// `tailSwitch`: squelch-tail elimination on TX release.
    pub tail_elimination: bool,
    /// `txBusyLock`: block PTT while the channel is busy.
    pub busy_lock: bool,
    /// `txForbid`: global TX inhibit.
    pub tx_forbid: bool,
    /// `keyAutoLock`: idle time before the keypad locks itself, 0 (off) - 3;
    /// lock delay is `value * 5` seconds.
    pub key_auto_lock: u8,
    /// `dualRxFlag` != 0: dual-standby enabled.
    pub dual_standby: bool,
    /// `voxSwitch`.
    pub vox_switch: bool,
    /// `voxLevel`, 1-9 (meaningless while `vox_switch` is off).
    pub vox_level: u8,
    /// `totLevel`, steps of 15s; 0 = disabled.
    pub tot_level: u8,
    /// `beepsSwitch`: audible tone on keypress.
    pub beeps_switch: bool,
    /// `txOffTone`: 0 = off, 1 = roger beep, 2 = MDC1200 burst.
    pub roger_tone: u8,
    /// `scarmble`: voice-inversion scramble group, 0 (off) - 3.
    pub scramble_level: u8,
    /// Repeater-access tone selection, 0-3: 1000/1450/1750/2100 Hz.
    pub rtone: u8,
    /// Channel/frequency scan resume behavior, 0-2 (Time/Carrier/Search).
    pub scan_mode: u8,
    /// Receiver incremental tuning (clarifier), USB only: units of 10Hz.
    pub rit_offset: i8,
    /// RX duty-cycle power-save level, 0 (off) - 4.
    pub save_level: u8,
    /// Automatic backlight-off delay, 0-4 (0 = always on).
    pub backlight_time: u8,
    /// Standby-screen channel readout, 0-2.
    pub channel_display_mode: u8,
    /// Auto-send our own ANI frame on PTT press.
    pub ani_tx: bool,
    /// `rptrl`: 0-10, steps of 100ms.
    pub rptrl: u8,
    /// `voxDelay`: 0-15, steps of 0.1s over 0.5-2.0s.
    pub vox_delay: u8,
    /// Raw ADC calibration point for battery-voltage readout.
    pub battery_cal_raw: u16,
    /// `app::keyfn::KeyFunction` index (0..=10) for the physical Side1
    /// (FM button, PB6), Side2 (flashlight button, PA15) and Band (SCAN
    /// key) buttons' short/long press. Added once `app/` existed to point
    /// at -- see this struct's own doc comment.
    pub side1_short: u8,
    pub side1_long: u8,
    pub side2_short: u8,
    pub side2_long: u8,
    pub band_short: u8,
    pub band_long: u8,
    /// `device::radio::BandLock` index.
    pub band_lock: u8,
    /// 0=None, 1=Volt, 2=Msg, 3=Logo.
    pub boot_display_mode: u8,
    /// Forced off whenever `boot_display_mode` is 0.
    pub boot_sound_enabled: bool,
    /// Observer position, units of 1e-5 degrees (about 1.1 m), north and east
    /// positive. `COORD_NOT_SET` until the user enters one
    pub obs_lat: i32,
    pub obs_lon: i32,
}

/// `obs_lat`/`obs_lon` value meaning "the user has not set a position".
/// Out of range for both, so it can never collide with a real coordinate.
pub const COORD_NOT_SET: i32 = i32::MAX;

/// Size of the serialised Settings record in bytes.
pub const SETTINGS_BYTES: usize = 40;

impl Settings {
    pub const DEFAULT: Settings = Settings {
        sql_level: 3,
        tail_elimination: true,
        busy_lock: false,
        tx_forbid: false,
        key_auto_lock: 0,
        dual_standby: true,
        vox_switch: false,
        vox_level: 1,
        tot_level: 8,
        beeps_switch: true,
        roger_tone: 0,
        scramble_level: 0,
        rtone: 2,
        scan_mode: 1,
        rit_offset: 0,
        save_level: 0,
        backlight_time: 2,
        channel_display_mode: 2,
        ani_tx: false,
        rptrl: 0,
        vox_delay: 5, // 1.0s
        battery_cal_raw: 2537,
        side1_short: 5, // FM radio (physical "FM" button)
        side1_long: 0,  // None
        side2_short: 8, // Flashlight (physical flashlight button)
        side2_long: 0,  // None
        band_short: 6,  // Scan (physical "SCAN" button)
        band_long: 0,   // None
        band_lock: 0,
        boot_display_mode: 2,
        boot_sound_enabled: false,
        obs_lat: COORD_NOT_SET,
        obs_lon: COORD_NOT_SET,
    };

    pub fn from_bytes(buf: &[u8; SETTINGS_BYTES]) -> Settings {
        Settings {
            sql_level: buf[0],
            tail_elimination: buf[1] != 0,
            busy_lock: buf[2] != 0,
            tx_forbid: buf[3] != 0,
            key_auto_lock: buf[4].min(3),
            dual_standby: buf[5] != 0,
            vox_switch: buf[6] != 0,
            vox_level: buf[7].clamp(1, 9),
            tot_level: buf[8],
            beeps_switch: buf[9] != 0,
            roger_tone: buf[10].min(2),
            scramble_level: buf[11],
            rtone: buf[12].min(3),
            scan_mode: buf[13].min(2),
            rit_offset: (buf[14] as i8).clamp(-127, 127),
            save_level: buf[15].min(4),
            backlight_time: buf[16].min(4),
            channel_display_mode: buf[17].min(2),
            ani_tx: buf[18] != 0,
            rptrl: buf[19].min(10),
            vox_delay: buf[20].min(15),
            battery_cal_raw: u16::from_le_bytes([buf[21], buf[22]]),
            side1_short: buf[23].min(10),
            side1_long: buf[24].min(10),
            side2_short: buf[25].min(10),
            side2_long: buf[26].min(10),
            band_short: buf[27].min(10),
            band_long: buf[28].min(10),
            band_lock: buf[29].min(9),
            boot_display_mode: buf[30].min(3),
            boot_sound_enabled: buf[31] != 0,
            obs_lat: coord_or_unset(
                i32::from_le_bytes([buf[32], buf[33], buf[34], buf[35]]),
                90_00000,
            ),
            obs_lon: coord_or_unset(
                i32::from_le_bytes([buf[36], buf[37], buf[38], buf[39]]),
                180_00000,
            ),
        }
    }

    pub fn to_bytes(self) -> [u8; SETTINGS_BYTES] {
        let mut buf = [0u8; SETTINGS_BYTES];
        buf[0] = self.sql_level;
        buf[1] = self.tail_elimination as u8;
        buf[2] = self.busy_lock as u8;
        buf[3] = self.tx_forbid as u8;
        buf[4] = self.key_auto_lock;
        buf[5] = self.dual_standby as u8;
        buf[6] = self.vox_switch as u8;
        buf[7] = self.vox_level;
        buf[8] = self.tot_level;
        buf[9] = self.beeps_switch as u8;
        buf[10] = self.roger_tone;
        buf[11] = self.scramble_level;
        buf[12] = self.rtone;
        buf[13] = self.scan_mode;
        buf[14] = self.rit_offset as u8;
        buf[15] = self.save_level;
        buf[16] = self.backlight_time;
        buf[17] = self.channel_display_mode;
        buf[18] = self.ani_tx as u8;
        buf[19] = self.rptrl;
        buf[20] = self.vox_delay;
        buf[21..23].copy_from_slice(&self.battery_cal_raw.to_le_bytes());
        buf[23] = self.side1_short;
        buf[24] = self.side1_long;
        buf[25] = self.side2_short;
        buf[26] = self.side2_long;
        buf[27] = self.band_short;
        buf[28] = self.band_long;
        buf[29] = self.band_lock;
        buf[30] = self.boot_display_mode;
        buf[31] = self.boot_sound_enabled as u8;
        buf[32..36].copy_from_slice(&self.obs_lat.to_le_bytes());
        buf[36..40].copy_from_slice(&self.obs_lon.to_le_bytes());
        buf
    }

    pub fn tot_seconds(&self) -> u16 {
        self.tot_level as u16 * 15
    }
}
