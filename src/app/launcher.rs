use super::overlay;
use super::{App, Mode};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LauncherEntry {
    Settings,
    ScanQt,
    Search,
}

pub const STATIC_ITEMS: &[LauncherEntry] = &[
    LauncherEntry::Settings,
    LauncherEntry::ScanQt,
    LauncherEntry::Search,
];

impl LauncherEntry {
    pub fn label(self) -> &'static str {
        match self {
            LauncherEntry::Settings => "SETTINGS",
            LauncherEntry::ScanQt => "QT SCAN",
            LauncherEntry::Search => "FREQ HUNT",
        }
    }

    pub fn is_available(self) -> bool {
        true
    }

    pub fn target_mode(self) -> Mode {
        match self {
            LauncherEntry::Settings => Mode::Settings,
            LauncherEntry::ScanQt => Mode::ScanQt,
            LauncherEntry::Search => Mode::Search,
        }
    }
}

pub fn total_item_count() -> usize {
    STATIC_ITEMS.len() + overlay::SLOT_COUNT as usize
}

pub fn label_at(app: &mut App, index: usize, w: &mut dyn core::fmt::Write) {
    if index < STATIC_ITEMS.len() {
        let _ = write!(w, "{}", STATIC_ITEMS[index].label());
    } else {
        overlay::slot_name(app, (index - STATIC_ITEMS.len()) as u8, w);
    }
}

pub fn is_available_at(app: &mut App, index: usize) -> bool {
    if index < STATIC_ITEMS.len() {
        STATIC_ITEMS[index].is_available()
    } else {
        overlay::slot_valid(app, (index - STATIC_ITEMS.len()) as u8)
    }
}
