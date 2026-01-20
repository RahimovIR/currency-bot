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
        vec!["/echo", "/start"]
    }

    async fn handle(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(text) = msg.text() {
            if text.starts_with("/start") {
                bot.send_message(
                    msg.chat.id,
                    "Добро пожаловать в Currency Bot!\n\
                     Используйте /echo <текст> для эхо-ответа.",
                )
                .await?;
            } else if text.starts_with("/echo") {
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
