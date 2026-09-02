use super::overlay;
use super::App;

/// Every launcher slot is an overlay app now
pub fn total_item_count() -> usize {
    overlay::SLOT_COUNT as usize
}

pub fn label_at(app: &mut App, index: usize, w: &mut dyn core::fmt::Write) {
    overlay::slot_name(app, index as u8, w);
}

pub fn is_available_at(app: &mut App, index: usize) -> bool {
    overlay::slot_valid(app, index as u8)
}
