// Domain module for currency pairs
// Defines the domain-specific currency pairs used by the application

use serde::{Deserialize, Serialize};
use std::fmt;
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
    Network(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Parsing error: {0}")]
    Parsing(String),

    #[error("Provider-specific error: {0}")]
    Provider(String),
}

/// Domain currency pairs used in the application
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurrencyPair {
    /// USDC (ERC20) to RUB
    USDCeRUB,
    /// USDT (ERC20) to RUB  
    USDTeRUB,
    /// USD to RUB
    Usdrub,
}

impl fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CurrencyPair::USDCeRUB => "USDCe/RUB",
            CurrencyPair::USDTeRUB => "USDTe/RUB",
            CurrencyPair::Usdrub => "USD/RUB",
        };
        write!(f, "{}", s)
    }
}

impl CurrencyPair {
    /// Parse string to CurrencyPair
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "USDCe/RUB" => Some(CurrencyPair::USDCeRUB),
            "USDTe/RUB" => Some(CurrencyPair::USDTeRUB),
            "USD/RUB" => Some(CurrencyPair::Usdrub),
            _ => None,
        }
    }
}

/// Get all available domain currency pairs
pub fn get_all_currency_pairs() -> Vec<CurrencyPair> {
    vec![
        CurrencyPair::USDCeRUB,
        CurrencyPair::USDTeRUB,
        CurrencyPair::Usdrub,
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
            Some(CurrencyPair::Usdrub)
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
        assert!(provider.supports_currency_pair(&CurrencyPair::Usdrub));
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
        assert!(provider.supports_currency_pair(&CurrencyPair::Usdrub));
    }

    #[test]
    fn test_get_all_currency_pairs() {
        let pairs = get_all_currency_pairs();
        assert_eq!(pairs.len(), 3);
        assert!(pairs.contains(&CurrencyPair::Usdrub));
        assert!(pairs.contains(&CurrencyPair::USDCeRUB));
        assert!(pairs.contains(&CurrencyPair::USDTeRUB));
    }
}
