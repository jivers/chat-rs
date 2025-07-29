use std::env;
use anyhow::Result;
use inquire::Text;
use chat::{Chat, Message, Role};
use assistant::{Assistant, Action};

pub mod chat;
pub mod assistant;

fn get_response(chat: &mut Chat, prompt: String) -> Result<String> {
    let response = chat.send(&prompt)?;
    let message = response.first()?;
    Ok(message.content.clone())
}

fn main() -> Result<()> {
    let mut chat = Chat::new("o4-mini");
    chat.add_message(Message::new(Role::Developer, "you generate only ffmpeg commands and nothing else")); 

    let mut action = Action { 
        chat,
        post : Some(Box::new(|mut s: String| {
            s = s.replace("```", "");
            if let Some(newline_index) = s.find('\n') {
                s = (&s[newline_index + 1..]).to_string();
            }
            s.trim().to_string()
        })),
    };

    let result = action.run("convert test.wav which is a 5.1 dolby 48khz file to a stereo 96 kbpsto test.mp3")?;
    println!("{}", result);

    let mut chat = Chat::new("gpt-4o");
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
