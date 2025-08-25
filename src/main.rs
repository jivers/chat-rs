use std::collections::HashMap;
use anyhow::Result;
use inquire::Text;
use chat::{Chat};
use tool::ToolType;
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

    let tool_chooser = Tool {
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
    chat.add_tool(tool_chooser);

    loop {
        let prompt = Text::new("Prompt:").prompt()?;
        chat.complete(prompt)?;
    }
}
