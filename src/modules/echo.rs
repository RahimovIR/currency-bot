use super::Module;
use async_trait::async_trait;
use std::error::Error;
use teloxide::prelude::*;

pub struct EchoModule;

impl EchoModule {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Module for EchoModule {
    fn name(&self) -> &str {
        "Echo"
    }

    fn commands(&self) -> Vec<&str> {
        vec!["/echo"]
    }

    async fn handle(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(text) = msg.text() {
            if text.starts_with("/echo") {
                let echo_text = text.trim_start_matches("/echo").trim();
                if echo_text.is_empty() {
                    bot.send_message(msg.chat.id, "Использование: /echo <текст>")
                        .await?;
                } else {
                    bot.send_message(msg.chat.id, echo_text).await?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_name() {
        let module = EchoModule::new();
        assert_eq!(module.name(), "Echo");
    }

    #[test]
    fn test_module_commands() {
        let module = EchoModule::new();
        assert_eq!(module.commands(), vec!["/echo"]);
    }
}
