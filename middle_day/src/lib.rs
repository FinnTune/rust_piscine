extern crate chrono;
use chrono::{Utc, Datelike, TimeZone, LocalResult};
pub use chrono::Weekday as wd;  // Alias as wd for convenience

pub fn middle_day(year: i32) -> Option<wd> {
    // Accurate leap year check
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

    // Check if the year has an even number of days
    if is_leap && 366 % 2 == 0 || !is_leap && 365 % 2 == 0 {
        return None;
    }

    // Calculate the middle day
    let middle_day = if is_leap {
        366 / 2 + 1
    } else {
        365 / 2 + 1
    };

    // Create a DateTime for the middle day and get its weekday
    let date = match Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0) {
        LocalResult::Single(dt) => dt,
        _ => return None,
    };

    let new_date = date + chrono::Duration::days(middle_day as i64 - 1);
    Some(new_date.weekday())
}
