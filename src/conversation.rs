use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::command_interpreters::Instruction;
use crate::llm_api::get_next_message;
use crate::simulations::Simulation;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    content: String
}


pub struct Conversation {
    pub simulation: Simulation,
    pub history: Vec<Message>,
    pub api_key: String,
}

impl Conversation {
    pub fn new<'a>(simulation: Simulation) -> Conversation {
        let api_key = std::env::var("OPENAI_API_KEY").expect("Missing API key");
        let content = simulation.initial_prompt.clone();

        let mut conversation = Conversation { simulation, history: vec![], api_key };
        conversation.add_response("system", content);
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

    pub fn is_over(&mut self) -> bool {
        // I'm not sure if this is the best way to express this, but it's not terrible
        match self.simulation.end_condition {
            None => false,
            Some(end_string) => self.history.len() > 1 && self.history.last().unwrap().content.contains(end_string)
        }
    }

    pub fn filter_instructions(&mut self, text: &str) -> (String, Vec<Instruction>) {
        if let Some(filter_fn) = self.simulation.filter_fn {
            filter_fn(&text)
        } else {
            (text.to_owned(), vec![])
        }
    }
}
