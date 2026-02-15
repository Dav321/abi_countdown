use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Event {
    pub name: String,
    pub flag: String,
    pub time: DateTime<Utc>,
    pub color: String,
}
