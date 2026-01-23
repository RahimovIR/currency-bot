use super::Module;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};
use teloxide::prelude::*;
use teloxide::types::MessageId;

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
    next_send_time: Arc<std::sync::Mutex<Option<Instant>>>,
    message_counters: Arc<std::sync::Mutex<HashMap<ChatId, u64>>>,
    message_ids: Arc<std::sync::Mutex<HashMap<ChatId, MessageId>>>,
    message_text: String,
}

impl SubscriberManager {
    pub fn new(message_text: String) -> Self {
        Self {
            subscribers: Arc::new(std::sync::Mutex::new(HashSet::new())),
            next_send_time: Arc::new(std::sync::Mutex::new(None)),
            message_counters: Arc::new(std::sync::Mutex::new(HashMap::new())),
            message_ids: Arc::new(std::sync::Mutex::new(HashMap::new())),
            message_text,
        }
    }

    pub fn subscribe(&self, chat_id: ChatId) -> SubscriptionAction {
        let mut subscribers = self.subscribers.lock().unwrap();
        if subscribers.insert(chat_id) {
            log::info!("User {} subscribed to periodic messages", chat_id);
            let mut counters = self.message_counters.lock().unwrap();
            counters.insert(chat_id, 0);
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
            let mut counters = self.message_counters.lock().unwrap();
            counters.remove(&chat_id);
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

    pub fn get_message_count(&self, chat_id: ChatId) -> u64 {
        let counters = self.message_counters.lock().unwrap();
        *counters.get(&chat_id).unwrap_or(&0)
    }

    pub fn increment_message_counter(&self, chat_id: ChatId) {
        let mut counters = self.message_counters.lock().unwrap();
        if let Some(counter) = counters.get_mut(&chat_id) {
            *counter += 1;
        }
    }

    pub async fn send_periodic_message_to_chat(
        &self,
        bot: &Bot,
        chat_id: ChatId,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let message_with_counter = self.format_periodic_message(chat_id);

        match self.get_message_id(chat_id) {
            Some(message_id) => {
                match bot
                    .edit_message_text(chat_id, message_id, &message_with_counter)
                    .await
                {
                    Ok(_) => {
                        self.increment_message_counter(chat_id);
                        Ok(true)
                    }
                    Err(e) => {
                        log::error!("Failed to edit message for {}: {}", chat_id, e);
                        Ok(false)
                    }
                }
            }
            None => {
                log::debug!(
                    "No message ID found for chat {}, skipping periodic message",
                    chat_id
                );
                Ok(false)
            }
        }
    }

    pub fn set_message_id(&self, chat_id: ChatId, message_id: MessageId) {
        let mut ids = self.message_ids.lock().unwrap();
        ids.insert(chat_id, message_id);
    }

    pub fn get_message_id(&self, chat_id: ChatId) -> Option<MessageId> {
        let ids = self.message_ids.lock().unwrap();
        ids.get(&chat_id).copied()
    }

    pub fn remove_message_id(&self, chat_id: ChatId) {
        let mut ids = self.message_ids.lock().unwrap();
        ids.remove(&chat_id);
    }

    pub fn set_next_send_time(&self, time: Instant) {
        let mut next = self.next_send_time.lock().unwrap();
        *next = Some(time);
    }

    pub fn get_time_until_next(&self) -> Option<Duration> {
        let next = self.next_send_time.lock().unwrap();
        next.map(|t| {
            let remaining = t.duration_since(Instant::now());
            if remaining.is_zero() {
                Duration::from_secs(0)
            } else {
                remaining
            }
        })
    }

    pub fn get_periodic_message_text(&self) -> String {
        self.message_text.clone()
    }

    pub fn format_periodic_message(&self, chat_id: ChatId) -> String {
        let current_count = self.get_message_count(chat_id);
        let message_text = self.get_periodic_message_text();
        format!(
            "Периодическое сообщение #{}:
{}",
            current_count + 1,
            message_text
        )
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
            let time_left = self.manager.get_time_until_next();
            let time_text = match time_left {
                Some(d) if d.as_secs() > 0 => {
                    let minutes = d.as_secs() / 60;
                    let seconds = d.as_secs() % 60;
                    format!("Следующее сообщение через {} мин {} сек", minutes, seconds)
                }
                Some(_) => "Сообщение будет отправлено скоро...".to_string(),
                None => "Информация о времени рассылки недоступна".to_string(),
            };
            format!(
                "Вы подписаны на рассылку.\n{}\nВсего подписчиков: {}",
                time_text,
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

                    if let SubscriptionAction::Subscribed = action {
                        let initial_message = self.manager.format_periodic_message(chat_id);
                        let message = bot.send_message(chat_id, &initial_message).await?;
                        self.manager.set_message_id(chat_id, message.id);
                        self.manager.increment_message_counter(chat_id);
                    }
                }
                "/unsubscribe" => {
                    let action = self.manager.unsubscribe(chat_id);
                    let response = match action {
                        SubscriptionAction::Unsubscribed => "Вы успешно отписались от рассылки.",
                        SubscriptionAction::NotSubscribed => "Вы не были подписаны на рассылку.",
                        _ => unreachable!(),
                    };
                    bot.send_message(chat_id, response).await?;
                    self.manager.remove_message_id(chat_id);
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
        let manager = SubscriberManager::new("Test message".to_string());
        let chat_id = ChatId(12345);
        assert_eq!(manager.subscribe(chat_id), SubscriptionAction::Subscribed);
        assert!(manager.is_subscribed(chat_id));
    }

    #[test]
    fn test_subscribe_already_subscribed() {
        let manager = SubscriberManager::new("Test message".to_string());
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
        let manager = SubscriberManager::new("Test message".to_string());
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
        let manager = SubscriberManager::new("Test message".to_string());
        let chat_id = ChatId(12345);
        assert_eq!(
            manager.unsubscribe(chat_id),
            SubscriptionAction::NotSubscribed
        );
    }

    #[test]
    fn test_get_subscribers() {
        let manager = SubscriberManager::new("Test message".to_string());
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
    fn test_individual_counters() {
        let manager = SubscriberManager::new("Test message".to_string());
        let chat_id1 = ChatId(111);
        let chat_id2 = ChatId(222);

        manager.subscribe(chat_id1);
        manager.subscribe(chat_id2);

        assert_eq!(manager.get_message_count(chat_id1), 0);
        assert_eq!(manager.get_message_count(chat_id2), 0);

        manager.increment_message_counter(chat_id1);
        assert_eq!(manager.get_message_count(chat_id1), 1);
        assert_eq!(manager.get_message_count(chat_id2), 0);

        manager.increment_message_counter(chat_id2);
        manager.increment_message_counter(chat_id2);
        assert_eq!(manager.get_message_count(chat_id1), 1);
        assert_eq!(manager.get_message_count(chat_id2), 2);
    }

    #[test]
    fn test_counter_removed_on_unsubscribe() {
        let manager = SubscriberManager::new("Test message".to_string());
        let chat_id = ChatId(12345);

        manager.subscribe(chat_id);
        manager.increment_message_counter(chat_id);
        assert_eq!(manager.get_message_count(chat_id), 1);

        manager.unsubscribe(chat_id);
        assert_eq!(manager.get_message_count(chat_id), 0);
    }

    #[test]
    fn test_message_id_management() {
        let manager = SubscriberManager::new("Test message".to_string());
        let chat_id = ChatId(12345);
        let message_id = MessageId(67890);

        manager.subscribe(chat_id);
        manager.set_message_id(chat_id, message_id);
        assert_eq!(manager.get_message_id(chat_id), Some(message_id));

        manager.remove_message_id(chat_id);
        assert_eq!(manager.get_message_id(chat_id), None);
    }

    #[test]
    fn test_message_counter() {
        let manager = Arc::new(SubscriberManager::new("Test message".to_string()));
        let chat_id = ChatId(12345);

        manager.subscribe(chat_id);
        assert_eq!(manager.get_message_count(chat_id), 0);

        manager.increment_message_counter(chat_id);
        assert_eq!(manager.get_message_count(chat_id), 1);

        manager.increment_message_counter(chat_id);
        assert_eq!(manager.get_message_count(chat_id), 2);
    }

    #[test]
    fn test_format_periodic_message() {
        let manager = Arc::new(SubscriberManager::new(
            "Периодическое сообщение от бота".to_string(),
        ));
        let chat_id = ChatId(12345);

        manager.subscribe(chat_id);
        let message = manager.format_periodic_message(chat_id);
        assert!(message.contains("Периодическое сообщение #1:"));
        assert!(message.contains("Периодическое сообщение от бота"));

        manager.increment_message_counter(chat_id);
        let message = manager.format_periodic_message(chat_id);
        assert!(message.contains("Периодическое сообщение #2:"));
    }

    #[test]
    fn test_module_name() {
        let manager = Arc::new(SubscriberManager::new("Test message".to_string()));
        let module = SubscriberModule::new(manager);
        assert_eq!(module.name(), "Subscriber");
    }

    #[test]
    fn test_module_commands() {
        let manager = Arc::new(SubscriberManager::new("Test message".to_string()));
        let module = SubscriberModule::new(manager);
        assert_eq!(
            module.commands(),
            vec!["/subscribe", "/unsubscribe", "/status"]
        );
    }
}
