#![cfg_attr(not(test), no_std)]

pub mod look;
pub mod math;
pub mod record;
pub mod sgp4;
pub mod time;

pub use look::{doppler_hz, look, sub_point, Look, Observer, SubPoint};
pub use record::{Record, MAX_SATELLITES, RECORD_SIZE, TABLE_BYTES};
pub use sgp4::{Constants, Elements, Error, Prediction};
pub use time::{
    civil_from_days, days_from_civil, days_since_j2000,
    days_since_j2000_from_tle_epoch, days_since_j2000_from_utc_secs, gmst_rad,
};
