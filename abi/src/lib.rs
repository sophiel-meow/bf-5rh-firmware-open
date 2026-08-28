#![no_std]

include!(concat!(env!("OUT_DIR"), "/build_hash.rs"));

pub const MAGIC: [u8; 4] = *b"OVL1";

pub const ARENA_SIZE: usize = 8192;

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
}

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
}

#[repr(C)]
pub struct ImageHeader {
    pub magic: [u8; 4],
    pub build_hash: u32,
    pub image_len: u32,
    pub bss_len: u32,
    pub entry_off: u32,
    pub crc32: u32,

    /// ASCII, padding to 16
    pub name: [u8; 16],
}

impl ImageHeader {
    pub const SIZE: usize = 40;

    pub fn from_bytes(b: &[u8; Self::SIZE]) -> ImageHeader {
        let mut name = [0u8; 16];
        name.copy_from_slice(&b[24..40]);
        ImageHeader {
            magic: [b[0], b[1], b[2], b[3]],
            build_hash: u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            image_len: u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
            bss_len: u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
            entry_off: u32::from_le_bytes([b[16], b[17], b[18], b[19]]),
            crc32: u32::from_le_bytes([b[20], b[21], b[22], b[23]]),
            name,
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
}

/// app entry
///
/// ```ignore
/// #[no_mangle]
/// pub extern "C" fn app_entry(api: &Api, ev: AppEvent) -> AppResult { ... }
/// ```
pub type AppEntryFn = extern "C" fn(api: &Api, ev: AppEvent) -> AppResult;
