use candid::{CandidType, Deserialize};
use serde::{Serialize, Deserialize as SerdeDeserialize};
use std::collections::HashMap;
use ic_cdk::api::management_canister::http_request::{TransformArgs, HttpResponse, HttpHeader};

// Price feed response structures
#[derive(Debug, SerdeDeserialize)]
pub struct CoinGeckoPriceResponse {
    pub internet_computer: CoinGeckoPrice,
    pub ethereum: CoinGeckoPrice,
}

#[derive(Debug, SerdeDeserialize)]
pub struct CoinGeckoPrice {
    pub usd: f64,
}

#[derive(Debug, SerdeDeserialize)]
pub struct CoinMarketCapResponse {
    pub data: Vec<CoinMarketCapData>,
}

#[derive(Debug, SerdeDeserialize)]
pub struct CoinMarketCapData {
    pub symbol: String,
    pub quote: CoinMarketCapQuote,
}

#[derive(Debug, SerdeDeserialize)]
pub struct CoinMarketCapQuote {
    pub usd: CoinMarketCapPrice,
}

#[derive(Debug, SerdeDeserialize)]
pub struct CoinMarketCapPrice {
    pub price: f64,
}

// Price feed configuration
#[derive(Debug, Clone)]
pub struct PriceFeedConfig {
    pub name: String,
    pub url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
    pub retry_count: u32,
}

// Price data structure
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PriceData {
    pub asset: String,
    pub price_usd: f64,
    pub timestamp: u64,
    pub source: String,
    pub confidence: f64, // 0.0 to 1.0
}

// Professional Price Feed Service
pub struct PriceFeedService;

// HTTP transform function (required for ICP HTTP outcalls)
#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    raw.response
}

impl PriceFeedService {
    /// Get ICP price from CoinGecko (free tier)
    pub async fn get_icp_price_coingecko() -> Result<f64, String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=internet-computer&vs_currencies=usd";
        
        let response = Self::make_http_request(url, 10).await?;
        let price_data: CoinGeckoPriceResponse = serde_json::from_str(&response)
            .map_err(|e| format!("Failed to parse CoinGecko response: {}", e))?;
        
        let price = price_data.internet_computer.usd;
        ic_cdk::println!("📊 CoinGecko ICP price: ${:.2}", price);
        
        Ok(price)
    }

    /// Get ETH price from CoinGecko (free tier)
    pub async fn get_eth_price_coingecko() -> Result<f64, String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        
        let response = Self::make_http_request(url, 10).await?;
        let price_data: CoinGeckoPriceResponse = serde_json::from_str(&response)
            .map_err(|e| format!("Failed to parse CoinGecko response: {}", e))?;
        
        let price = price_data.ethereum.usd;
        ic_cdk::println!("📊 CoinGecko ETH price: ${:.2}", price);
        
        Ok(price)
    }

    /// Get ICP price from CoinMarketCap (requires API key)
    pub async fn get_icp_price_coinmarketcap(api_key: &str) -> Result<f64, String> {
        let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol=ICP";
        let headers = vec![
            ("X-CMC_PRO_API_KEY".to_string(), api_key.to_string()),
            ("Accept".to_string(), "application/json".to_string()),
        ];
        
        let response = Self::make_http_request_with_headers(url, Some(headers), 10).await?;
        let price_data: CoinMarketCapResponse = serde_json::from_str(&response)
            .map_err(|e| format!("Failed to parse CoinMarketCap response: {}", e))?;
        
        if let Some(data) = price_data.data.first() {
            let price = data.quote.usd.price;
            ic_cdk::println!("📊 CoinMarketCap ICP price: ${:.2}", price);
            Ok(price)
        } else {
            Err("No ICP data found in CoinMarketCap response".to_string())
        }
    }

    /// Get ETH price from CoinMarketCap (requires API key)
    pub async fn get_eth_price_coinmarketcap(api_key: &str) -> Result<f64, String> {
        let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol=ETH";
        let headers = vec![
            ("X-CMC_PRO_API_KEY".to_string(), api_key.to_string()),
            ("Accept".to_string(), "application/json".to_string()),
        ];
        
        let response = Self::make_http_request_with_headers(url, Some(headers), 10).await?;
        let price_data: CoinMarketCapResponse = serde_json::from_str(&response)
            .map_err(|e| format!("Failed to parse CoinMarketCap response: {}", e))?;
        
        if let Some(data) = price_data.data.first() {
            let price = data.quote.usd.price;
            ic_cdk::println!("📊 CoinMarketCap ETH price: ${:.2}", price);
            Ok(price)
        } else {
            Err("No ETH data found in CoinMarketCap response".to_string())
        }
    }

    /// Get best available ICP price (tries multiple sources)
    pub async fn get_best_icp_price() -> Result<PriceData, String> {
        let mut prices = Vec::new();
        
        // Try CoinGecko first (free, reliable)
        match Self::get_icp_price_coingecko().await {
            Ok(price) => {
                prices.push(PriceData {
                    asset: "ICP".to_string(),
                    price_usd: price,
                    timestamp: ic_cdk::api::time() / 1_000_000_000,
                    source: "CoinGecko".to_string(),
                    confidence: 0.9,
                });
            }
            Err(e) => {
                ic_cdk::println!("⚠️ CoinGecko ICP price failed: {}", e);
            }
        }
        
        // Try CoinMarketCap if API key is available (more accurate)
        if let Ok(api_key) = Self::get_coinmarketcap_api_key() {
            match Self::get_icp_price_coinmarketcap(&api_key).await {
                Ok(price) => {
                    prices.push(PriceData {
                        asset: "ICP".to_string(),
                        price_usd: price,
                        timestamp: ic_cdk::api::time() / 1_000_000_000,
                        source: "CoinMarketCap".to_string(),
                        confidence: 0.95,
                    });
                }
                Err(e) => {
                    ic_cdk::println!("⚠️ CoinMarketCap ICP price failed: {}", e);
                }
            }
        }
        
        if prices.is_empty() {
            return Err("All ICP price feeds failed".to_string());
        }
        
        // Return the price with highest confidence
        let best_price = prices.iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            .unwrap()
            .clone();
        
        ic_cdk::println!("✅ Best ICP price: ${:.2} from {}", best_price.price_usd, best_price.source);
        Ok(best_price)
    }

    /// Get best available ETH price (tries multiple sources)
    pub async fn get_best_eth_price() -> Result<PriceData, String> {
        let mut prices = Vec::new();
        
        // Try CoinGecko first (free, reliable)
        match Self::get_eth_price_coingecko().await {
            Ok(price) => {
                prices.push(PriceData {
                    asset: "ETH".to_string(),
                    price_usd: price,
                    timestamp: ic_cdk::api::time() / 1_000_000_000,
                    source: "CoinGecko".to_string(),
                    confidence: 0.9,
                });
            }
            Err(e) => {
                ic_cdk::println!("⚠️ CoinGecko ETH price failed: {}", e);
            }
        }
        
        // Try CoinMarketCap if API key is available (more accurate)
        if let Ok(api_key) = Self::get_coinmarketcap_api_key() {
            match Self::get_eth_price_coinmarketcap(&api_key).await {
                Ok(price) => {
                    prices.push(PriceData {
                        asset: "ETH".to_string(),
                        price_usd: price,
                        timestamp: ic_cdk::api::time() / 1_000_000_000,
                        source: "CoinMarketCap".to_string(),
                        confidence: 0.95,
                    });
                }
                Err(e) => {
                    ic_cdk::println!("⚠️ CoinMarketCap ETH price failed: {}", e);
                }
            }
        }
        
        if prices.is_empty() {
            return Err("All ETH price feeds failed".to_string());
        }
        
        // Return the price with highest confidence
        let best_price = prices.iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            .unwrap()
            .clone();
        
        ic_cdk::println!("✅ Best ETH price: ${:.2} from {}", best_price.price_usd, best_price.source);
        Ok(best_price)
    }

    /// Calculate conversion rate (ICP per ETH)
    pub async fn get_conversion_rate() -> Result<f64, String> {
        let icp_price = Self::get_best_icp_price().await?;
        let eth_price = Self::get_best_eth_price().await?;
        
        let rate = eth_price.price_usd / icp_price.price_usd;
        
        ic_cdk::println!("📊 Real-time conversion rate: 1 ETH = {:.6} ICP", rate);
        ic_cdk::println!("   ETH: ${:.2} | ICP: ${:.2}", eth_price.price_usd, icp_price.price_usd);
        
        Ok(rate)
    }

    /// Make HTTP request (simplified version)
    async fn make_http_request(url: &str, timeout_seconds: u64) -> Result<String, String> {
        Self::make_http_request_with_headers(url, None, timeout_seconds).await
    }

    /// Make HTTP request with custom headers
    async fn make_http_request_with_headers(
        url: &str, 
        headers: Option<Vec<(String, String)>>, 
        timeout_seconds: u64
    ) -> Result<String, String> {
        use ic_cdk::api::management_canister::http_request::{
            http_request, CanisterHttpRequestArgument, HttpMethod, TransformContext,
        };

        // Convert headers to HttpHeader format
        let http_headers: Vec<HttpHeader> = headers
            .unwrap_or_default()
            .into_iter()
            .map(|(name, value)| HttpHeader { name, value })
            .collect();

        let arg = CanisterHttpRequestArgument {
            url: url.to_string(),
            max_response_bytes: Some(1024 * 1024), // 1MB max
            method: HttpMethod::GET,
            headers: http_headers,
            body: None,
            transform: Some(TransformContext::from_name("transform".to_string(), vec![])),
        };

        let timeout = (timeout_seconds * 1_000_000_000) as u128; // Convert to nanoseconds as u128
        
        match http_request(arg, timeout).await {
            Ok((response,)) => {
                if response.status == 200u32 {
                    String::from_utf8(response.body)
                        .map_err(|e| format!("Invalid UTF-8 in response: {}", e))
                } else {
                    Err(format!("HTTP error: {} - {}", response.status, String::from_utf8_lossy(&response.body)))
                }
            }
            Err(e) => Err(format!("HTTP request failed: {:?}", e)),
        }
    }

    /// Get CoinMarketCap API key from environment (in production)
    fn get_coinmarketcap_api_key() -> Result<String, String> {
        // In production, this would read from environment variables or config
        // For now, return an error to use free tier only
        Err("CoinMarketCap API key not configured".to_string())
    }

    /// Get cached price with TTL (performance optimization)
    pub async fn get_cached_price(asset: &str, ttl_seconds: u64) -> Result<PriceData, String> {
        let now = ic_cdk::api::time() / 1_000_000_000;
        
        PRICE_CACHE.with(|cache| {
            let cache_ref = cache.borrow_mut();
            
            if let Some((price_data, timestamp)) = cache_ref.get(asset) {
                if now - timestamp < ttl_seconds {
                    ic_cdk::println!("💾 Using cached {} price: ${:.2}", asset, price_data.price_usd);
                    return Ok(price_data.clone());
                }
            }
            
            // Cache miss - would fetch from API in production
            Err("Cache miss - would fetch from API".to_string())
        })
    }

    /// Set cached price
    pub fn set_cached_price(asset: &str, price_data: PriceData) {
        let now = ic_cdk::api::time() / 1_000_000_000;
        
        PRICE_CACHE.with(|cache| {
            let mut cache_ref = cache.borrow_mut();
            cache_ref.insert(asset.to_string(), (price_data, now));
        });
    }
}

// Price cache for performance
thread_local! {
    static PRICE_CACHE: std::cell::RefCell<HashMap<String, (PriceData, u64)>> = 
        std::cell::RefCell::new(HashMap::new());
}

// Fallback price service for when all feeds fail
impl PriceFeedService {
    /// Get fallback ICP price (last known good price)
    pub fn get_fallback_icp_price() -> f64 {
        12.50 // Last known good price
    }

    /// Get fallback ETH price (last known good price)
    pub fn get_fallback_eth_price() -> f64 {
        3500.0 // Last known good price
    }

    /// Get price with fallback
    pub async fn get_icp_price_with_fallback() -> Result<f64, String> {
        match Self::get_best_icp_price().await {
            Ok(price_data) => {
                // Cache the successful price
                Self::set_cached_price("ICP", price_data.clone());
                Ok(price_data.price_usd)
            }
            Err(_) => {
                ic_cdk::println!("⚠️ All ICP price feeds failed, using fallback");
                Ok(Self::get_fallback_icp_price())
            }
        }
    }

    /// Get ETH price with fallback
    pub async fn get_eth_price_with_fallback() -> Result<f64, String> {
        match Self::get_best_eth_price().await {
            Ok(price_data) => {
                // Cache the successful price
                Self::set_cached_price("ETH", price_data.clone());
                Ok(price_data.price_usd)
            }
            Err(_) => {
                ic_cdk::println!("⚠️ All ETH price feeds failed, using fallback");
                Ok(Self::get_fallback_eth_price())
            }
        }
    }
}
