use crate::{domain::CurrencyPair, domain::PriceProviderError, price_service::PriceService};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use teloxide::prelude::*;

/// NewLine module for handling NewLine-specific commands
pub struct NewLineModule {
    price_service: Arc<PriceService>,
}

impl NewLineModule {
    /// Create a new NewLineModule instance
    pub fn new(price_service: Arc<PriceService>) -> Self {
        Self { price_service }
    }
}

#[async_trait]
impl super::Module for NewLineModule {
    fn name(&self) -> &str {
        "NewLineModule"
    }

    fn commands(&self) -> Vec<&str> {
        vec!["/newLine"]
    }

    async fn handle(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        let pair = CurrencyPair::USDTeRUB;

        match self.price_service.get_price(&pair).await {
            Ok(price_data) => {
                let response = format!(
                    "💰 {} Price\n\nCurrency Pair: {}\nPrice: {:.2}",
                    pair, price_data.pair, price_data.price
                );
                bot.send_message(msg.chat.id, response).await?;
            }
            Err(e) => {
                let error_msg = match e {
                    PriceProviderError::Network(msg) => format!("🌐 Network error: {}", msg),
                    PriceProviderError::Api(msg) => format!("🔌 API error: {}", msg),
                    PriceProviderError::Parsing(msg) => format!("📜 Parsing error: {}", msg),
                    PriceProviderError::Provider(msg) => format!("❌ Provider error: {}", msg),
                };
                bot.send_message(msg.chat.id, error_msg).await?;
            }
        }

        Ok(())
    }
}
