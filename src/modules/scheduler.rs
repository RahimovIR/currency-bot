use super::subscribers::SubscriberManager;
use std::sync::Arc;
use std::time::{Duration, Instant};
use teloxide::prelude::*;

const DEFAULT_INTERVAL_MINUTES: u64 = 10;
const DEFAULT_MESSAGE: &str = "Периодическое сообщение от бота";

pub struct Scheduler {
    interval: Duration,
    message_text: String,
    subscribers: Arc<SubscriberManager>,
}

impl Scheduler {
    pub fn new(subscribers: Arc<SubscriberManager>) -> Self {
        let interval_minutes = std::env::var("SUBSCRIPTION_INTERVAL_MINUTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_INTERVAL_MINUTES);

        let message_text = std::env::var("PERIODIC_MESSAGE_TEXT")
            .ok()
            .unwrap_or_else(|| DEFAULT_MESSAGE.to_string());

        Self {
            interval: Duration::from_secs(interval_minutes * 60),
            message_text,
            subscribers,
        }
    }

    #[cfg(test)]
    pub fn with_config(
        subscribers: Arc<SubscriberManager>,
        interval: Duration,
        message: &str,
    ) -> Self {
        Self {
            interval,
            message_text: message.to_string(),
            subscribers,
        }
    }

    pub async fn start(&self, bot: Bot) {
        log::info!(
            "Scheduler started with interval: {} minutes",
            self.interval.as_secs() / 60
        );
        log::info!("Message: {}", self.message_text);

        let mut interval = tokio::time::interval(self.interval);

        loop {
            let next_send = Instant::now() + self.interval;
            self.subscribers.set_next_send_time(next_send);

            interval.tick().await;
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
            match bot.send_message(chat_id, &self.message_text).await {
                Ok(_) => {
                    success_count += 1;
                }
                Err(e) => {
                    log::error!("Failed to send message to {}: {}", chat_id, e);
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
        let manager = Arc::new(SubscriberManager::new());
        let scheduler = Scheduler::new(manager);
        assert_eq!(
            scheduler.interval,
            Duration::from_secs(DEFAULT_INTERVAL_MINUTES * 60)
        );
    }

    #[tokio::test]
    async fn test_scheduler_no_subscribers() {
        std::env::set_var(
            "TELOXIDE_TOKEN",
            "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11",
        );

        let manager = Arc::new(SubscriberManager::new());
        let scheduler = Scheduler::with_config(manager, Duration::from_millis(100), "Test message");

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
    fn test_scheduler_default_values() {
        std::env::remove_var("SUBSCRIPTION_INTERVAL_MINUTES");
        std::env::remove_var("PERIODIC_MESSAGE_TEXT");

        let manager = Arc::new(SubscriberManager::new());
        let scheduler = Scheduler::new(manager);

        assert_eq!(
            scheduler.interval,
            Duration::from_secs(DEFAULT_INTERVAL_MINUTES * 60)
        );
        assert_eq!(scheduler.message_text, DEFAULT_MESSAGE);
    }

    #[test]
    fn test_scheduler_custom_values() {
        std::env::set_var("SUBSCRIPTION_INTERVAL_MINUTES", "5");
        std::env::set_var("PERIODIC_MESSAGE_TEXT", "Custom message");

        let manager = Arc::new(SubscriberManager::new());
        let scheduler = Scheduler::new(manager);

        assert_eq!(scheduler.interval, Duration::from_secs(5 * 60));
        assert_eq!(scheduler.message_text, "Custom message");

        std::env::remove_var("SUBSCRIPTION_INTERVAL_MINUTES");
        std::env::remove_var("PERIODIC_MESSAGE_TEXT");
    }
}
