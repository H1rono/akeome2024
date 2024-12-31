use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;

pub const TIMEZONE: Tz = chrono_tz::Asia::Tokyo;
pub type Timestamp = DateTime<Tz>;
pub type UtcTimestamp = DateTime<Utc>;
pub type Duration = chrono::TimeDelta;

pub fn now() -> Timestamp {
    Utc::now().with_timezone(&TIMEZONE)
}

pub fn akeome_at() -> Timestamp {
    TIMEZONE.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap()
}

pub fn duration_until<Tz: TimeZone>(timestamp: DateTime<Tz>) -> Duration {
    let timestamp = timestamp.with_timezone(&TIMEZONE);
    let now = now();
    timestamp - now
}
