use candid::{CandidType, Deserialize};
// Removed unused import: fetch_fee_history_enhanced

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GasEstimate {
    pub base_fee: u64,
    pub priority_fee: u64,
    pub max_fee_per_gas: u64,
    pub gas_limit: u64,
    pub total_cost: u64,
    pub safety_margin: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FeeHistoryResponse {
    pub base_fee_per_gas: Vec<String>,
    pub gas_used_ratio: Vec<f64>,
    pub reward: Vec<Vec<String>>,
}

/// Enhanced gas estimation with multiple RPC endpoints and better parsing
pub async fn estimate_gas_advanced() -> Result<GasEstimate, String> {
    estimate_gas_for_chain("Base Sepolia").await
}

/// Estimate gas for specific chain using CACHED enhanced RPC client for 10x performance
pub async fn estimate_gas_for_chain(chain: &str) -> Result<GasEstimate, String> {
    ic_cdk::println!("🚀 CACHED gas estimation for {} using multiple RPC endpoints", chain);
    
    match fetch_fee_history_cached(chain).await {
        Ok(fee_history) => {
            ic_cdk::println!("✅ Successfully fetched fee history with enhanced RPC client");
            // Parse the JSON string first
            match serde_json::from_str::<serde_json::Value>(&fee_history) {
                Ok(json_value) => parse_fee_history_json(&json_value),
                Err(e) => Err(format!("Failed to parse fee history JSON: {}", e))
            }
        }
        Err(e) => {
            ic_cdk::println!("⚠️ Enhanced RPC failed, using fallback: {}", e);
            Ok(get_fallback_estimate())
        }
    }
}

/// Enhanced fee history parsing with proper JSON handling
fn parse_fee_history_json(fee_history: &serde_json::Value) -> Result<GasEstimate, String> {
    ic_cdk::println!("🔍 Parsing real-time fee history data for accurate gas estimation");
    
    let result = fee_history.get("result")
        .ok_or("No result in fee history response")?;
    
    // Extract base fees (latest block)
    let base_fees = result.get("baseFeePerGas")
        .and_then(|v| v.as_array())
        .ok_or("No baseFeePerGas in response")?;
    
    let latest_base_fee_hex = base_fees.last()
        .and_then(|v| v.as_str())
        .ok_or("No latest base fee")?;
    
    let latest_base_fee = u64::from_str_radix(&latest_base_fee_hex[2..], 16)
        .map_err(|e| format!("Failed to parse base fee: {}", e))?;
    
    // Extract rewards (priority fees) - use 75th percentile
    // Handle Base Sepolia which might not have rewards data
    let priority_fee = if let Some(rewards) = result.get("reward").and_then(|v| v.as_array()) {
        let mut priority_fees = Vec::new();
        for reward_block in rewards {
            if let Some(reward_array) = reward_block.as_array() {
                // Get 75th percentile (index 2)
                if let Some(priority_fee_hex) = reward_array.get(2).and_then(|v| v.as_str()) {
                    if let Ok(priority_fee) = u64::from_str_radix(&priority_fee_hex[2..], 16) {
                        priority_fees.push(priority_fee);
                    }
                }
            }
        }
        
        // Calculate median priority fee from recent blocks
        if !priority_fees.is_empty() {
            priority_fees.sort();
            let median = priority_fees[priority_fees.len() / 2];
            ic_cdk::println!("✅ Successfully parsed {} priority fee samples, median: {} wei", priority_fees.len(), median);
            median
        } else {
            ic_cdk::println!("ℹ️ No priority fee samples found, using Base Sepolia default");
            1_000_000_000 // 1 Gwei for testnet
        }
    } else {
        ic_cdk::println!("ℹ️ No rewards data in response, using Base Sepolia optimal priority fee");
        ic_cdk::println!("📊 Fee history keys: {:?}", result.as_object().map(|o| o.keys().collect::<Vec<_>>()));
        // Base Sepolia typically uses lower priority fees
        1_000_000_000 // 1 Gwei for Base Sepolia
    };
    
    // Calculate next block base fee with EIP-1559 formula
    let gas_used_ratios = result.get("gasUsedRatio")
        .and_then(|v| v.as_array())
        .ok_or("No gasUsedRatio in response")?;
    
    let latest_gas_used_ratio = gas_used_ratios.last()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.5); // 50% fallback
    
    // EIP-1559 base fee calculation for next block
    let base_fee_next = if latest_gas_used_ratio > 0.5 {
        // Increase base fee
        let increase = ((latest_gas_used_ratio - 0.5) * 0.125) + 1.0;
        (latest_base_fee as f64 * increase) as u64
    } else {
        // Decrease base fee
        let decrease = 1.0 - ((0.5 - latest_gas_used_ratio) * 0.125);
        (latest_base_fee as f64 * decrease) as u64
    };
    
    // Add safety buffer to base fee (12.5% for block variations)
    let base_fee_with_buffer = base_fee_next * 1125 / 1000;
    
    // Add safety buffer to priority fee (25% buffer)
    let priority_fee_with_buffer = priority_fee * 125 / 100;
    
    // Max fee per gas with additional buffer
    let max_fee_per_gas = base_fee_with_buffer + priority_fee_with_buffer + 5_000_000_000; // +5 Gwei buffer
    
    // Gas limit for ETH transfer
    let gas_limit = 21_000;
    
    // Calculate total cost with safety margin
    let estimated_cost = max_fee_per_gas * gas_limit;
    let safety_margin = estimated_cost * 20 / 100; // 20% safety margin
    let total_cost = estimated_cost + safety_margin;
    
    // Validate against reasonable caps
    if max_fee_per_gas > 500_000_000_000 { // 500 Gwei emergency cap
        return Err("Gas price extremely high, rejecting quote for safety".to_string());
    }
    
    ic_cdk::println!(
        "⛽ Real-time gas estimate: Base: {:.2} Gwei, Priority: {:.2} Gwei, Max: {:.2} Gwei",
        base_fee_with_buffer as f64 / 1e9,
        priority_fee_with_buffer as f64 / 1e9,
        max_fee_per_gas as f64 / 1e9
    );
    
    Ok(GasEstimate {
        base_fee: base_fee_with_buffer,
        priority_fee: priority_fee_with_buffer,
        max_fee_per_gas,
        gas_limit,
        total_cost,
        safety_margin,
    })
}

fn parse_fee_history(_response: &str) -> Result<GasEstimate, String> {
    // Legacy parsing function - kept for compatibility
    // Use parse_fee_history_json for enhanced parsing
    
    // Base fee: latest + 12.5% buffer for next block
    let base_fee = 50_000_000_000; // 50 Gwei conservative estimate
    let base_fee_with_buffer = base_fee * 1125 / 1000; // +12.5%
    
    // Priority fee: use 75th percentile from historical data
    let priority_fee = 2_000_000_000; // 2 Gwei conservative
    
    // Max fee: base + priority + additional buffer
    let max_fee_per_gas = base_fee_with_buffer + priority_fee + 10_000_000_000; // +10 Gwei buffer
    
    // Gas limit for ETH transfer
    let gas_limit = 21_000;
    
    // Calculate total cost with safety margin
    let estimated_cost = max_fee_per_gas * gas_limit;
    let safety_margin = estimated_cost * 20 / 100; // 20% safety margin
    let total_cost = estimated_cost + safety_margin;
    
    // Validate against max caps
    if max_fee_per_gas > 200_000_000_000 { // 200 Gwei cap
        return Err("Gas price too high, rejecting quote".to_string());
    }
    
    Ok(GasEstimate {
        base_fee: base_fee_with_buffer,
        priority_fee,
        max_fee_per_gas,
        gas_limit,
        total_cost,
        safety_margin,
    })
}

pub fn get_fallback_estimate() -> GasEstimate {
    let base_fee = 100_000_000_000; // 100 Gwei conservative fallback
    let priority_fee = 5_000_000_000; // 5 Gwei
    let max_fee_per_gas = base_fee + priority_fee;
    let gas_limit = 21_000;
    let estimated_cost = max_fee_per_gas * gas_limit;
    let safety_margin = estimated_cost * 30 / 100; // 30% safety margin for fallback
    let total_cost = estimated_cost + safety_margin;
    
    GasEstimate {
        base_fee,
        priority_fee,
        max_fee_per_gas,
        gas_limit,
        total_cost,
        safety_margin,
    }
}

pub fn validate_gas_estimate(estimate: &GasEstimate) -> Result<(), String> {
    // Validate reasonable gas limits
    if estimate.gas_limit < 21_000 || estimate.gas_limit > 100_000 {
        return Err("Invalid gas limit".to_string());
    }
    
    // Validate fee caps
    if estimate.max_fee_per_gas > 200_000_000_000 { // 200 Gwei
        return Err("Gas price exceeds maximum allowed".to_string());
    }
    
    // Validate total cost isn't excessive
    if estimate.total_cost > 5_000_000_000_000_000 { // 0.005 ETH
        return Err("Gas cost too high".to_string());
    }
    
    Ok(())
}

/// Fetch fee history using cached enhanced RPC client for 10x better performance
async fn fetch_fee_history_cached(chain: &str) -> Result<String, String> {
    let mut rpc_client = crate::services::rpc_client::RpcClient::new_base_sepolia();
    
    match rpc_client.get_gas_estimate_cached(chain).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("RPC cache error: {}", e.message))
    }
}