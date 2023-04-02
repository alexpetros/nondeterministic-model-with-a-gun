use conversation::Conversation;
use std::io;
use dotenv::dotenv;

mod conversation;
mod transcription;
mod simulations;

fn main() {
    dotenv().ok();
    // Initialize conversation
    let mut conversation = Conversation::new(simulations::ASSISTANT);
    println!("Conversation started");

    // Loop over input lines until the user quits
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let input_line = line.trim();

        conversation = if input_line.starts_with(".") {
            run_command(input_line, conversation)
        } else {
            next_dialogue(input_line, conversation)
        }
    }
}

fn next_dialogue(input: &str, mut conversation: Conversation) -> Conversation {
    let dialogue = if input == "" { transcription::listen() } else { input.to_string() };
    let response = conversation
        .say(&dialogue)
        .unwrap_or_else(|err| {
            eprint!("{}", err);
            "Sorry, I didn't quite catch that. Could you try again?".to_string()
        });
    println!("{}", response);
    conversation
}

fn run_command (command: &str, conversation: Conversation) -> Conversation {
    match command {
        ".a" => Conversation::new(simulations::ASSISTANT),
        ".i" => Conversation::new(simulations::INTERROGATION),
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

