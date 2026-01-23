use crate::domain::{CurrencyPair, PriceData, PriceProviderError};
use crate::price_service::provider::PriceProvider;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;

/// Configuration for the NewLine provider
#[derive(Debug, Clone)]
pub struct NewLineConfig {
    pub base_url: String,
    pub cookie: String,
    pub preferred_city: String,
}

/// NewLine API response structure for exchange data
#[derive(Debug, Serialize, Deserialize)]
pub struct NewLineExchange {
    pub from_: String,
    pub to_data: Vec<NewLineToData>,
}

/// NewLine API response structure for to_data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewLineToData {
    pub course_from: f64,
    pub course_to: f64,
    pub to: String,
}

/// NewLine API response structure for city data
#[derive(Debug, Serialize, Deserialize)]
pub struct NewLineCityData {
    pub city_code: String,
    pub data: Vec<NewLineExchange>,
}

/// Price provider implementation for NewLine API
pub struct NewLineProvider {
    config: NewLineConfig,
    client: Client,
    supported_symbols: Vec<String>,
}

impl NewLineProvider {
    /// Create a new NewLineProvider instance
    pub fn new(config: NewLineConfig) -> Self {
        let supported_symbols = vec![
            "BTC_TO_CASHRUB".to_string(),
            "USDTERC_TO_CASHRUB".to_string(),
            "CASHRUB_TO_USDTERC".to_string(),
            "USDTTRC_TO_CASHRUB".to_string(),
            "ETH_TO_CASHRUB".to_string(),
            "CASHUSD_TO_USDTERC".to_string(),
            "CASHRUB_TO_USDTTRC".to_string(),
            "CASHUSD_TO_USDTTRC".to_string(),
            "CASHRUB_TO_ETH".to_string(),
            "CASHRUB_TO_BTC".to_string(),
            "CASHUSD_TO_ETH".to_string(),
            "CASHUSD_TO_BTC".to_string(),
            "ETH_TO_CASHUSD".to_string(),
            "BTC_TO_CASHUSD".to_string(),
            "USDTERC_TO_CASHUSD".to_string(),
            "USDTTRC_TO_CASHUSD".to_string(),
        ];

        Self {
            config,
            client: Client::new(),
            supported_symbols,
        }
    }

    /// Extract price data from NewLine exchange data
    ///
    /// # Arguments
    ///
    /// * `to_data` - NewLine to_data containing to currency and rates
    /// * `pair` - The currency pair this data represents
    ///
    /// # Returns
    ///
    /// PriceData struct with currency pair and calculated price (course_to / course_from)
    fn extract_price_data(&self, to_data: &NewLineToData, pair: &CurrencyPair) -> PriceData {
        PriceData {
            pair: pair.clone(),
            price: to_data.course_to / to_data.course_from,
        }
    }

    /// Find price data in the preferred city
    ///
    /// # Arguments
    ///
    /// * `city_data_list` - List of city data from NewLine API
    /// * `symbol` - Symbol to search for (e.g., "USDTERC_TO_CASHRUB")
    ///
    /// # Returns
    ///
    /// Option<NewLineToData> - Some(to_data) if found in preferred city, None otherwise
    fn find_price_in_city_data(
        &self,
        city_data_list: &[NewLineCityData],
        symbol: &str,
    ) -> Option<NewLineToData> {
        for city_data in city_data_list {
            if city_data.city_code == self.config.preferred_city {
                for exchange in &city_data.data {
                    for to_data in &exchange.to_data {
                        let current_symbol = format!("{}_TO_{}", exchange.from_, to_data.to);
                        if current_symbol == symbol {
                            return Some(to_data.clone());
                        }
                    }
                }
            }
        }
        None
    }

    /// Map domain currency pair to NewLine provider symbol (private method)
    ///
    /// Note: Both USDCeRUB and USDTeRUB map to the same USDTERC_TO_CASHRUB symbol
    /// since the NewLine API doesn't distinguish between different ERC20 stablecoins.
    /// This is a provider limitation, not a bug in the mapping logic.
    fn map_currency_pair(&self, pair: &CurrencyPair) -> Option<String> {
        match pair {
            CurrencyPair::USDCeRUB => Some("USDTERC_TO_CASHRUB".to_string()),
            CurrencyPair::USDTeRUB => Some("USDTERC_TO_CASHRUB".to_string()),
            CurrencyPair::Usdrub => Some("CASHUSD_TO_USDTERC".to_string()),
        }
    }
}

#[async_trait]
impl PriceProvider for NewLineProvider {
    fn name(&self) -> &str {
        "NewLineProvider"
    }

    async fn fetch_price(&self, pair: &CurrencyPair) -> Result<PriceData, PriceProviderError> {
        let symbol = self.map_currency_pair(pair).ok_or_else(|| {
            PriceProviderError::Provider(format!(
                "Currency pair {} not supported by this provider",
                pair
            ))
        })?;

        let url = format!("{}/api/direction/", self.config.base_url);
        log::debug!("NewLineProvider: Fetching price for pair: {}", pair);
        log::debug!("NewLineProvider: Mapped to symbol: {}", symbol);
        log::debug!("NewLineProvider: Request URL: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .header("Cookie", &self.config.cookie)
            .send()
            .await
            .map_err(|e| PriceProviderError::Network(e.to_string()))?;

        let status = response.status();
        log::debug!("NewLineProvider: Response status: {}", status);

        let response_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unable to read response body".to_string());
        log::debug!("NewLineProvider: Response body: {}", response_text);

        if !status.is_success() {
            log::error!(
                "NewLineProvider: API request failed with status: {}, response: {}",
                status,
                response_text
            );
            return Err(PriceProviderError::Api(format!(
                "API request failed with status: {}",
                status
            )));
        }

        let city_data_list: Vec<NewLineCityData> = serde_json::from_str(&response_text)
            .map_err(|e| PriceProviderError::Parsing(e.to_string()))?;

        // Find the requested symbol in the preferred city
        if let Some(to_data) = self.find_price_in_city_data(&city_data_list, &symbol) {
            let price_data = self.extract_price_data(&to_data, pair);
            return Ok(price_data);
        }

        Err(PriceProviderError::Provider(format!(
            "Symbol {} not found in API response for pair {}",
            symbol, pair
        )))
    }

    /// Check if this provider supports the given currency pair
    fn supports_currency_pair(&self, pair: &CurrencyPair) -> bool {
        if let Some(symbol) = self.map_currency_pair(pair) {
            self.supported_symbols.contains(&symbol)
        } else {
            false
        }
    }
}
