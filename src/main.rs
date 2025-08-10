use std::env;
use std::collections::HashMap;
use anyhow::Result;
use inquire::Text;
use chat::{Chat};
use atty::Stream;
use tool::ToolType;
use std::io::{self, Read};
use termimad::print_text;
use function::{Function, Parameters, Property, JsonType};
use tool::Tool;

pub mod chat;
pub mod action;
pub mod function;
pub mod tool;

fn main() -> Result<()> {
    let mut props = HashMap::new();
    props.insert("tool".to_string(), Property {
        r#type: JsonType::String,
        description: "the cli tool to choose".to_string(),
        r#enum: Some(vec!["ffmpeg".into(), "ls".into()]),
    });

    let tool_choose = Tool {
        r#type: ToolType::Function,
        function: Some(Function {
            name: "choose_tool".to_string(),
            description: "choose the best cli tool to get the job done".to_string(),
            parameters: Parameters {
                r#type: "object".to_string(),
                properties: props,
                required: vec!["tools".into()],
            },
        }),
    };

    let mut chat = Chat::new("gpt-4o");
    chat.add_tool(tool_choose);

    let args: Vec<String> = env::args().skip(1).collect();
    let is_pipe = !atty::is(Stream::Stdin);
    let has_args = args.len() > 0; 

    // todo: this could get cleaned up
    match (is_pipe, has_args) {
        // pipe with prompt
        (true, true) => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;

            let prompt = format!("{} \n --- \n {}", buffer, args.join(" "));
            return chat.complete(prompt)
        },
        // pipe no prompt
        (true, false) => {
            let mut prompt = String::new();
            io::stdin().read_to_string(&mut prompt)?;

            return chat.complete(prompt)
        },
        // just prompt (as args)
        (false, true) => {
            let prompt = args.join(" ");
            return chat.complete(prompt)
        },
        // interactive chat mode
        _ => {
            loop {
                let prompt = Text::new("Prompt:").prompt()?;
                chat.complete(prompt)?;
            }
        }
    }
}
