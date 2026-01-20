use dotenvy::dotenv;
use std::sync::Arc;
use teloxide::prelude::*;

mod modules;
use modules::{EchoModule, ModuleRegistry, StartModule};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting currency bot...");

    let mut registry = ModuleRegistry::new();
    registry.register(Box::new(StartModule::new()));
    registry.register(Box::new(EchoModule::new()));
    let registry = Arc::new(registry);

    let bot = Bot::from_env();

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
