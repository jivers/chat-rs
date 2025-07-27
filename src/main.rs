use std::env;
use anyhow::Result;
use inquire::Text;
use chat::Chat;
pub mod chat;

fn get_response(chat: &mut Chat, prompt: String) -> Result<String> {
    let response = chat.send(&prompt)?;
    let message = response.first()?;
    Ok(message.content.clone())
}

fn main() -> Result<()> {
    let mut chat = Chat::new("gpt-4o"); 
    let prompt = env::args().collect::<Vec<String>>()[1..].join(" "); 
    if prompt.len() > 0 {
        let prompt_display = format!("
```
{}
```

", prompt);
        termimad::print_text(&prompt_display);

        let response = get_response(&mut chat, prompt)?;
        termimad::print_text(&response);
    }

    loop {
        let prompt = Text::new("Prompt:").prompt()?;
        let response = get_response(&mut chat, prompt)?; 
    
        termimad::print_text(&response);
    }
}
