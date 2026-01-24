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
    USDCe2RUB,
    /// USDT (ERC20) to RUB  
    USDTe2RUB,
    /// USD to RUB
    USD2RUB,
}

impl fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CurrencyPair::USDCe2RUB => "USDCe/RUB",
            CurrencyPair::USDTe2RUB => "USDTe/RUB",
            CurrencyPair::USD2RUB => "USD/RUB",
        };
        write!(f, "{}", s)
    }
}

impl CurrencyPair {
    /// Parse string to CurrencyPair
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "USDCe/RUB" => Some(CurrencyPair::USDCe2RUB),
            "USDTe/RUB" => Some(CurrencyPair::USDTe2RUB),
            "USD/RUB" => Some(CurrencyPair::USD2RUB),
            _ => None,
        }
    }
}

/// Get all available domain currency pairs
pub fn get_all_currency_pairs() -> Vec<CurrencyPair> {
    vec![
        CurrencyPair::USDCe2RUB,
        CurrencyPair::USDTe2RUB,
        CurrencyPair::USD2RUB,
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
            Some(CurrencyPair::USD2RUB)
        );
        assert_eq!(
            CurrencyPair::from_str("USDCe/RUB"),
            Some(CurrencyPair::USDCe2RUB)
        );
        assert_eq!(
            CurrencyPair::from_str("USDTe/RUB"),
            Some(CurrencyPair::USDTe2RUB)
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
        assert!(provider.supports_currency_pair(&CurrencyPair::USDCe2RUB));
        assert!(provider.supports_currency_pair(&CurrencyPair::USDTe2RUB));
        assert!(provider.supports_currency_pair(&CurrencyPair::USD2RUB));
    }

    #[test]
    fn test_currency_pair_support() {
        let config = NewLineConfig {
            base_url: "https://test.com".to_string(),
            cookie: "test_cookie".to_string(),
            preferred_city: "spb".to_string(),
        };

        let provider = NewLineProvider::new(config);

        assert!(provider.supports_currency_pair(&CurrencyPair::USDCe2RUB));
        assert!(provider.supports_currency_pair(&CurrencyPair::USDTe2RUB));
        assert!(provider.supports_currency_pair(&CurrencyPair::USD2RUB));
    }

    #[test]
    fn test_get_all_currency_pairs() {
        let pairs = get_all_currency_pairs();
        assert_eq!(pairs.len(), 3);
        assert!(pairs.contains(&CurrencyPair::USD2RUB));
        assert!(pairs.contains(&CurrencyPair::USDCe2RUB));
        assert!(pairs.contains(&CurrencyPair::USDTe2RUB));
    }
}
