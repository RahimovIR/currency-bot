use crate::{domain::CurrencyPair, domain::PriceProviderError, price_service::PriceService};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use teloxide::prelude::*;

/// Price module for handling price-related commands
pub struct PriceModule {
    price_service: Arc<PriceService>,
}

impl PriceModule {
    /// Create a new PriceModule instance
    pub fn new(price_service: Arc<PriceService>) -> Self {
        Self { price_service }
    }
}

#[async_trait]
impl super::Module for PriceModule {
    fn name(&self) -> &str {
        "PriceModule"
    }

    fn commands(&self) -> Vec<&str> {
        vec!["/price"]
    }

    async fn handle(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(text) = msg.text() {
            let parts: Vec<&str> = text.split_whitespace().collect();

            if parts.len() == 2 && parts[0] == "/price" {
                let pair_input = parts[1];

                // Try to parse as domain currency pair
                if let Some(pair) = CurrencyPair::from_str(pair_input) {
                    // Use the new interface that works directly with currency pairs
                    match self.price_service.get_price(&pair).await {
                        Ok(price_data) => {
                            let response = format!(
                                "💰 {} Price\n\nCurrency Pair: {}\nPrice: {:.2}",
                                pair.to_string(),
                                price_data.pair.to_string(),
                                price_data.price
                            );
                            bot.send_message(msg.chat.id, response).await?;
                        }
                        Err(e) => {
                            let error_msg = match e {
                                PriceProviderError::NetworkError(msg) => {
                                    format!("🌐 Network error: {}", msg)
                                }
                                PriceProviderError::ApiError(msg) => {
                                    format!("🔌 API error: {}", msg)
                                }
                                PriceProviderError::ParsingError(msg) => {
                                    format!("📜 Parsing error: {}", msg)
                                }
                                PriceProviderError::ProviderError(msg) => {
                                    format!("❌ Provider error: {}", msg)
                                }
                            };
                            bot.send_message(msg.chat.id, error_msg).await?;
                        }
                    }
                } else {
                    bot.send_message(
                                msg.chat.id,
                                "❌ Invalid currency pair format. Available pairs: USD/RUB, USDCe/RUB, USDTe/RUB",
                            )
                    .await?;
                }
            } else {
                let available_pairs = crate::domain::get_all_currency_pairs();
                let pairs_list = available_pairs
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "Usage: /price CURRENCY_PAIR\nAvailable pairs: {}\nExample: /price USD/RUB",
                        pairs_list
                    ),
                )
                .await?;
            }
        }

        Ok(())
    }
}
