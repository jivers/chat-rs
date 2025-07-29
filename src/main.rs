use std::env;
use anyhow::Result;
use inquire::Text;
use chat::{Chat, Message, Role};
pub mod chat;

fn get_response(chat: &mut Chat, prompt: String) -> Result<String> {
    let response = chat.send(&prompt)?;
    let message = response.first()?;
    Ok(message.content.clone())
}

fn main() -> Result<()> {
    let mut chat = Chat::new("gpt-4o");
    chat.add_message(Message::new(Role::Developer, "you are a helpful terminal-based agent")); 

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 0 {
        let prompt = args.join(" ");
        let response = get_response(&mut chat, prompt)?;
        termimad::print_text(&response);
        return Ok(());
    }
    
    loop {
        let prompt = Text::new("Prompt:").prompt()?;
        let response = get_response(&mut chat, prompt)?; 
    
        termimad::print_text(&response);
    }
}
