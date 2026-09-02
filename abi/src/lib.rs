#![no_std]

include!(concat!(env!("OUT_DIR"), "/build_hash.rs"));

pub const MAGIC: [u8; 4] = *b"OVL1";

pub const ARENA_SIZE: usize = 8192;

/// Hard ceiling for an *oversized* segment.
pub const ARENA_MAX: usize = 12288;

/// firmware-owned scratch that outlives a segment switch
pub const HANDOFF_SIZE: usize = 64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ListRow {
    pub label: [u8; 16],
    pub value: [u8; 18],
    pub has_value: bool,
    pub cursor: i16,
}

#[repr(C)]
pub struct Api {
    // time
    pub uptime_100us: extern "C" fn() -> u32,
    pub delay_ms: extern "C" fn(ms: u32),

    // ui
    pub fill_rect: extern "C" fn(x: i16, y: i16, w: u16, h: u16, rgb565: u16),
    pub blit: extern "C" fn(x: i16, y: i16, w: u16, h: u16, px: *const u16),
    /// `font`: 0 = 5x8, 1 = 6x10, 2 = 9x18
    /// return next x position
    pub draw_text: extern "C" fn(
        x: i16,
        y: i16,
        text: *const u8,
        len: u16,
        fg: u16,
        bg: u16,
        font: u8,
    ) -> u16,
    pub draw_line:
        extern "C" fn(x0: i16, y0: i16, x1: i16, y1: i16, rgb565: u16),
    /// `filled = false`: stroke 1px, `true`: fill
    pub draw_circle:
        extern "C" fn(x: i16, y: i16, r: u16, rgb565: u16, filled: bool),

    // RF
    pub set_rx_freq: extern "C" fn(hz: u32),
    pub set_tx_freq: extern "C" fn(hz: u32),
    pub set_ptt: extern "C" fn(on: bool),
    pub read_rssi: extern "C" fn() -> u16,
    /// 0=FM 1=AM 2=USB
    pub set_modulation: extern "C" fn(mode: u8),
    /// 0=High 1=Mid 2=Low
    pub set_power: extern "C" fn(level: u8),
    /// `*_dcs_cts_num`：0=off
    pub set_subaudio_tx: extern "C" fn(code: u16),
    pub set_subaudio_rx: extern "C" fn(code: u16),

    // audio
    pub tone_on: extern "C" fn(hz_div10: u16),
    pub tone_off: extern "C" fn(),
    pub set_speaker: extern "C" fn(on: bool),
    pub read_mic_level: extern "C" fn() -> u8,

    // spi flash
    pub nor_read: extern "C" fn(addr: u32, buf: *mut u8, len: u32) -> bool,
    pub nor_write: extern "C" fn(addr: u32, buf: *const u8, len: u32) -> bool,
    pub nor_erase_sector: extern "C" fn(addr: u32) -> bool,

    /// format decimal to out
    pub fmt_u32: extern "C" fn(v: u32, out: *mut u8) -> u16,
    /// format `xxx.xxxx` format
    pub fmt_freq: extern "C" fn(hz: u32, out: *mut u8) -> u16,

    // FM radio
    pub fm_tune_khz: extern "C" fn(freq_khz: u32),
    pub fm_seek: extern "C" fn(up: bool),
    /// bit0=seek done, bit1=seek failed, bit2=is_station
    /// bit8..16=RSSI(0-255)
    pub fm_status: extern "C" fn() -> u32,
    pub fm_tuned_freq_khz: extern "C" fn() -> u32,

    /// power off FM radio and restore from PARK
    pub fm_power_off: extern "C" fn(),

    pub fm_channels_load: extern "C" fn(buf: *mut u8, len: u16) -> bool,
    pub fm_channels_save: extern "C" fn(buf: *const u8, len: u16) -> bool,

    /// read-only access
    /// cooy 32B raw channel (`flash_map::Channel::to_bytes`) to out
    /// return `false` when empty or out-of-bounds
    pub chan_read: extern "C" fn(num: u16, out: *mut u8, out_len: u16) -> bool,
    /// `0` sql, `1` tot, `2` vox on, `3` vox level, `4` beep,
    /// `5` observer latitude, `6` observer longitude, `7` chip id. The two
    /// coordinates are `i32` bit patterns in units of 1e-5 degrees
    /// (north/east positive), and `SETTING_COORD_NOT_SET` when the user has
    /// not entered a position.
    pub settings_get: extern "C" fn(id: u16) -> u32,

    /// fault
    pub app_fault: extern "C" fn(line: u32) -> !,

    pub chan_write: extern "C" fn(num: u16, buf: *const u8, len: u16) -> bool,
    pub get_master_freq: extern "C" fn() -> u32,

    pub draw_list: extern "C" fn(
        title: *const u8,
        title_len: u16,
        rows: *const ListRow,
        row_count: u16,
        window_start: u16,
        selected: u16,
        total: u16,
        show_arrows: bool,
    ),
    pub subaudio_index_of_code: extern "C" fn(code: u16) -> i32,
    pub subaudio_code_of_index: extern "C" fn(v: i32) -> u16,
    // return length
    pub subaudio_format: extern "C" fn(v: i32, out: *mut u8, cap: u16) -> u16,

    /// handoff buffer, survives `AppResult::Chain` but not app exit.
    /// `handoff_write` fails when `len > HANDOFF_SIZE`;
    /// `handoff_read` returns the number of bytes copied out
    pub handoff_write: extern "C" fn(buf: *const u8, len: u16) -> bool,
    pub handoff_read: extern "C" fn(buf: *mut u8, len: u16) -> u16,

    // RF, tracking-side
    pub set_bandwidth: extern "C" fn(wide: bool),
    pub set_sql_level: extern "C" fn(level: u8),
    pub set_monitor: extern "C" fn(on: bool),
    /// Apply everything programmed so far (frequency, modulation, bandwidth,
    /// subaudio, squelch) to the transceiver and enter RX. `set_*` only
    /// updates the in-memory config; this is what reaches the chip.
    pub enter_rx: extern "C" fn(),
    /// Frequency-only retune that stays in RX, for per-second Doppler
    /// stepping: cheaper than `enter_rx`, but leaves squelch and the audio
    /// path alone.
    pub retune_rx: extern "C" fn(hz: u32, wide: bool),
    /// writes 4 u16: LNAs, LNA, PGA, IF
    pub read_rf_gains: extern "C" fn(out: *mut u16) -> bool,
    /// `menu`: 0=LNAs 1=LNA 2=PGA 3=IF
    pub adjust_rf_gain: extern "C" fn(menu: u8, up: bool),
    /// bit0 = transmitting, bit1 = last TX attempt refused
    pub tx_state: extern "C" fn() -> u32,
    /// Whether the PTT key may key the transmitter while this app is loaded.
    /// Loading an app, and every segment switch, resets this to `false`, so an
    /// app that never calls it cannot transmit at all
    pub set_tx_enabled: extern "C" fn(on: bool),

    /// UTC wall clock, seconds since 2000-01-01T00:00:00Z, or
    /// `UTC_NOT_SET` when nobody has set it since power-on
    pub utc_get: extern "C" fn() -> u32,
    pub utc_set: extern "C" fn(secs: u32),

    pub set_backlight_hold: extern "C" fn(on: bool),

    pub get_master_wide: extern "C" fn() -> bool,
    pub squelch_open: extern "C" fn() -> bool,
    pub set_subaudio_scan_filter: extern "C" fn(on: bool),
    pub detect_subaudio: extern "C" fn() -> i32,
    pub save_master_subaudio: extern "C" fn(code: u16, also_tx: bool),

    pub freq_scan_enable: extern "C" fn(),
    pub freq_scan_disable: extern "C" fn(),
    pub check_freq_scan: extern "C" fn() -> u32,
    pub correct_measured_freq_word: extern "C" fn(raw_word: u32) -> u32,
    pub tune_search_candidate: extern "C" fn(freq_hz: u32, uhf_path: bool),
    pub save_master_vfo: extern "C" fn(freq_hz: u32, subaudio_code: u16),

    // settings app
    pub settings_read: extern "C" fn(buf: *mut u8, len: u16) -> bool,
    pub settings_preview: extern "C" fn(buf: *const u8, len: u16) -> bool,
    pub settings_commit: extern "C" fn(buf: *const u8, len: u16) -> bool,
    pub side_cfg_read: extern "C" fn(buf: *mut u8, len: u16) -> bool,
    pub side_cfg_write: extern "C" fn(buf: *const u8, len: u16) -> bool,
    pub factory_reset: extern "C" fn() -> !,
    pub battery_raw12_avg: extern "C" fn() -> u16,
}

/// `settings_get(5)` / `settings_get(6)` when no position has been entered.
pub const SETTING_COORD_NOT_SET: i32 = i32::MAX;
/// `utc_get()` before anyone has set the clock.
pub const UTC_NOT_SET: u32 = 0;

#[repr(C)]
pub enum AppEvent {
    Enter,
    Key { id: u8, kind: u8 },
    Tick { dt_100us: u32 },
    Draw,
    Leave,
}

#[repr(C)]
pub enum AppResult {
    Continue,
    Exit,
    Fault(u32),
    /// switch to another segment of the same package; the handoff buffer
    /// survives the switch, the arena does not
    Chain(u8),
}

/// Max sub-programs packed into one `.app` package.
pub const MAX_SEGMENTS: usize = 4;

/// One sub-program inside a package.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SegmentEntry {
    /// byte offset of the image from the start of the package
    pub offset: u32,
    pub image_len: u32,
    pub bss_len: u32,
    pub entry_off: u32,
    pub crc32: u32,
}

impl SegmentEntry {
    pub const SIZE: usize = 20;

    pub fn from_bytes(b: &[u8]) -> SegmentEntry {
        let w =
            |i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        SegmentEntry {
            offset: w(0),
            image_len: w(4),
            bss_len: w(8),
            entry_off: w(12),
            crc32: w(16),
        }
    }
}

/// Package header, at offset 0 of a flash slot.
#[repr(C)]
pub struct PackageHeader {
    pub magic: [u8; 4],
    pub build_hash: u32,
    /// 1..=[`MAX_SEGMENTS`]
    pub segment_count: u8,
    pub _pad: [u8; 3],

    /// ASCII, padding to 16
    pub name: [u8; 16],

    pub segments: [SegmentEntry; MAX_SEGMENTS],
}

impl PackageHeader {
    pub const SIZE: usize = 28 + MAX_SEGMENTS * SegmentEntry::SIZE;

    pub fn from_bytes(b: &[u8; Self::SIZE]) -> PackageHeader {
        let mut name = [0u8; 16];
        name.copy_from_slice(&b[12..28]);
        let mut segments = [SegmentEntry::default(); MAX_SEGMENTS];
        for (i, seg) in segments.iter_mut().enumerate() {
            let off = 28 + i * SegmentEntry::SIZE;
            *seg = SegmentEntry::from_bytes(&b[off..off + SegmentEntry::SIZE]);
        }
        PackageHeader {
            magic: [b[0], b[1], b[2], b[3]],
            build_hash: u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            segment_count: b[8],
            _pad: [b[9], b[10], b[11]],
            name,
            segments,
        }
    }

    pub fn name_str(&self) -> &str {
        let end = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());
        core::str::from_utf8(&self.name[..end]).unwrap_or("APP")
    }

    pub fn segment(&self, idx: u8) -> Option<&SegmentEntry> {
        if idx as usize >= self.segment_count as usize
            || self.segment_count as usize > MAX_SEGMENTS
        {
            return None;
        }
        self.segments.get(idx as usize)
    }
}

/// app entry
///
/// ```ignore
/// #[no_mangle]
/// pub extern "C" fn app_entry(api: &Api, ev: AppEvent) -> AppResult { ... }
/// ```
pub type AppEntryFn = extern "C" fn(api: &Api, ev: AppEvent) -> AppResult;
