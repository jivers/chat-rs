use std::env;
use anyhow::Result;
use inquire::Text;
use chat::{Chat, Message, Role};
use assistant::{Action};
use atty::Stream;
use std::io::{self, Read};
use termimad::print_text;

pub mod chat;
pub mod assistant;

fn get_response(chat: &mut Chat, prompt: String) -> Result<String> {
    let response = chat.send(&prompt)?;
    let message = response.first()?;
    Ok(message.content.clone())
}

fn main() -> Result<()> {
    let mut prompt = String::new();
    let mut chat = Chat::new("gpt-4o");

    // handle pipe input
    if atty::is(Stream::Stdin) {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() > 0 {
            prompt = args.join(" ");
            let response = get_response(&mut chat, prompt)?;
            print_text(&response);
            return Ok(());
        }
    }; 


    // handle stdin
    if atty::isnt(Stream::Stdin) {
         io::stdin().read_to_string(&mut prompt)?;
         let response = get_response(&mut chat, prompt)?;
         print_text(&response); 
         return Ok(())
     };


    // handle interactive chat
    loop {
        let prompt = Text::new("Prompt:").prompt()?;
        let response = get_response(&mut chat, prompt)?; 
    
        termimad::print_text(&response);
    }
}
