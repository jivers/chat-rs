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
use anyhow::Result;
use shell_words;

#[derive(Debug)]
enum IO {
    Prompt(String),
    Command(String),
    Context(String),
    None,
}

trait Action {
    fn process(input: IO) -> Result<IO>;
}

struct Ls;
impl Action for Ls {
    fn process(_input: IO) -> Result<IO> {
        let output = Command::new("ls")
            .arg("-l")
            .output()?;

        let ls_context = String::from_utf8(output.stdout)?;
        Ok(IO::Context(ls_context))
    }
}

struct DraftFfmpeg;

struct ValidateFfmpeg;  

struct ExecFfmpeg;
impl Action for ExecFfmpeg {
    fn process(input: IO) -> Result<IO> {
        match input {
            IO::Command(command) => {
                let args = shell_words::split(&command)?;
                let mut cmd = Command::new("ffmpeg");
                cmd.args(&args[1..]);
                let _ = cmd.output();

                // TODO print results
                // handle error
            },
            _ => return Err(anyhow::anyhow!("ExecFfmpeg only takes IO::Command"))
        }

        Ok(IO::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ls() {
        let input = IO::Prompt("List all files in the current directory".to_string());
        if let Ok(ls_value) = Ls::process(input) {
            println!("{:?}", ls_value);
        };
    }

    #[test]
    fn exec_ffmpeg() {
        let result = ExecFfmpeg::process(IO::Command("ffmpeg -i /Users/jivers/Dev/chat-rs/test/content/test.wav -c:a aac -b:a 192k /Users/jivers/Dev/chat-rs/test/content/test.m4a".to_string()));
        assert!(result.is_ok(), "ExecFfmpeg failed: {:?}", result);

    }
}

