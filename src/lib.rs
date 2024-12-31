use anyhow::Context;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct Input {
    #[serde(rename = "due")]
    _due: Option<UtcTimestamp>,
    pub github_pat: String,
    pub merging_pr: PullRequest,
    pub traq_pat: String,
    pub traq_messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct PullRequest {
    pub owner: String,
    pub repository: String,
    pub number: u32,
}

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct Message {
    // FIXME: UUID
    pub channel: String,
    pub content: String,
}

impl Input {
    pub fn read_from<F: std::io::Read>(mut file: F) -> anyhow::Result<Self> {
        let mut buf = vec![];
        file.read_to_end(&mut buf)
            .context("failed to read input content")?;
        let slf: Self = serde_json::from_slice(&buf).context("failed to parse input content")?;
        Ok(slf)
    }

    pub fn read_from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)?;
        Self::read_from(file)
    }

    pub fn due(&self) -> Timestamp {
        self._due
            .map(|d| d.with_timezone(&TIMEZONE))
            .unwrap_or_else(akeome_at)
    }
}
