use conversation::Conversation;
use std::env;
use std::{io, process::Command};
use dotenv::dotenv;
use crate::serial_output::send_instructions;

mod conversation;
mod transcription;
mod simulations;
mod llm_api;
mod serial_output;
mod command_interpreters;

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    let mut connection = None;
    if !args.iter().any(|a| a == "--nc") {
        connection =  Some(serial_output::get_usb_connection());
    }

    // Initialize conversation

    let mut conversation = Conversation::new(simulations::FOAMCAR);
    println!("Conversation started");

    // Loop over input lines until the user quits
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let input = line.trim();

        if input.starts_with(".") {
            conversation = run_command(input, conversation);
            continue;
        }

        let next_dialogue = if input == "" { listen() } else { input.to_owned() };
        let response = match conversation.say(&next_dialogue) {
            Ok(response) => response,
            Err(_err) => "Sorry, I wasn't able to connect to the internet. Please try again.".to_owned()
        };

        let (spoken_text, instructions) = conversation.filter_instructions(&response);
        say(&spoken_text);
        if let Some(mut conn) = connection.as_mut() {
            send_instructions(&mut conn, instructions);
        }
    }

}

fn listen() -> String {
    loop {
        let transcription = transcription::listen();
        match transcription {
            Ok(transcription) => {
                println!("{}", transcription);
                return transcription;
            },
            Err(e) => {
                eprint!("{}", e);
                say("Sorry, I didn't quite catch that. Could you try again?");
            }
        }
    }
}

fn say (input: &str) {
    // TODO use the system "say" command
    eprintln!("{}", input);
    Command::new("say")
        .args([input])
        .output()
        .expect("Failed to execute say");
}

fn run_command (command: &str, conversation: Conversation) -> Conversation {
    match command {
        ".a" => Conversation::new(simulations::ASSISTANT),
        ".e" => Conversation::new(simulations::ETHICS),
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

