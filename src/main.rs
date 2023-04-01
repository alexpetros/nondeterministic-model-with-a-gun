use dotenv::dotenv;

mod conversation;
mod transcription;

fn main() {
    dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("Missing API key");

    // let input = transcription::listen();
    // println!("{}", input);
    // let response = conversation::send_prompt(&input);

    let mut conversation = conversation::Conversation::new(&openai_api_key);
    let response = conversation.say("test");
    println!("{}", response);
}
