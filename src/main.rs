use dotenvy::dotenv;
use std::sync::Arc;
use teloxide::prelude::*;

mod modules;
use modules::scheduler::Scheduler;
use modules::{EchoModule, ModuleRegistry, StartModule, SubscriberManager, SubscriberModule};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting currency bot...");

    let subscriber_manager = Arc::new(SubscriberManager::new());

    let mut registry = ModuleRegistry::new();
    registry.register(Box::new(StartModule::new()));
    registry.register(Box::new(EchoModule::new()));
    registry.register(Box::new(SubscriberModule::new(Arc::clone(
        &subscriber_manager,
    ))));
    let registry = Arc::new(registry);

    let bot = Bot::from_env();

    let scheduler = Scheduler::new(Arc::clone(&subscriber_manager));
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
