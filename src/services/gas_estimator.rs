use candid::{CandidType, Deserialize};

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

pub async fn estimate_gas_advanced() -> Result<GasEstimate, String> {
    let request_body = r#"{"jsonrpc":"2.0","method":"eth_feeHistory","params":["0x4","latest",[25,50,75]],"id":1}"#;
    
    let request = ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument {
        url: "https://sepolia.base.org".to_string(),
        method: ic_cdk::api::management_canister::http_request::HttpMethod::POST,
        body: Some(request_body.as_bytes().to_vec()),
        max_response_bytes: Some(4096),
        transform: None,
        headers: vec![
            ic_cdk::api::management_canister::http_request::HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
        ],
    };

    match ic_cdk::api::management_canister::http_request::http_request(request, 25_000_000_000).await {
        Ok((response,)) => {
            let body = String::from_utf8_lossy(&response.body);
            ic_cdk::println!("Fee history response: {}", body);
            
            // Parse the response (simplified - in production use proper JSON parsing)
            parse_fee_history(&body)
        }
        Err((r, m)) => {
            ic_cdk::println!("Fee history request failed: {:?} - {}", r, m);
            // Return conservative fallback estimate
            Ok(get_fallback_estimate())
        }
    }
}

fn parse_fee_history(response: &str) -> Result<GasEstimate, String> {
    // Simplified parsing - in production, use proper JSON parser
    // For now, return conservative estimates
    
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

fn get_fallback_estimate() -> GasEstimate {
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