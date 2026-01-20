use super::Module;
use async_trait::async_trait;
use std::collections::HashSet;
use std::error::Error;
use std::sync::Arc;
use teloxide::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionAction {
    Subscribed,
    Unsubscribed,
    AlreadySubscribed,
    NotSubscribed,
}

#[derive(Debug, Clone)]
pub struct SubscriberManager {
    subscribers: Arc<std::sync::Mutex<HashSet<ChatId>>>,
}

impl SubscriberManager {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(std::sync::Mutex::new(HashSet::new())),
        }
    }

    pub fn subscribe(&self, chat_id: ChatId) -> SubscriptionAction {
        let mut subscribers = self.subscribers.lock().unwrap();
        if subscribers.insert(chat_id) {
            log::info!("User {} subscribed to periodic messages", chat_id);
            SubscriptionAction::Subscribed
        } else {
            log::debug!("User {} already subscribed", chat_id);
            SubscriptionAction::AlreadySubscribed
        }
    }

    pub fn unsubscribe(&self, chat_id: ChatId) -> SubscriptionAction {
        let mut subscribers = self.subscribers.lock().unwrap();
        if subscribers.remove(&chat_id) {
            log::info!("User {} unsubscribed from periodic messages", chat_id);
            SubscriptionAction::Unsubscribed
        } else {
            log::debug!("User {} was not subscribed", chat_id);
            SubscriptionAction::NotSubscribed
        }
    }

    pub fn is_subscribed(&self, chat_id: ChatId) -> bool {
        let subscribers = self.subscribers.lock().unwrap();
        subscribers.contains(&chat_id)
    }

    pub fn get_subscribers(&self) -> Vec<ChatId> {
        let subscribers = self.subscribers.lock().unwrap();
        subscribers.iter().cloned().collect()
    }

    pub fn subscriber_count(&self) -> usize {
        let subscribers = self.subscribers.lock().unwrap();
        subscribers.len()
    }
}

pub struct SubscriberModule {
    manager: Arc<SubscriberManager>,
}

impl SubscriberModule {
    pub fn new(manager: Arc<SubscriberManager>) -> Self {
        Self { manager }
    }

    fn format_status(&self, chat_id: ChatId) -> String {
        if self.manager.is_subscribed(chat_id) {
            format!(
                "Вы подписаны на рассылку. Всего подписчиков: {}",
                self.manager.subscriber_count()
            )
        } else {
            "Вы не подписаны на рассылку.".to_string()
        }
    }
}

#[async_trait]
impl Module for SubscriberModule {
    fn name(&self) -> &str {
        "Subscriber"
    }

    fn commands(&self) -> Vec<&str> {
        vec!["/subscribe", "/unsubscribe", "/status"]
    }

    async fn handle(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        let chat_id = msg.chat.id;

        if let Some(text) = msg.text() {
            match text {
                "/subscribe" => {
                    let action = self.manager.subscribe(chat_id);
                    let response = match action {
                        SubscriptionAction::Subscribed => "Вы успешно подписались на рассылку!",
                        SubscriptionAction::AlreadySubscribed => "Вы уже подписаны на рассылку.",
                        _ => unreachable!(),
                    };
                    bot.send_message(chat_id, response).await?;
                }
                "/unsubscribe" => {
                    let action = self.manager.unsubscribe(chat_id);
                    let response = match action {
                        SubscriptionAction::Unsubscribed => "Вы успешно отписались от рассылки.",
                        SubscriptionAction::NotSubscribed => "Вы не были подписаны на рассылку.",
                        _ => unreachable!(),
                    };
                    bot.send_message(chat_id, response).await?;
                }
                "/status" => {
                    let status = self.format_status(chat_id);
                    bot.send_message(chat_id, status).await?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teloxide::types::ChatId;

    #[test]
    fn test_subscribe_new_user() {
        let manager = SubscriberManager::new();
        let chat_id = ChatId(12345);
        assert_eq!(manager.subscribe(chat_id), SubscriptionAction::Subscribed);
        assert!(manager.is_subscribed(chat_id));
    }

    #[test]
    fn test_subscribe_already_subscribed() {
        let manager = SubscriberManager::new();
        let chat_id = ChatId(12345);
        manager.subscribe(chat_id);
        assert_eq!(
            manager.subscribe(chat_id),
            SubscriptionAction::AlreadySubscribed
        );
        assert_eq!(manager.subscriber_count(), 1);
    }

    #[test]
    fn test_unsubscribe() {
        let manager = SubscriberManager::new();
        let chat_id = ChatId(12345);
        manager.subscribe(chat_id);
        assert_eq!(
            manager.unsubscribe(chat_id),
            SubscriptionAction::Unsubscribed
        );
        assert!(!manager.is_subscribed(chat_id));
    }

    #[test]
    fn test_unsubscribe_not_subscribed() {
        let manager = SubscriberManager::new();
        let chat_id = ChatId(12345);
        assert_eq!(
            manager.unsubscribe(chat_id),
            SubscriptionAction::NotSubscribed
        );
    }

    #[test]
    fn test_get_subscribers() {
        let manager = SubscriberManager::new();
        let chat_id1 = ChatId(111);
        let chat_id2 = ChatId(222);
        manager.subscribe(chat_id1);
        manager.subscribe(chat_id2);
        let subscribers = manager.get_subscribers();
        assert_eq!(subscribers.len(), 2);
        assert!(subscribers.contains(&chat_id1));
        assert!(subscribers.contains(&chat_id2));
    }

    #[test]
    fn test_module_name() {
        let manager = Arc::new(SubscriberManager::new());
        let module = SubscriberModule::new(manager);
        assert_eq!(module.name(), "Subscriber");
    }

    #[test]
    fn test_module_commands() {
        let manager = Arc::new(SubscriberManager::new());
        let module = SubscriberModule::new(manager);
        assert_eq!(
            module.commands(),
            vec!["/subscribe", "/unsubscribe", "/status"]
        );
    }
}
