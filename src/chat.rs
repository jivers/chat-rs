use std::env;
use ureq;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use crate::tool::{Tool, ToolCall};

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
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl Message {
    pub fn new(role: Role, content: &str) -> Message {
        Message {
            role,
            content: Some(content.to_string()),
            tool_calls: None,
        }
    }
}
pub struct Chat {
    model: String,
    messages: Vec<Message>,
    tools: Vec<Tool>,
}

impl Chat {
    pub fn new(model: &str) -> Chat {
        Chat {
            model: model.to_string(),
            messages: Vec::<Message>::new(),
            tools: Vec::<Tool>::new(),
        }
    }

    pub fn send(&mut self, content: &str) -> Result<ChatResponse> {
        let message = Message::new(Role::User, content);
        self.messages.push(message);

        let chat_request = ChatRequest::new(&self.model, &self.messages, self.tools.clone());
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

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }

    pub fn add_user_message(&mut self, content: &str) {
        self.messages.push(Message::new(Role::User, content));
    }

    pub fn add_dev_message(&mut self, content: &str) {
        self.messages.push(Message::new(Role::Developer, content));
    }

    // I should really just implement a custom debug or display trait here! 
    pub fn print_messages(&self) {
        dbg!(&self.messages);
    }
}



#[derive(Serialize, Debug)]
pub struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [Message], 

    tools: Vec<Tool>,
}

impl<'a> ChatRequest<'a> {
    pub fn new(model: &'a str, messages: &'a [Message], tools: Vec<Tool>) -> ChatRequest<'a> {
        ChatRequest {
            model,
            messages,
            tools: tools,
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
