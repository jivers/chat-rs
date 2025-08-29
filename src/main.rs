use std::collections::HashMap;
use clap::{Parser, Subcommand};
use anyhow::Result;
use inquire::Text;
use chat::{Chat};
use tool::ToolType;
use function::{Function, Parameters, Property, JsonType};
use tool::Tool;
use std::io::Write;
use std::fs;

pub mod chat;
pub mod action;
pub mod function;
pub mod tool;

#[derive(Parser)]
#[command(name = "chatrs", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    New,
    Last,
}

fn main() -> Result<()> {
    // create tool chooser
    let tool_chooser = Tool {
        r#type: ToolType::Function,
        function: Some(Function {
            name: "choose_tool".to_string(),
            description: "choose the best cli tool to get the job done".to_string(),
            parameters: Parameters {
                r#type: "object".to_string(),
                properties: HashMap::from([("tool".to_string(), Property {
                    r#type: JsonType::String,
                    description: "the cli tool to choose".to_string(),
                    r#enum: Some(vec!["ffmpeg".into(), "ls".into()]),
                })]),
                required: vec!["tool".into()],
            },
        }),
    };

    let cli = Cli::parse();
    let local_storage_dir = dirs::data_local_dir()
        .expect("unable to locate local app dir")
        .join("chatrs");

    if !local_storage_dir.exists() {
        fs::create_dir_all(dirs::data_local_dir()
            .expect("unable to locate local data dir")
            .join("chatrs"))?;
    }

    match cli.command {
        Commands::New => {
            let mut chat = Chat::new("gpt-5");
            chat.add_tool(tool_chooser);

            let chat_path = local_storage_dir.join(format!("{:?}.json", chrono::Local::now()));

            let mut log_file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(chat_path)?;


            loop {
                let prompt = Text::new("Prompt:").prompt()?;
                chat.complete(prompt)?;

                let chat_string = chat.get_messages_string()?;
                write!(&mut log_file, "{}", chat_string);
            }
        },
        Commands::Last => {
            for entry in fs::read_dir(&local_storage_dir)? {
                dbg!(entry);
            }
            Ok(())
        }
    }
}
