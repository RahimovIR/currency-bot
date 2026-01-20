use async_trait::async_trait;
use std::error::Error;
use teloxide::prelude::*;

#[async_trait]
pub trait Module: Send + Sync {
    fn name(&self) -> &str;

    fn commands(&self) -> Vec<&str>;

    async fn handle(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct ModuleRegistry {
    modules: Vec<Box<dyn Module>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn register(&mut self, module: Box<dyn Module>) {
        log::info!("Registered module: {}", module.name());
        self.modules.push(module);
    }

    pub async fn handle_message(
        &self,
        bot: Bot,
        msg: Message,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(text) = msg.text() {
            for module in &self.modules {
                for cmd in module.commands() {
                    if text.starts_with(cmd) {
                        log::debug!("Module '{}' handling message", module.name());
                        return module.handle(bot, msg).await;
                    }
                }
            }

            log::debug!("No module found for command: {}", text);
            bot.send_message(msg.chat.id, "Неизвестная команда. Используйте /help")
                .await?;
        }
        Ok(())
    }
}

pub mod echo;
pub mod scheduler;
pub mod start;
pub mod subscribers;

pub use self::echo::EchoModule;
pub use self::start::StartModule;
pub use self::subscribers::{SubscriberManager, SubscriberModule};
