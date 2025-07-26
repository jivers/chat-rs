use std::env;
use ureq;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

pub struct Chat {
    // todo add message history
    model: String,
    messages: Vec<Message>,
}

impl Chat {
    pub fn new(model: &str) -> Chat {
        Chat {
            model: model.to_string(),
            messages: Vec::<Message>::new(),
        }
    }

    pub fn send(&mut self, content: &String) -> Result<ChatResponse> {
        let message = Message::new(Role::User, content);
        self.messages.push(message);

        let chat_request = ChatRequest::new(&self.model, &self.messages);
        let openai_api_key = env::var("OPENAI_API_KEY")?;

        let mut response = ureq::post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", openai_api_key))
            .header("Content-Type", "application/json")
            .send_json(chat_request)?;

        let response = response
            .body_mut()
            .read_json::<ChatResponse>()?;
        
        let message = response.first()?;
        self.messages.push(message);

        Ok(response)
    }

    pub fn print_messages(&self) {
        dbg!(&self.messages);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Developer,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new(role: Role, content: &String) -> Message {
        Message {
            role,
            content: content.clone(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [Message], 
}

impl<'a> ChatRequest<'a> {
    pub fn new(model: &'a str, messages: &'a [Message]) -> ChatRequest<'a> {
        ChatRequest {
            model,
            messages,
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
    pub fn first(&self) -> Result<Message> {
        let first_choice = self.choices.get(0).context("no first choice exists")?;
        Ok(first_choice.message.clone())
    }
}
