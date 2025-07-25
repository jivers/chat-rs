use std::env;
use anyhow::Result;
use ureq;
pub mod chat;
use chat::{ChatRequest, ChatResponse};

fn main() -> Result<()> {
    let input = match env::args().nth(1) {
        Some(x) => x,
        None => "gimme the beat boy".to_string(),
    };

    let chat_request = ChatRequest::new(input);

    let openai_api_key = env::var("OPENAI_API_KEY")?;
    let mut response = ureq::post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .header("Content-Type", "application/json")
        .send_json(chat_request)?;

    let response = response
        .body_mut()
        .read_json::<ChatResponse>()?;

    let chat_response = response.first()?;
    termimad::print_inline(&chat_response);

    Ok(())
}
