use dotenvy::dotenv;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file if present
    dotenv().ok();

    // Initialize logger (optional but useful)
    pretty_env_logger::init();
    log::info!("Starting simple echo bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // Echo back the received text, if any
        if let Some(txt) = msg.text() {
            bot.send_message(msg.chat.id, txt).await?;
        }
        Ok(())
    })
    .await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_math() {
        assert_eq!(1 + 1, 2);
    }
}
