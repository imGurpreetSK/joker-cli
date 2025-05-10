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

async fn get_joke(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let joke = response.json::<Joke>().await?;

    let mut output = String::new();

    match joke.error {
        true => {
            output.push_str("Error occurred while fetching joke");
        }
        false => match (joke.joke, joke.setup, joke.delivery) {
            (Some(joke), _, _) => output.push_str(&joke),
            (_, Some(setup), Some(delivery)) => {
                output.push_str(&format!("{}\n\n{}", setup, delivery))
            }
            _ => output.push_str("No joke found"),
        },
    }

    Ok(output)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let joke = get_joke("https://v2.jokeapi.dev/joke/Any").await?;
    println!("{}", joke);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_single_line_joke() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/joke/Any")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "error": false,
                "category": "Programming",
                "type": "single",
                "joke": "Why do programmers prefer dark mode? Because light attracts bugs!",
                "id": 42
            }"#,
            )
            .create();

        let result = get_joke(&format!("{}/joke/Any", server.url()))
            .await
            .unwrap();

        assert_eq!(
            result,
            "Why do programmers prefer dark mode? Because light attracts bugs!"
        );
    }

    #[tokio::test]
    async fn test_two_part_joke() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/joke/Any")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "error": false,
                "category": "Programming",
                "type": "twopart",
                "setup": "Why don't programmers like nature?",
                "delivery": "It has too many bugs.",
                "id": 43
            }"#,
            )
            .create();

        let result = get_joke(&format!("{}/joke/Any", server.url()))
            .await
            .unwrap();
        assert_eq!(
            result,
            "Why don't programmers like nature?\n\nIt has too many bugs."
        );
    }

    #[tokio::test]
    async fn test_error_response() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/joke/Any")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "error": true,
                "internalError": false,
                "code": 106,
                "message": "No matching joke found",
                "causedBy": ["No jokes found for the provided flags"]
            }"#,
            )
            .create();

        let result = get_joke(&format!("{}/joke/Any", server.url()))
            .await
            .unwrap();
        assert_eq!(result, "Error occurred while fetching joke");
    }

    #[tokio::test]
    async fn test_missing_joke_data() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/joke/Any")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "error": false,
                "category": "Programming",
                "id": 44
            }"#,
            )
            .create();

        let result = get_joke(&format!("{}/joke/Any", server.url()))
            .await
            .unwrap();
        assert_eq!(result, "No joke found");
    }

    #[tokio::test]
    async fn test_http_error() {
        let mut server = mockito::Server::new_async().await;
        let _m = server.mock("GET", "/joke/Any").with_status(500).create();

        let result = get_joke(&format!("{}/joke/Any", server.url())).await;
        assert!(result.is_err());
    }
}
