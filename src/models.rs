use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Joke {
    pub id: Option<u32>,
    pub joke: Option<String>,
    pub setup: Option<String>,
    pub delivery: Option<String>,
    pub category: Option<String>,
    pub error: bool,
}
