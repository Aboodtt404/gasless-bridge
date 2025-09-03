

use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::http_request::{
    CanisterHttpRequestArgument, HttpHeader, HttpMethod, http_request
};
use super::rpc_cache::{RpcCache, CacheStats, ttl};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RpcEndpoint {
    pub name: String,
    pub url: String,
    pub priority: u8,
    pub is_active: bool,
    pub last_success: Option<u64>,
    pub failure_count: u32,
    pub max_failures: u32,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RpcResponse {
    pub endpoint_used: String,
    pub response_time_ms: u64,
    pub body: String,
    pub success: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RpcError {
    pub endpoint: String,
    pub error_type: String,
    pub message: String,
    pub retry_after: Option<u64>,
}

pub struct RpcClient {
    endpoints: Vec<RpcEndpoint>,
    timeout_cycles: u128,
    max_response_bytes: u64,
    cache: RpcCache,
}

impl RpcClient {
    /// Create new RPC client with multiple Base Sepolia endpoints
    pub fn new_base_sepolia() -> Self {
        let endpoints = vec![
            RpcEndpoint {
                name: "Base Sepolia Public".to_string(),
                url: "https://base-sepolia.publicnode.com".to_string(),
                priority: 1,
                is_active: true,
                last_success: None,
                failure_count: 0,
                max_failures: 3,
            },
            RpcEndpoint {
                name: "Base Sepolia Ankr".to_string(),
                url: "https://rpc.ankr.com/base_sepolia".to_string(),
                priority: 2,
                is_active: true,
                last_success: None,
                failure_count: 0,
                max_failures: 3,
            },
            RpcEndpoint {
                name: "Base Sepolia 1RPC".to_string(),
                url: "https://1rpc.io/base-sepolia".to_string(),
                priority: 3,
                is_active: true,
                last_success: None,
                failure_count: 0,
                max_failures: 3,
            },
            RpcEndpoint {
                name: "Base Sepolia Official".to_string(),
                url: "https://sepolia.base.org".to_string(),
                priority: 4,
                is_active: true,
                last_success: None,
                failure_count: 0,
                max_failures: 3,
            },
        ];

        Self {
            endpoints,
            timeout_cycles: 25_000_000_000u128, // 25B cycles
            max_response_bytes: 4096,
            cache: RpcCache::new(100), // Cache up to 100 responses
        }
    }

    /// Make JSON-RPC call with automatic failover
    pub async fn call_with_failover(&mut self, method: &str, params: serde_json::Value) -> Result<RpcResponse, RpcError> {
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        // Sort endpoints by priority and filter active ones
        let mut active_endpoints = self.endpoints.iter_mut()
            .filter(|e| e.is_active && e.failure_count < e.max_failures)
            .collect::<Vec<_>>();
        
        active_endpoints.sort_by_key(|e| e.priority);

        if active_endpoints.is_empty() {
            return Err(RpcError {
                endpoint: "All".to_string(),
                error_type: "NoEndpoints".to_string(),
                message: "No active RPC endpoints available".to_string(),
                retry_after: Some(300), // 5 minutes
            });
        }

        let mut last_error = None;

        // Try each endpoint in order
        for endpoint in active_endpoints {
            ic_cdk::println!("🌐 Trying RPC endpoint: {} (priority {})", endpoint.name, endpoint.priority);
            
            let start_time = ic_cdk::api::time();
            
            let result = Self::make_request_static(endpoint, &request_body.to_string(), self.timeout_cycles, self.max_response_bytes).await;
            
            let response_time = (ic_cdk::api::time() - start_time) / 1_000_000; // Convert to ms

            match result {
                Ok(body) => {
                    // Success - update endpoint stats
                    endpoint.last_success = Some(ic_cdk::api::time());
                    endpoint.failure_count = 0; // Reset failure count on success
                    
                    ic_cdk::println!("✅ RPC success with {} in {}ms", endpoint.name, response_time);
                    
                    return Ok(RpcResponse {
                        endpoint_used: endpoint.name.clone(),
                        response_time_ms: response_time,
                        body,
                        success: true,
                    });
                }
                Err(error) => {
                    // Failure - update endpoint stats
                    endpoint.failure_count += 1;
                    
                    ic_cdk::println!(
                        "❌ RPC failed with {} (attempt {}/{}): {}", 
                        endpoint.name, 
                        endpoint.failure_count, 
                        endpoint.max_failures,
                        error.message
                    );
                    
                    // Disable endpoint if it exceeds max failures
                    if endpoint.failure_count >= endpoint.max_failures {
                        ic_cdk::println!("🚫 Disabling endpoint {} due to repeated failures", endpoint.name);
                        endpoint.is_active = false;
                    }
                    
                    last_error = Some(error);
                    continue; // Try next endpoint
                }
            }
        }

        // All endpoints failed
        Err(last_error.unwrap_or(RpcError {
            endpoint: "Unknown".to_string(),
            error_type: "AllFailed".to_string(),
            message: "All RPC endpoints failed".to_string(),
            retry_after: Some(60),
        }))
    }

    /// Make HTTP request to specific endpoint (static version to avoid borrowing issues)
    async fn make_request_static(endpoint: &RpcEndpoint, body: &str, timeout_cycles: u128, max_response_bytes: u64) -> Result<String, RpcError> {
        let request = CanisterHttpRequestArgument {
            url: endpoint.url.clone(),
            method: HttpMethod::POST,
            body: Some(body.as_bytes().to_vec()),
            max_response_bytes: Some(max_response_bytes),
            transform: None,
            headers: vec![
                HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                },
                HttpHeader {
                    name: "Accept".to_string(),
                    value: "application/json".to_string(),
                },
                HttpHeader {
                    name: "User-Agent".to_string(),
                    value: "HyperBridge/1.0".to_string(),
                },
            ],
        };

        match http_request(request, timeout_cycles).await {
            Ok((response,)) => {
                if response.status == 200u32 {
                    let body = String::from_utf8_lossy(&response.body);
                    Ok(body.to_string())
                } else {
                    Err(RpcError {
                        endpoint: endpoint.name.clone(),
                        error_type: "HttpError".to_string(),
                        message: format!("HTTP {}", response.status),
                        retry_after: Some(30),
                    })
                }
            }
            Err((rejection_code, message)) => {
                Err(RpcError {
                    endpoint: endpoint.name.clone(),
                    error_type: format!("RejectionCode({:?})", rejection_code),
                    message,
                    retry_after: Some(60),
                })
            }
        }
    }

    /// Get endpoint health status
    /// Get cached gas estimation with automatic cache management
    pub async fn get_gas_estimate_cached(&mut self, chain: &str) -> Result<String, RpcError> {
        let cache_key = RpcCache::gas_estimation_key(chain);
        
        // Try cache first
        if let Some(cached_response) = self.cache.get(&cache_key) {
            return Ok(cached_response);
        }
        
        // Cache miss - fetch fresh data
        // Request 10 blocks with 25th, 50th, and 75th percentile rewards
        let params = serde_json::json!([10, "latest", [25, 50, 75]]);
        match self.call_with_failover("eth_feeHistory", params).await {
            Ok(response) => {
                // Cache the response
                self.cache.set(cache_key, response.body.clone(), ttl::GAS_ESTIMATE);
                Ok(response.body)
            }
            Err(e) => Err(e)
        }
    }

    /// Broadcast a signed Ethereum transaction to the network
    /// This is the final step in the ckETH → ETH flow!
    pub async fn broadcast_transaction(&mut self, raw_transaction: &str, chain: &str) -> Result<String, RpcError> {
        ic_cdk::println!("📡 Broadcasting transaction to {}: {}", chain, raw_transaction);
        
        let params = serde_json::json!([raw_transaction]);
        
        match self.call_with_failover("eth_sendRawTransaction", params).await {
            Ok(response) => {
                // Parse transaction hash from response
                let json: serde_json::Value = serde_json::from_str(&response.body)
                    .map_err(|e| RpcError {
                        endpoint: "broadcast".to_string(),
                        error_type: "ParseError".to_string(),
                        message: format!("Failed to parse broadcast response: {}", e),
                        retry_after: None,
                    })?;
                
                let tx_hash = json.get("result")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| RpcError {
                        endpoint: "broadcast".to_string(),
                        error_type: "DataError".to_string(),
                        message: "No transaction hash in broadcast response".to_string(),
                        retry_after: None,
                    })?;
                
                ic_cdk::println!("✅ Transaction broadcast successful! Hash: {}", tx_hash);
                Ok(tx_hash.to_string())
            }
            Err(e) => {
                ic_cdk::println!("❌ Transaction broadcast failed: {:?}", e);
                Err(e)
            }
        }
    }

    /// Get cached nonce with automatic cache management
    pub async fn get_nonce_cached(&mut self, address: &str, chain: &str) -> Result<u64, RpcError> {
        let cache_key = RpcCache::nonce_key(address, chain);
        
        // Try cache first
        if let Some(cached_response) = self.cache.get(&cache_key) {
            // Parse cached nonce
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cached_response) {
                if let Some(nonce_hex) = json.get("result").and_then(|v| v.as_str()) {
                    if let Ok(nonce) = u64::from_str_radix(&nonce_hex[2..], 16) {
                        return Ok(nonce);
                    }
                }
            }
        }
        
        // Cache miss - fetch fresh data
        let params = serde_json::json!([address, "pending"]);
        match self.call_with_failover("eth_getTransactionCount", params).await {
            Ok(response) => {
                // Parse nonce
                let json: serde_json::Value = serde_json::from_str(&response.body)
                    .map_err(|e| RpcError {
                        endpoint: "cache".to_string(),
                        error_type: "ParseError".to_string(),
                        message: format!("Failed to parse nonce response: {}", e),
                        retry_after: None,
                    })?;
                
                let nonce_hex = json.get("result")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| RpcError {
                        endpoint: "cache".to_string(),
                        error_type: "DataError".to_string(),
                        message: "No nonce in response".to_string(),
                        retry_after: None,
                    })?;
                
                let nonce = u64::from_str_radix(&nonce_hex[2..], 16)
                    .map_err(|e| RpcError {
                        endpoint: "cache".to_string(),
                        error_type: "ParseError".to_string(),
                        message: format!("Failed to parse nonce hex: {}", e),
                        retry_after: None,
                    })?;
                
                // Cache the response
                self.cache.set(cache_key, response.body, ttl::NONCE);
                Ok(nonce)
            }
            Err(e) => Err(e)
        }
    }

    /// Get cache statistics for monitoring
    pub fn get_cache_stats(&self) -> CacheStats {
        self.cache.get_stats()
    }

    /// Manually invalidate gas estimate cache (call after detecting new block)
    pub fn invalidate_gas_cache(&mut self) {
        self.cache.invalidate_gas_estimates();
    }

    /// Cleanup expired cache entries (call periodically)
    pub fn cleanup_cache(&mut self) {
        self.cache.cleanup_expired();
    }

    pub fn get_health_status(&self) -> String {
        let total = self.endpoints.len();
        let active = self.endpoints.iter().filter(|e| e.is_active).count();
        let healthy = self.endpoints.iter().filter(|e| e.is_active && e.failure_count == 0).count();
        
        format!(
            "🌐 RPC Health: {}/{} active, {}/{} healthy\n{}",
            active, total, healthy, total,
            self.endpoints.iter()
                .map(|e| format!(
                    "  • {}: {} (failures: {}/{})",
                    e.name,
                    if e.is_active { "✅" } else { "❌" },
                    e.failure_count,
                    e.max_failures
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Reset all endpoint failure counts (for manual recovery)
    pub fn reset_all_endpoints(&mut self) {
        for endpoint in &mut self.endpoints {
            endpoint.failure_count = 0;
            endpoint.is_active = true;
        }
        ic_cdk::println!("🔄 All RPC endpoints reset and reactivated");
    }
}

/// Enhanced fee history fetching with multiple RPC support
pub async fn fetch_fee_history_enhanced(chain: &str) -> Result<serde_json::Value, String> {
    let mut rpc_client = match chain {
        "Base Sepolia" => RpcClient::new_base_sepolia(),
        _ => return Err(format!("Unsupported chain: {}", chain)),
    };

    let params = serde_json::json!(["0x4", "latest", [25, 50, 75]]);
    
    match rpc_client.call_with_failover("eth_feeHistory", params).await {
        Ok(response) => {
            // Parse the JSON response
            serde_json::from_str(&response.body)
                .map_err(|e| format!("Failed to parse fee history response: {}", e))
        }
        Err(error) => {
            ic_cdk::println!("🚨 All RPC endpoints failed for fee history: {}", error.message);
            Err(format!("RPC failure: {}", error.message))
        }
    }
}

/// Get current nonce for an address with RPC failover
pub async fn get_nonce_enhanced(address: &str, chain: &str) -> Result<u64, String> {
    let mut rpc_client = match chain {
        "Base Sepolia" => RpcClient::new_base_sepolia(),
        _ => return Err(format!("Unsupported chain: {}", chain)),
    };

    let params = serde_json::json!([address, "pending"]);
    
    match rpc_client.call_with_failover("eth_getTransactionCount", params).await {
        Ok(response) => {
            let json: serde_json::Value = serde_json::from_str(&response.body)
                .map_err(|e| format!("Failed to parse nonce response: {}", e))?;
            
            let nonce_hex = json.get("result")
                .and_then(|v| v.as_str())
                .ok_or("No nonce in response")?;
            
            let nonce = u64::from_str_radix(&nonce_hex[2..], 16)
                .map_err(|e| format!("Failed to parse nonce hex: {}", e))?;
            
            Ok(nonce)
        }
        Err(error) => {
            ic_cdk::println!("🚨 Failed to get nonce: {}", error.message);
            // Fallback to timestamp-based nonce
            Ok(ic_cdk::api::time() / 1_000_000_000)
        }
    }
}

/// Broadcast transaction with RPC failover
pub async fn broadcast_transaction_enhanced(raw_tx: &str, chain: &str) -> Result<String, String> {
    let mut rpc_client = match chain {
        "Base Sepolia" => RpcClient::new_base_sepolia(),
        _ => return Err(format!("Unsupported chain: {}", chain)),
    };

    let params = serde_json::json!([raw_tx]);
    
    match rpc_client.call_with_failover("eth_sendRawTransaction", params).await {
        Ok(response) => {
            let json: serde_json::Value = serde_json::from_str(&response.body)
                .map_err(|e| format!("Failed to parse broadcast response: {}", e))?;
            
            if let Some(error) = json.get("error") {
                return Err(format!("RPC error: {}", error));
            }
            
            let tx_hash = json.get("result")
                .and_then(|v| v.as_str())
                .ok_or("No transaction hash in response")?;
            
            Ok(tx_hash.to_string())
        }
        Err(error) => {
            Err(format!("Failed to broadcast transaction: {}", error.message))
        }
    }
}

/// Public API functions

/// Broadcast a signed Ethereum transaction
pub async fn broadcast_ethereum_transaction(raw_tx: &str, chain: &str) -> Result<String, String> {
    let mut client = RpcClient::new_base_sepolia();
    client.broadcast_transaction(raw_tx, chain)
        .await
        .map_err(|e| format!("Broadcast failed: {}", e.message))
}

/// Test the RPC client with a simple health check
pub async fn test_rpc_client_health() -> Result<String, String> {
    let client = RpcClient::new_base_sepolia();
    Ok(client.get_health_status())
}
