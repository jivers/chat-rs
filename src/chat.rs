use std::env;
use ureq;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

pub struct Chat {
    // todo add message history
    model: String,
}

impl Chat {
    pub fn new(model: &str) -> Chat {
        Chat {
            model: model.to_string(),
        }
    }

    pub fn send(&self, content: &String) -> Result<ChatResponse> {
        let chat_request = ChatRequest::new(&self.model, content);
        let openai_api_key = env::var("OPENAI_API_KEY")?;

        let mut response = ureq::post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", openai_api_key))
            .header("Content-Type", "application/json")
            .send_json(chat_request)?;

        let response = response
            .body_mut()
            .read_json::<ChatResponse>()?;

        Ok(response)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Developer,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: Role,
    content: String,
}

#[derive(Serialize, Debug)]
pub struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

impl ChatRequest {
    pub fn new(model: &String, content: &String) -> ChatRequest {
        ChatRequest {
            model: model.clone(),
            messages: vec![Message {
                role: Role::User,
                content: content.to_string(),
            }]
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    choices: Vec<Choice>,
}

impl ChatResponse {
    pub fn first(&self) -> Result<String> {
        let first_choice = self.choices.get(0).context("no first choice exists")?;
        Ok(first_choice.message.content.clone())
    }
}
