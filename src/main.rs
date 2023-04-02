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

    // Loop over input lines until the user quits
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let input_line = line.trim();

        if input_line.starts_with(".") {
            conversation = match input_line {
                ".dump" => { eprintln!("TODO: implement history dump"); conversation }
                ".a" => Conversation::new(&openai_api_key, ASSISTANT.initial_prompt),
                ".i" => Conversation::new(&openai_api_key, INTERROGATION.initial_prompt),
                _ => { eprintln!("Error: Unknown command"); conversation }
            }
        } else {
            let dialogue = if input_line == "" { transcription::listen() } else { input_line.to_string() };
            let response = conversation.say(&dialogue);
            println!("{}", response);
        }

    }
}
