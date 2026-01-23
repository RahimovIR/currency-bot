use crate::domain::{CurrencyPair, PriceData, PriceProviderError};
use async_trait::async_trait;

/// Trait for price providers
#[async_trait]
pub trait PriceProvider: Send + Sync {
    /// Get the name of the provider
    fn name(&self) -> &str;

    /// Fetch price data for a domain currency pair
    async fn fetch_price(&self, pair: &CurrencyPair) -> Result<PriceData, PriceProviderError>;

    /// Check if this provider supports the given currency pair
    fn supports_currency_pair(&self, pair: &CurrencyPair) -> bool;
}
