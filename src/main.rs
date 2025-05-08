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

async fn get_joke() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://v2.jokeapi.dev/joke/Any").await?;
    let joke = response.json::<Joke>().await?;

    match joke.error {
        true => {
            println!("Error occurred while fetching joke");
        }
        false => {
            match (joke.joke, joke.setup, joke.delivery) {
                (Some(joke), _, _) => println!("{}", joke),
                (_, Some(setup), Some(delivery)) => println!("{}\n\n{}", setup, delivery),
                _ => println!("No joke found"),
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_joke().await
}
