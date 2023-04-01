// use std::collections::HashMap;

use reqwest::header::AUTHORIZATION;
use reqwest::header::{HeaderMap, CONTENT_TYPE};

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct Conversation<'a> {
    history: Vec<String>,
    api_key: &'a str
}

impl Conversation<'_> {
    pub fn new(api_key: &str) -> Conversation {
        Conversation { history: vec![], api_key }
    }

    pub fn say(&mut self, prompt: &str) -> String {
        let authorization = format!("Bearer {}", self.api_key);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, authorization.parse().unwrap());

        let body = r#"{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Hello!"}] }"#;

        let client = reqwest::blocking::Client::new();
        let res = client.post(OPENAI_URL)
            .headers(headers)
            .body(body)
            .send()
            .expect("Unable to get response from API")
            .text()
            .expect("Unable to parse response");

        res

    }
}
