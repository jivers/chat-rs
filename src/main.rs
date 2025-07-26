use std::env;
use anyhow::Result;
use chat::Chat;
pub mod chat;

fn main() -> Result<()> {
    let input = match env::args().nth(1) {
        Some(x) => x,
        None => "gimme the beat boy".to_string(),
    };

    let chat = Chat::new("gpt-4o"); 
    let response = chat.send(&input)?;
    let content = response.first()?;
    termimad::print_inline(&content);

    Ok(())
}
