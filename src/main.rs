use simulations::{INTERROGATION, ASSISTANT};
use conversation::Conversation;
use std::io;
use dotenv::dotenv;

mod conversation;
mod transcription;
mod simulations;

fn main() {
    dotenv().ok();

    // Initialize conversation
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("Missing API key");
    let mut conversation = Conversation::new(&openai_api_key, ASSISTANT.initial_prompt);
    println!("Conversation started");

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");

        let mut input = String::new();
        match line.trim() {
            ".dump" => println!("TODO: History dump"),
            ".i" => conversation = Conversation::new(&openai_api_key, INTERROGATION.initial_prompt),
            "" => input = transcription::listen(),
            _ => input = line
        }

        if input != "" {
            let response = conversation.say(&input);
            println!("{}", response);
        }
    }
}
