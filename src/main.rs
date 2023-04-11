use conversation::Conversation;
use std::io;
use dotenv::dotenv;

use crate::llm_interpreter::get_commands;

mod conversation;
mod transcription;
mod simulations;
mod llm_api;
mod llm_interpreter;
mod serial_output;

// TODO find dynamically
const PORT_NUM: &str = "/dev/cu.usbmodem1112401";

fn main() {
    dotenv().ok();

    // For now, commend out this paragraph if you want to run the model without output
    // TODO gate this with a CLI option
    let mut connection = serial_output::open_connection(PORT_NUM);
    println!("Testing serial output");
    serial_output::send_instructions(&mut connection, get_commands("[forward-2]"));

    // Initialize conversation
    let mut conversation = Conversation::new(simulations::ASSISTANT);
    println!("Conversation started");

    // Loop over input lines until the user quits or the conversation ends
    while !conversation.is_over() {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let input = line.trim();

        let next_fn = if input.starts_with(".") { run_command } else { next_dialogue };
        conversation = next_fn(input, conversation);
    }
}

fn next_dialogue(input: &str, mut conversation: Conversation) -> Conversation {
    let dialogue = if input == "" {
        match transcription::listen() {
            Ok(transcription) => {
                println!("{}", transcription);
                transcription
            },
            // If there was an error transcribing, inform the user and return the same conversation
            Err(e) => {
                eprint!("{}", e);
                say("Sorry, I didn't quite catch that. Could you try again?");
                return conversation;
            }
        }
    } else {
        // If the input wasn't empty, just use that text input as the next line of dialogue
        input.to_string()
    };

    let response = conversation.say(&dialogue).unwrap_or_else(|err| {
        eprint!("{}", err);
        "Sorry, I wasn't able to connect to the internet. Please try again.".to_owned()
    });
    say(&response);
    conversation
}

fn say (input: &str) {
    // TODO use the system "say" command
    eprintln!("{}", input);
}

fn run_command (command: &str, conversation: Conversation) -> Conversation {
    match command {
        ".a" => Conversation::new(simulations::ASSISTANT),
        ".i" => Conversation::new(simulations::INTERROGATION),
        ".r" => Conversation::new(simulations::RCCAR),
        ".dump" => {
            eprintln!("TODO: implement history dump");
            conversation
        }
        _ => {
            eprintln!("Error: Unknown command");
            conversation
        }
    }
}

