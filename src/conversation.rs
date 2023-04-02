// use std::collections::HashMap;

use reqwest::header::AUTHORIZATION;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::{Serialize, Deserialize};

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String
}

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

pub struct Conversation {
    history: Vec<Message>,
    api_key: String
}

impl Conversation {
    pub fn new<'a>(api_key: &str, system_prompt: &str) -> Conversation {
        let mut conversation = Conversation { history: vec![], api_key: api_key.to_string() };
        conversation.add_response("system", system_prompt);
        conversation
    }

    // TODO convert to  result
    pub fn say(&mut self, prompt: &str) -> String {
        let authorization = format!("Bearer {}", self.api_key);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, authorization.parse().unwrap());

        self.add_response("user", prompt);
        let messages = serde_json::to_string(&self.history).unwrap();
        let body = format!(r#"{{"model": "gpt-3.5-turbo", "messages": {} }}"#, messages);
        // println!("{}", &body);

        let client = reqwest::blocking::Client::new();
        let res = client.post(OPENAI_URL)
            .headers(headers)
            .body(body)
            .send()
            .expect("Unable to get response from API")
            .text()
            .expect("Unable to parse response");

        let json: ApiResponse  = serde_json::from_str(&res).unwrap();
        // TODO convert to .get and then to Result
        json.choices[0].message.content.clone()
    }

    fn add_response(&mut self, role: &str, content: &str) {
        let response = Message { role: role.to_string(), content: content.to_string() };
        self.history.push(response)
    }

}
