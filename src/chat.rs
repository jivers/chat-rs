use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

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
    pub fn new(content: String) -> ChatRequest {
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
