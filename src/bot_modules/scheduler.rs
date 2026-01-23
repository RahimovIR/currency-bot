use super::subscribers::SubscriberManager;
use std::sync::Arc;
use std::time::{Duration, Instant};
use teloxide::prelude::*;

pub struct Scheduler {
    subscribers: Arc<SubscriberManager>,
    interval: Duration,
}

impl Scheduler {
    pub fn new(subscribers: Arc<SubscriberManager>, interval_minutes: u64) -> Self {
        let interval = Duration::from_secs(interval_minutes * 60);
        log::info!(
            "Scheduler initialized with interval: {} minutes",
            interval.as_secs() / 60
        );
        Self {
            subscribers,
            interval,
        }
    }

    pub async fn start(&self, bot: Bot) {
        let mut interval_timer = tokio::time::interval(self.interval);

        loop {
            let next_send = Instant::now() + self.interval;
            self.subscribers.set_next_send_time(next_send);

            interval_timer.tick().await;
            self.send_periodic_message(&bot).await;
        }
    }

    async fn send_periodic_message(&self, bot: &Bot) {
        let subscribers = self.subscribers.get_subscribers();
        let count = subscribers.len();

        if count == 0 {
            log::debug!("No subscribers to send message to");
            return;
        }

        log::info!("Sending periodic message to {} subscribers", count);

        let mut success_count = 0;
        let mut error_count = 0;

        for chat_id in subscribers {
            match self
                .subscribers
                .send_periodic_message_to_chat(bot, chat_id)
                .await
            {
                Ok(true) => success_count += 1,
                Ok(false) => error_count += 1,
                Err(e) => {
                    log::error!("Unexpected error for {}: {}", chat_id, e);
                    error_count += 1;
                }
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        log::info!(
            "Periodic message sent: {} success, {} errors",
            success_count,
            error_count
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time;

    #[tokio::test]
    async fn test_scheduler_creation() {
        let manager = Arc::new(SubscriberManager::new("Test message".to_string()));
        let scheduler = Scheduler::new(Arc::clone(&manager), 10);
        let expected_interval = scheduler.interval;
        assert_eq!(expected_interval, Duration::from_secs(10 * 60));
    }

    #[tokio::test]
    async fn test_scheduler_no_subscribers() {
        std::env::set_var(
            "TELOXIDE_TOKEN",
            "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11",
        );

        let manager = Arc::new(SubscriberManager::new("Test message".to_string()));
        let scheduler = Scheduler::new(manager.clone(), 1);

        let bot = Bot::from_env();
        let mut interval = time::interval(Duration::from_millis(100));

        tokio::spawn(async move {
            scheduler.start(bot).await;
        });

        interval.tick().await;
        interval.tick().await;

        std::env::remove_var("TELOXIDE_TOKEN");
    }

    #[test]
    fn test_message_counter_basic() {
        let manager = Arc::new(SubscriberManager::new("Test message".to_string()));
        let chat_id = ChatId(12345);
        manager.subscribe(chat_id);
        assert_eq!(manager.get_message_count(chat_id), 0);
        manager.increment_message_counter(chat_id);
        assert_eq!(manager.get_message_count(chat_id), 1);
    }

    #[test]
    fn test_scheduler_default_values() {
        let manager = Arc::new(SubscriberManager::new(
            "Периодическое сообщение от бота".to_string(),
        ));
        let scheduler = Scheduler::new(Arc::clone(&manager), 10);
        let expected_interval = scheduler.interval;
        let expected_message = manager.get_periodic_message_text();

        assert_eq!(expected_interval, Duration::from_secs(10 * 60));
        assert_eq!(expected_message, "Периодическое сообщение от бота");
    }

    #[test]
    fn test_scheduler_custom_values() {
        let manager = Arc::new(SubscriberManager::new("Custom message".to_string()));
        let scheduler = Scheduler::new(Arc::clone(&manager), 5);
        let interval = scheduler.interval;
        let message_text = manager.get_periodic_message_text();

        assert_eq!(interval, Duration::from_secs(5 * 60));
        assert_eq!(message_text, "Custom message");
    }
}
