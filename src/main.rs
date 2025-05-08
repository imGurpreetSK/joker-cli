use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Joke {
    id: Option<u32>,
    joke: Option<String>,
    setup: Option<String>,
    delivery: Option<String>,
    category: Option<String>,
    error: bool,
}

fn main() {
    println!("Hello, world!");
}