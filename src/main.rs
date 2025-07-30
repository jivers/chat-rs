use std::env;
use anyhow::Result;
use inquire::Text;
use chat::{Chat};
use atty::Stream;
use std::io::{self, Read};
use termimad::print_text;

pub mod chat;
pub mod assistant;

fn respond(chat: &mut Chat, prompt: String) -> Result<()> {
    let response = chat.send(&prompt)?
        .first()?
        .content
        .clone();

    print_text(&response);
    Ok(())
}

fn main() -> Result<()> {
    let mut chat = Chat::new("gpt-4o");

    let args: Vec<String> = env::args().skip(1).collect();
    let is_pipe = !atty::is(Stream::Stdin);
    let has_args = args.len() > 0; 

    match (is_pipe, has_args) {
        (true, true) => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;

            let prompt = format!("{} \n --- \n {}", buffer, args.join(" "));
            return respond(&mut chat, prompt)
        },
        (true, false) => {
            let mut prompt = String::new();
            io::stdin().read_to_string(&mut prompt)?;

            return respond(&mut chat, prompt)
        },
        (false, true) => {
            let prompt = args.join(" ");
            return respond(&mut chat, prompt)
        },
        _ => {
            loop {
                let prompt = Text::new("Prompt:").prompt()?;
                return respond(&mut chat, prompt)
            }
        }
    }
}
