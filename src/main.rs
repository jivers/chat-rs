use std::env;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use ureq;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    input: String,
}

impl ChatRequest {
    fn new(input: String) -> ChatRequest {
        ChatRequest {
            model: "gpt-4.1".to_string(),
            input
        }
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    id: String,
    output: Vec<Output>,
}

impl Response {
    fn text(&self) -> Result<String> {
        let output = self.output.get(0).context("no outputs")?;
        let content = output.content.get(0).context("no content")?; 

        Ok(content.text.clone())
    }
}

#[derive(Deserialize, Debug)]
struct Output {
    content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
struct Content {
    #[serde(rename = "type")]
    kind: String, // should be enum
    text: String,
}

fn main() -> Result<()> {
    let input = match env::args().nth(1) {
        Some(x) => x,
        None => "gimme the beat boy".to_string(),
    };
    let chat_request = ChatRequest::new(input);

    let openai_api_key = env::var("OPENAI_API_KEY")?;
    println!("{}", openai_api_key);
    let mut response = ureq::post("https://api.openai.com/v1/responses")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .header("Content-Type", "application/json")
        .send_json(chat_request)?;

    let response = response
        .body_mut()
        .read_json::<Response>()?;

    let text = response.text()?; 
    termimad::print_inline(&text);

    Ok(())
}
