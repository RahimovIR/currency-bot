use super::Module;
use async_trait::async_trait;
use std::error::Error;
use teloxide::prelude::*;

const GREETING: &str =
    "Добро пожаловать в Currency Bot!\nИспользуйте /echo <текст> для эхо-ответа.";

pub struct StartModule;

impl StartModule {
    pub fn new() -> Self {
        Self
    }

    pub fn greeting() -> &'static str {
        GREETING
    }
}

#[async_trait]
impl Module for StartModule {
    fn name(&self) -> &str {
        "Start"
    }

    fn commands(&self) -> Vec<&str> {
        vec!["/start"]
    }

    async fn handle(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        bot.send_message(msg.chat.id, Self::greeting()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_name() {
        let module = StartModule::new();
        assert_eq!(module.name(), "Start");
    }

    #[test]
    fn test_module_commands() {
        let module = StartModule::new();
        assert_eq!(module.commands(), vec!["/start"]);
    }

    #[test]
    fn test_greeting() {
        assert_eq!(
            StartModule::greeting(),
            "Добро пожаловать в Currency Bot!\nИспользуйте /echo <текст> для эхо-ответа."
        );
    }
}
