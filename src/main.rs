use dotenv::dotenv;

mod conversation;
mod transcription;
mod simulations;

fn main() {
    dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("Missing API key");

    // let input = transcription::listen();
    // println!("{}", input);
    // let response = conversation::send_prompt(&input);

    let prompt = simulations::INTERROGATION.initial_prompt;
    let mut conversation = conversation::Conversation::new(&openai_api_key, prompt);
    let response = conversation.say("Where am I?");
    println!("{}", response);
}
