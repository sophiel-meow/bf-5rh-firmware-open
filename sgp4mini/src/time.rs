use crate::math::{floor, TWO_PI};

pub fn days_since_j2000(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    sec: f64,
) -> f64 {
    // whole days first, in integers, so the J2000 offset cancels exactly
    let whole = 367 * year - 7 * (year + (month + 9) / 12) / 4
        + 275 * month / 9
        + day
        + 1_721_013
        - 2_451_545;
    let frac = (sec / 60.0 + minute as f64) / 60.0 + hour as f64;
    whole as f64 + 0.5 + frac / 24.0
}

/// Days since J2000 for a TLE epoch field (two-digit year, fractional day of
/// year). Years 57..=99 mean 1957..=1999, 0..=56 mean 2000..=2056
pub fn days_since_j2000_from_tle_epoch(
    two_digit_year: i32,
    day_of_year: f64,
) -> f64 {
    let year = if two_digit_year < 57 {
        two_digit_year + 2000
    } else {
        two_digit_year + 1900
    };
    // day_of_year is 1-based, so January 1.0 is `days_since_j2000(y,1,1,0,0,0)`
    days_since_j2000(year, 1, 1, 0, 0, 0.0) + (day_of_year - 1.0)
}

/// Days from 2000-01-01 to a civil date, proleptic Gregorian, negative before
/// 2000. Howard Hinnant's `days_from_civil`, shifted to a 2000 epoch: pure
/// integer arithmetic, so the radio's clock display costs no soft-float.
pub fn days_from_civil(year: i32, month: i32, day: i32) -> i32 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400; // 0..=399
    let mp = (month + 9) % 12; // March = 0
    let doy = (153 * mp + 2) / 5 + day - 1; // 0..=365
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // 0..=146096
                                                     // 730425 = days from 0000-03-01 (the era origin) to 2000-01-01
    era * 146_097 + doe - 730_425
}

/// Inverse of [`days_from_civil`], returning `(year, month, day)`.
pub fn civil_from_days(days: i32) -> (i32, i32, i32) {
    let z = days + 730_425;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // 0..=146096
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // 0..=399
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // 0..=365
    let mp = (5 * doy + 2) / 153; // 0..=11, March = 0
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i32::from(month <= 2);
    (year, month, day)
}

/// Days since J2000 for the radio's wall clock, which counts UTC seconds from
/// 2000-01-01T00:00:00Z. That instant is half a day before the J2000 epoch.
pub fn days_since_j2000_from_utc_secs(secs: u32) -> f64 {
    secs as f64 / 86400.0 - 0.5
}

/// Greenwich mean sidereal time in rad, from the IAU expression.
pub fn gmst_rad(days_j2000: f64) -> f64 {
    let t = days_j2000 / 36525.0;
    let day_frac = days_j2000 - floor(days_j2000);
    let sec = 67310.54841
        + 8_640_184.812866 * t
        + 0.093104 * t * t
        + -6.2e-6 * t * t * t
        + 86400.0 * day_frac;
    let folded = sec - 86400.0 * floor(sec / 86400.0);
    folded * (TWO_PI / 86400.0)
}
