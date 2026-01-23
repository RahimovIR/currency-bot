use crate::domain::{CurrencyPair, PriceData, PriceProviderError};
use crate::price_service::provider::PriceProvider;
use std::sync::Arc;

/// Main price service that manages multiple providers
pub struct PriceService {
    providers: Vec<Arc<dyn PriceProvider>>,
}

impl PriceService {
    /// Create a new PriceService instance
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    /// Add a price provider to the service
    pub fn add_provider(&mut self, provider: Arc<dyn PriceProvider>) {
        log::info!("Added price provider: {}", provider.name());
        self.providers.push(provider);
    }

    /// Get price from the first available provider that supports the currency pair
    pub async fn get_price(&self, pair: &CurrencyPair) -> Result<PriceData, PriceProviderError> {
        let mut errors = Vec::new();

        for provider in &self.providers {
            if provider.supports_currency_pair(pair) {
                match provider.fetch_price(pair).await {
                    Ok(price) => return Ok(price),
                    Err(e) => {
                        log::warn!(
                            "Provider {} failed for {}: {}",
                            provider.name(),
                            pair.to_string(),
                            e
                        );
                        errors.push(e);
                    }
                }
            }
        }

        Err(PriceProviderError::ProviderError(format!(
            "All providers failed to fetch price for {}: {:?}",
            pair.to_string(),
            errors
        )))
    }
}
