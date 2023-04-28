use crate::conversation::{Message, Conversation};
use std::error::Error;
use reqwest::header::AUTHORIZATION;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use core::fmt;
use serde::{Serialize, Deserialize};
const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
    finish_reason: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    id: String,
    choices: Vec<Choice>
}

#[derive(Debug, Clone)]
struct EmptyMessageError;

impl std::error::Error for EmptyMessageError {}
impl fmt::Display for EmptyMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "API returned an empty set of choices")
    }
}

pub fn get_next_message(conversation: &Conversation) -> Result<Message, Box<dyn Error>> {
    let messages = serde_json::to_string(&conversation.history).expect("Unable to serialize history");
    // let body = format!(r#"{{"model": "gpt-3.5-turbo", "messages": {} }}"#, messages);
    let body = format!(r#"{{"model": "gpt-4", "messages": {} }}"#, messages);
    let authorization = format!("Bearer {}", conversation.api_key);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, authorization.parse().unwrap());

    let client = reqwest::blocking::Client::new();
    let res = client.post(OPENAI_URL).headers(headers).body(body).send()?;
    let body = res.text()?;
    let mut json: ApiResponse  = serde_json::from_str(&body)?;
    let choice = json.choices.swap_remove(0); // Remove the first element of the Vec
    Ok(choice.message)
}
