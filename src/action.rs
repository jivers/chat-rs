// Actions can:
// - execute commands
// - validate commands
// - parse content

// Chain example:
// "remove the _dialogue from all the filenames in this directory"
// 1. Gather context by listing all files in the directory
// 2. Create a new Prompt with Context to generate a Command
// 3. Validate that the Command is safe to execute
// 4. Execute Command
use std::process::Command;
use anyhow::{Result, anyhow, Context};
use shell_words;
use crate::chat::Chat;

#[derive(Debug)]
enum IO {
    Prompt(String),
    Command(String),
    Context(String),
    None,
}

#[derive(Debug)]
enum Action {
    Draft,
    Validate,
    Exec,
}

impl Action {
    fn process(&self, input: IO) -> Result<IO> {
        match self {
            Action::Draft => draft(input),
            Action::Validate => validate(input),
            Action::Exec => exec(input),
            _ => Err(anyhow!("Unable to process action {:?} for input {:?}", self, input)),
        }
    }
}

fn draft(input: IO) -> Result<IO> {
    if let IO::Prompt(prompt) = input {
        let mut chat = Chat::new("o4-mini");
        chat.add_dev_message("You are a useful terminal agent that generates only single commands that are executable in a unix or linux terminal");

        let prompt = format!("Generate a single command, no explanation, to: {}", prompt);
        let response = chat.send(&prompt)?;
        let content = response.first()?.content;
        let split: Vec<_> = content.split("\n").collect();
        
        // might have ``` or might just be the command
        let command = match split.len() {
            1 => split.first(),
            _ => split.get(2),
        }.context("unable to extract command from Message content")?;

        return Ok(IO::Command(command.to_string()))

    } else {
        return Err(anyhow!("Draft only takes IO::Prompt"))
    } 
}

fn validate(input: IO) -> Result<IO> {
    if let IO::Command(command) = input {
        let mut chat = Chat::new("o4-mini");
        chat.add_dev_message("You are highly discerning agent who validates whether or not terminal commands are executable or not. You say 'true' if it's valid and 'false' if it's not");

        let prompt = format!("Is this a valid command: {}?", command);
        let response = chat.send(&prompt)?;
        let content = response.first()?.content;

        if content == "false" {
            return Err(anyhow!("Command is invalid"))
        }

        return Ok(IO::Command(command))
    } else {
        return Err(anyhow!("Validate only takse IO::Prompt"))
    }
}

fn exec(input: IO) -> Result<IO> {
    match input {
        IO::Command(command) => {
            let args = shell_words::split(&command)?;
            let mut cmd = Command::new(&args[0]);
            let results = cmd.args(&args[1..]).output()?;

            if !results.status.success() {
                return Err(anyhow!("failed to exec command: {}", command))
            }

            let context = String::from_utf8(results.stdout)?;
            return Ok(IO::Context(context.to_owned()))
        },
        _ => return Err(anyhow!("Exec only takes IO::Command"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_ffmpeg_command() {
        let prompt = IO::Prompt("convert test/content/test.wav to m4a using the same name".to_string());
        let command = Action::Draft.process(prompt);
        println!("{:?}", command);

        assert!(command.is_ok(), "Draft prompt failed");
    }

    #[test]
    fn exec_ffmpeg() {
        let input_path = std::path::Path::new("test/content/test.wav");
        let output_path = std::path::Path::new("test/content/test.m4a");
        let command = format!("ffmpeg -i {} -c:a aac -b:a 192k {}", input_path.to_str().unwrap(), output_path.to_str().unwrap());
        let result = Action::Exec.process(IO::Command(command));
        let _ = std::fs::remove_file(output_path);

        assert!(result.is_ok(), "ExecFfmpeg failed: {:?}", result);

    }

    #[test]
    fn exec_ffmpeg_failed() {
        let input_path = std::path::Path::new("test/content/test.wav");
        let output_path = std::path::Path::new("test/content/test.m4a");
        let command = format!("ffmpg i {} -ca ac -b:a 19k {}", input_path.to_str().unwrap(), output_path.to_str().unwrap());
        let result = Action::Exec.process(IO::Command(command));

        assert!(!result.is_ok(), "Exec failed: {:?}", result);
    }

    #[test]
    fn ffmpeg_chain() {
        let prompt = IO::Prompt("convert test/content/test.wav to m4a using the same name".to_string());
        let command = Action::Draft.process(prompt).unwrap();
        let result = Action::Exec.process(command);

        assert!(result.is_ok(), "Exec failed: {:?}", result);
    }

    #[test]
    fn invalid_ffmpeg_command() {
        let input_path = std::path::Path::new("test/content/test.wav");
        let output_path = std::path::Path::new("test/content/test.m4a");
        let command = format!("ffmpg i {} -ca ac -b:a 19k {}", input_path.to_str().unwrap(), output_path.to_str().unwrap());
        let validated_command = Action::Validate.process(IO::Command(command));

        assert!(!validated_command.is_ok(), "Validation failed: {:?}", validated_command);
    }
}

