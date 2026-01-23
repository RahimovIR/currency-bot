use dotenvy::dotenv;
use std::sync::Arc;
use teloxide::prelude::*;

mod modules;
use modules::scheduler::Scheduler;
use modules::{
    EchoModule, HelpModule, ModuleRegistry, StartModule, SubscriberManager, SubscriberModule,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting currency bot...");

    let subscription_interval_minutes = std::env::var("SUBSCRIPTION_INTERVAL_MINUTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let periodic_message_text = std::env::var("PERIODIC_MESSAGE_TEXT")
        .ok()
        .unwrap_or_else(|| "Периодическое сообщение от бота".to_string());

    let subscriber_manager = Arc::new(SubscriberManager::new(periodic_message_text.clone()));

    let mut registry = ModuleRegistry::new();
    registry.register(Box::new(StartModule::new()));
    registry.register(Box::new(EchoModule::new()));
    registry.register(Box::new(SubscriberModule::new(Arc::clone(
        &subscriber_manager,
    ))));
    registry.register(Box::new(HelpModule::new()));
    let registry = Arc::new(registry);

    let bot = Bot::from_env();

    let scheduler = Scheduler::new(
        Arc::clone(&subscriber_manager),
        subscription_interval_minutes,
    );
    let scheduler_bot = bot.clone();

    tokio::spawn(async move {
        scheduler.start(scheduler_bot).await;
    });

    log::info!("Scheduler started in background");

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let registry = Arc::clone(&registry);
        async move {
            if let Err(e) = registry.handle_message(bot, msg).await {
                log::error!("Error handling message: {}", e);
                Ok(())
            } else {
                Ok(())
            }
        }
    })
    .await;
}
