use std::sync::Arc;
use teloxide::prelude::*;

mod bot_modules;
mod domain;
mod price_service;
use bot_modules::scheduler::Scheduler;
use bot_modules::{
    EchoModule, HelpModule, ModuleRegistry, NewLineModule, PriceModule, StartModule,
    SubscriberManager, SubscriberModule,
};
use price_service::{
    providers::{NewLineConfig, NewLineProvider},
    PriceService,
};

#[tokio::main]
async fn main() {
    // Try to load .env file, but don't fail if it's not present
    let _ = dotenvy::dotenv();

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

    // Initialize price service
    let mut price_service = PriceService::new();

    // Configure NewLine provider from environment
    let newline_base_url = std::env::var("NEWLINE_API_BASE_URL")
        .unwrap_or_else(|_| "https://newline.online".to_string());
    let newline_cookie = std::env::var("NEWLINE_COOKIE")
        .unwrap_or_else(|_| panic!("NEWLINE_COOKIE environment variable is required but not set"));
    let newline_preferred_city =
        std::env::var("NEWLINE_PREFERRED_CITY").unwrap_or_else(|_| "spb".to_string());

    let newline_config = NewLineConfig {
        base_url: newline_base_url,
        cookie: newline_cookie,
        preferred_city: newline_preferred_city,
    };

    let newline_provider = Arc::new(NewLineProvider::new(newline_config));
    price_service.add_provider(newline_provider);
    let price_service = Arc::new(price_service);

    let mut registry = ModuleRegistry::new();
    registry.register(Box::new(StartModule::new()));
    registry.register(Box::new(EchoModule::new()));
    registry.register(Box::new(PriceModule::new(Arc::clone(&price_service))));
    registry.register(Box::new(NewLineModule::new(Arc::clone(&price_service))));
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
