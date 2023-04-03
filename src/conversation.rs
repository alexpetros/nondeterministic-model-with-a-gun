use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::llm_api::get_next_message;
use crate::simulations::Simulation;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    content: String
}


pub struct Conversation {
    pub history: Vec<Message>,
    pub api_key: String
}

impl Conversation {
    pub fn new<'a>(simulation: Simulation) -> Conversation {
        let api_key = std::env::var("OPENAI_API_KEY").expect("Missing API key");
        let mut conversation = Conversation { history: vec![], api_key };
        conversation.add_response("system", simulation.initial_prompt);
        conversation
    }

    pub fn say(&mut self, prompt: &str) -> Result<String, Box<dyn Error>> {
        self.add_response("user", prompt);
        let message = get_next_message(&self)?;
        self.add_response(&message.role, &message.content);
        Ok(message.content.clone())
    }

    fn add_response(&mut self, role: &str, content: &str) {
        let response = Message { role: role.to_string(), content: content.to_string() };
        self.history.push(response)
    }
}
