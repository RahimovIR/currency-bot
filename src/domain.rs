// Domain module for currency pairs
// Defines the domain-specific currency pairs used by the application

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Price data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub pair: CurrencyPair,
    pub price: f64,
}

/// Error type for price providers
#[derive(Debug, Error)]
pub enum PriceProviderError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),

    #[error("Provider-specific error: {0}")]
    ProviderError(String),
}

/// Domain currency pairs used in the application
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurrencyPair {
    /// USDC (ERC20) to RUB
    USDCeRUB,
    /// USDT (ERC20) to RUB  
    USDTeRUB,
    /// USD to RUB
    USDRUB,
}

impl CurrencyPair {
    /// Convert currency pair to string representation
    pub fn to_string(&self) -> String {
        match self {
            CurrencyPair::USDCeRUB => "USDCe/RUB".to_string(),
            CurrencyPair::USDTeRUB => "USDTe/RUB".to_string(),
            CurrencyPair::USDRUB => "USD/RUB".to_string(),
        }
    }

    /// Parse string to CurrencyPair
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "USDCe/RUB" => Some(CurrencyPair::USDCeRUB),
            "USDTe/RUB" => Some(CurrencyPair::USDTeRUB),
            "USD/RUB" => Some(CurrencyPair::USDRUB),
            _ => None,
        }
    }
}

/// Get all available domain currency pairs
pub fn get_all_currency_pairs() -> Vec<CurrencyPair> {
    vec![
        CurrencyPair::USDCeRUB,
        CurrencyPair::USDTeRUB,
        CurrencyPair::USDRUB,
    ]
}

#[cfg(test)]
mod tests {
    use super::{get_all_currency_pairs, CurrencyPair};
    use crate::price_service::provider::PriceProvider;
    use crate::price_service::providers::newline_provider::{NewLineConfig, NewLineProvider};

    #[test]
    fn test_currency_pair_parsing() {
        assert_eq!(
            CurrencyPair::from_str("USD/RUB"),
            Some(CurrencyPair::USDRUB)
        );
        assert_eq!(
            CurrencyPair::from_str("USDCe/RUB"),
            Some(CurrencyPair::USDCeRUB)
        );
        assert_eq!(
            CurrencyPair::from_str("USDTe/RUB"),
            Some(CurrencyPair::USDTeRUB)
        );
        assert_eq!(CurrencyPair::from_str("INVALID"), None);
    }

    #[test]
    fn test_currency_pair_mapping() {
        let config = NewLineConfig {
            base_url: "https://test.com".to_string(),
            cookie: "test_cookie".to_string(),
            preferred_city: "spb".to_string(),
        };

        let provider = NewLineProvider::new(config);

        // Test mapping indirectly through supports_currency_pair
        assert!(provider.supports_currency_pair(&CurrencyPair::USDCeRUB));
        assert!(provider.supports_currency_pair(&CurrencyPair::USDTeRUB));
        assert!(provider.supports_currency_pair(&CurrencyPair::USDRUB));
    }

    #[test]
    fn test_currency_pair_support() {
        let config = NewLineConfig {
            base_url: "https://test.com".to_string(),
            cookie: "test_cookie".to_string(),
            preferred_city: "spb".to_string(),
        };

        let provider = NewLineProvider::new(config);

        assert!(provider.supports_currency_pair(&CurrencyPair::USDCeRUB));
        assert!(provider.supports_currency_pair(&CurrencyPair::USDTeRUB));
        assert!(provider.supports_currency_pair(&CurrencyPair::USDRUB));
    }

    #[test]
    fn test_get_all_currency_pairs() {
        let pairs = get_all_currency_pairs();
        assert_eq!(pairs.len(), 3);
        assert!(pairs.contains(&CurrencyPair::USDRUB));
        assert!(pairs.contains(&CurrencyPair::USDCeRUB));
        assert!(pairs.contains(&CurrencyPair::USDTeRUB));
    }
}
