mod models;
use models::Joke;
use notify_rust::{Notification, Timeout};

#[derive(Debug, PartialEq)]
enum JokeType {
    Single(String),
    TwoPart(String, String),
    Error(String),
}

async fn get_joke(url: &str) -> Result<JokeType, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let joke = response.json::<Joke>().await?;

    let output: JokeType;
    match joke.error {
        true => {
            output = JokeType::Error("Error occurred while fetching joke".to_string());
        }
        false => match (joke.joke, joke.setup, joke.delivery) {
            (Some(joke), _, _) => output = JokeType::Single(joke),
            (_, Some(setup), Some(delivery)) => output = JokeType::TwoPart(setup, delivery),
            _ => output = JokeType::Error("No joke found".to_string()),
        },
    }

    Ok(output)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let joke = get_joke("https://v2.jokeapi.dev/joke/Any").await?;
    match joke {
        JokeType::Single(joke) => {
            println!("{}", joke);
            if let Err(e) = send_notification("Joke", &joke) {
                eprintln!("Error sending notification: {:?}", e);
            }
        }
        JokeType::TwoPart(first, second) => {
            println!("{}", first);
            println!("{}", second);
            if let Err(e) = send_notification(&first, &second) {
                eprintln!("Error sending notification: {:?}", e);
            }
        }
        JokeType::Error(_) => {
            eprintln!("Error fetching joke, trying again.");
            let _ = main();
        }
    }

    Ok(())
}

fn send_notification(summary: &str, body: &str) -> Result<(), notify_rust::error::Error> {
    Notification::new()
        .summary(summary)
        .body(body)
        .timeout(Timeout::Never) // Make notification manually dismissible
        .show()?;
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
            JokeType::Single(
                "Why do programmers prefer dark mode? Because light attracts bugs!".to_string()
            )
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
            JokeType::TwoPart(
                "Why don't programmers like nature?".to_string(),
                "It has too many bugs.".to_string()
            )
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
        assert_eq!(
            result,
            JokeType::Error("Error occurred while fetching joke".to_string())
        );
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
        assert_eq!(result, JokeType::Error("No joke found".to_string()));
    }

    #[tokio::test]
    async fn test_http_error() {
        let mut server = mockito::Server::new_async().await;
        let _m = server.mock("GET", "/joke/Any").with_status(500).create();

        let result = get_joke(&format!("{}/joke/Any", server.url())).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_send_notification_success() {
        let result = send_notification("Test Summary", "Test Body");
        assert!(result.is_ok());
    }
}
