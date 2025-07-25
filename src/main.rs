use std::env;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use ureq;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Role {
    Developer,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: Role,
    content: String,
}

#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

impl ChatRequest {
    fn new(content: String) -> ChatRequest {
        ChatRequest {
            model: "gpt-4.1".to_string(),
            messages: vec![Message {
                role: Role::User,
                content,
            }]
        }
    }

}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

impl ChatResponse {
    fn first(&self) -> Result<String> {
        let first_choice = self.choices.get(0).context("no first choice exists")?;
        Ok(first_choice.message.content.clone())
    }
}

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
