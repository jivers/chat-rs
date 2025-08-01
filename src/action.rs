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

struct Exec;
impl Action for Exec {
    fn process(input: IO) -> Result<IO> {
        match input {
            IO::Command(command) => {
                let args = shell_words::split(&command)?;
                let mut cmd = Command::new(&args[0]);
                let status = cmd.args(&args[1..]).status()?;
                
                if !status.success() {
                    return Err(anyhow::anyhow!("failed to exec command: {}", command))
                }
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
    fn exec_ffmpeg() {
        let input_path = std::path::Path::new("test/content/test.wav");
        let output_path = std::path::Path::new("test/content/test.m4a");
        let command = format!("ffmpeg -i {} -c:a aac -b:a 192k {}", input_path.to_str().unwrap(), output_path.to_str().unwrap());
        let result = Exec::process(IO::Command(command));
        let _ = std::fs::remove_file(output_path);

        assert!(result.is_ok(), "ExecFfmpeg failed: {:?}", result);

    }

    #[test]
    fn exec_ffmpeg_failed() {
        let input_path = std::path::Path::new("test/content/test.wav");
        let output_path = std::path::Path::new("test/content/test.m4a");
        let command = format!("ffmpg i {} -ca ac -b:a 19k {}", input_path.to_str().unwrap(), output_path.to_str().unwrap());
        let result = Exec::process(IO::Command(command));

        assert!(!result.is_ok(), "ExecFfmpeg failed: {:?}", result);

    }
}

