use chrono::{DateTime, Utc};
use chrono_tz::Tz;

pub const TIMEZONE: Tz = chrono_tz::Asia::Tokyo;
pub type Timestamp = DateTime<Tz>;
pub type UtcTimestamp = DateTime<Utc>;

pub fn akeome_at() -> Timestamp {
    use chrono::TimeZone;

    TIMEZONE.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap()
}
