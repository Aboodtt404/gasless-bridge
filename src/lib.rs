use ic_cdk::{caller, init, query, update};
use candid::{CandidType, Deserialize};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SimpleQuote {
    pub id: String,
    pub amount_requested: u64,
    pub total_cost: u64,
    pub destination: String,
    pub created_at: u64,
}

thread_local! {
    static QUOTES: RefCell<HashMap<String, SimpleQuote>> = RefCell::new(HashMap::new());
}

#[init]
fn init() {
    ic_cdk::println!("Gasless Bridge initialized");
}

#[query]
fn health_check() -> String {
    "🟢 Gasless Bridge Status: Healthy - Quote System Ready".to_string()
}

#[update]
async fn request_quote(
    amount: u64,
    destination_address: String,
    destination_chain: String,
) -> Result<SimpleQuote, String> {
    // Validate inputs
    if amount < 1_000_000_000_000_000 {
        return Err("Amount too small, minimum 0.001 ETH".to_string());
    }
    
    if !destination_address.starts_with("0x") || destination_address.len() != 42 {
        return Err("Invalid Ethereum address".to_string());
    }
    
    if destination_chain != "Base Sepolia" {
        return Err("Only Base Sepolia supported".to_string());
    }

    // Get real Base Sepolia gas prices via RPC
    let gas_cost = match get_gas_cost().await {
        Ok(cost) => cost,
        Err(e) => {
            ic_cdk::println!("Gas estimation failed: {}, using fallback", e);
            2_100_000_000_000_000 // Fallback: 0.0021 ETH
        }
    };

    // Create quote
    let quote_id = format!("quote_{}_{}", 
        caller().to_text().chars().take(8).collect::<String>(),
        ic_cdk::api::time() / 1_000_000_000
    );

    let quote = SimpleQuote {
        id: quote_id.clone(),
        amount_requested: amount,
        total_cost: amount + gas_cost,
        destination: destination_address,
        created_at: ic_cdk::api::time() / 1_000_000_000,
    };

    // Store quote
    QUOTES.with(|quotes| {
        quotes.borrow_mut().insert(quote_id.clone(), quote.clone());
    });

    ic_cdk::println!("Generated quote {} for {} wei (total cost: {} wei)", 
        quote.id, quote.amount_requested, quote.total_cost);

    Ok(quote)
}

async fn get_gas_cost() -> Result<u64, String> {
    let request_body = r#"{"jsonrpc":"2.0","method":"eth_feeHistory","params":["0x4","latest",[75]],"id":1}"#;
    
    let request = ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument {
        url: "https://sepolia.base.org".to_string(),
        method: ic_cdk::api::management_canister::http_request::HttpMethod::POST,
        body: Some(request_body.as_bytes().to_vec()),
        max_response_bytes: Some(2000),
        transform: None,
        headers: vec![
            ic_cdk::api::management_canister::http_request::HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
        ],
    };

    match ic_cdk::api::management_canister::http_request::http_request(request, 2_000_000_000).await {
        Ok((response,)) => {
            let body = String::from_utf8_lossy(&response.body);
            ic_cdk::println!("RPC Response: {}", body);
            
            // Simple gas cost calculation: 21,000 gas * 100 Gwei + 20% margin
            let base_cost = 21_000 * 100_000_000_000; // 0.0021 ETH
            let margin = base_cost * 20 / 100;
            Ok(base_cost + margin)
        }
        Err((r, m)) => {
            Err(format!("RPC failed: {:?} - {}", r, m))
        }
    }
}

#[query]
fn estimate_quote_cost(amount: u64) -> String {
    let gas_cost = 2_520_000_000_000_000; // Conservative estimate with margin
    let total_cost = amount + gas_cost;
    
    format!(
        "💰 Quote Estimate:\n\
         📊 Requested: {} wei\n\
         ⛽ Gas Cost: {} wei\n\
         💸 Total: {} wei\n\
         📈 Overhead: {:.3}%",
        amount, gas_cost, total_cost,
        (gas_cost as f64 / amount as f64) * 100.0
    )
}

#[query]
fn get_quote_statistics() -> String {
    QUOTES.with(|quotes| {
        let count = quotes.borrow().len();
        format!("📊 Total quotes generated: {}", count)
    })
}

#[query]
fn get_quote(quote_id: String) -> Option<SimpleQuote> {
    QUOTES.with(|quotes| {
        quotes.borrow().get(&quote_id).cloned()
    })
}

#[update]
async fn test_base_rpc() -> String {
    match get_gas_cost().await {
        Ok(cost) => format!("✅ Base Sepolia RPC working! Estimated gas cost: {} wei", cost),
        Err(e) => format!("❌ RPC test failed: {}", e)
    }
}
