use super::Module;
use async_trait::async_trait;
use std::error::Error;
use teloxide::prelude::*;

pub struct HelpModule;

impl HelpModule {
    pub fn new() -> Self {
        Self
    }

    fn get_help_text() -> String {
        "
Доступные команды:

/start - Начать работу с ботом
/echo <текст> - Отправить эхо-ответ
/subscribe - Подписаться на периодические сообщения
/unsubscribe - Отписаться от периодических сообщений
/status - Проверить статус подписки
/help - Показать эту справку

Используйте /help для получения информации о доступных командах.
"
        .trim()
        .to_string()
    }
}

#[async_trait]
impl Module for HelpModule {
    fn name(&self) -> &str {
        "Help"
    }

    fn commands(&self) -> Vec<&str> {
        vec!["/help"]
    }

    async fn handle(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        bot.send_message(msg.chat.id, Self::get_help_text()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_name() {
        let module = HelpModule::new();
        assert_eq!(module.name(), "Help");
    }

    #[test]
    fn test_module_commands() {
        let module = HelpModule::new();
        assert_eq!(module.commands(), vec!["/help"]);
    }

    #[test]
    fn test_help_text() {
        let help_text = HelpModule::get_help_text();
        assert!(help_text.contains("/start"));
        assert!(help_text.contains("/echo"));
        assert!(help_text.contains("/subscribe"));
        assert!(help_text.contains("/unsubscribe"));
        assert!(help_text.contains("/status"));
        assert!(help_text.contains("/help"));
    }
}
