use crate::chat::Chat;
use anyhow::Result;

pub struct Assistant {
    actions: Vec<Action>,
}

impl Assistant {
    fn run(&mut self, prompt: &str) -> Result<String> {
        let mut acc = prompt.to_string();
        for action in self.actions.iter_mut() {
            acc = action.run(&acc)?;
        }
        Ok(acc)
    }
}

pub struct Action {
    /// Actions are essentially one-shot chat messages that serve a particular function
    /// They can be chained together to create an assistant
    pub chat: Chat,
    // TODO I can improve this to handle more transformations?
    pub post: Option<Box<dyn Fn(String) -> String + Send + Sync>>,
}

impl Action {
    pub fn run(&mut self, prompt: &str) -> Result<String> {
        let response = self.chat.send(prompt)?;
        let message = response.first()?;
        let mut output = message.content.clone();

        // don't love
        if let Some(ref post_fn) = self.post {
            output = post_fn(output);
        }

        Ok(output)
    }
}


