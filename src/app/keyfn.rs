use super::{scan, search, App};
use cortex_m::peripheral::SYST;

// TODO: FM radio was made as app, add key function to app
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeyFunction {
    None,
    WideNarrow,
    Monitor,
    Mode,
    TxTone,
    Scan,
    Power,
    Flashlight,
    Search,
    Reverse,
}

impl KeyFunction {
    pub fn label(self) -> &'static str {
        match self {
            KeyFunction::None => "NONE",
            KeyFunction::WideNarrow => "WIDE/NAR",
            KeyFunction::Monitor => "MONITOR",
            KeyFunction::Mode => "MODE",
            KeyFunction::TxTone => "TX TONE",
            KeyFunction::Scan => "SCAN",
            KeyFunction::Power => "POWER",
            KeyFunction::Flashlight => "LIGHT",
            KeyFunction::Search => "SEARCH",
            KeyFunction::Reverse => "REVERSE",
        }
    }
}

pub(super) fn from_u8(v: u8) -> KeyFunction {
    match v {
        1 => KeyFunction::WideNarrow,
        2 => KeyFunction::Monitor,
        3 => KeyFunction::Mode,
        4 => KeyFunction::TxTone,
        // 5=> fm radio
        6 => KeyFunction::Scan,
        7 => KeyFunction::Power,
        8 => KeyFunction::Flashlight,
        9 => KeyFunction::Search,
        10 => KeyFunction::Reverse,
        _ => KeyFunction::None,
    }
}

pub(super) fn invoke(app: &mut App, syst: &mut SYST, func: KeyFunction) {
    match func {
        KeyFunction::None | KeyFunction::TxTone => {}
        KeyFunction::WideNarrow => app.toggle_wide_narrow(syst),
        KeyFunction::Monitor => app.radio.toggle_monitor(),
        KeyFunction::Mode => app.toggle_modulation(syst),
        // TODO: fm radio
        KeyFunction::Scan => scan::enter(app, syst),
        KeyFunction::Power => app.toggle_power(syst),
        KeyFunction::Flashlight => app.flashlight.toggle(),
        KeyFunction::Search => search::enter(app, syst),
        KeyFunction::Reverse => app.toggle_reverse(syst),
    }
}
