mod types;
mod services;
mod storage;
mod handlers;
mod utils;

use ic_cdk::{caller, init, post_upgrade, pre_upgrade, query, update};
use candid::{CandidType, Deserialize};
use std::cell::RefCell;

// Import our new types and services
use crate::types::{Quote, QuoteRequest, Settlement};
use crate::storage::state::BridgeState;
use crate::services::gas_estimator::{estimate_gas_advanced, validate_gas_estimate};

// Global state using our new BridgeState
thread_local! {
    static STATE: RefCell<BridgeState> = RefCell::new(BridgeState::new());
}

#[init]
fn init() {
    ic_cdk::println!("🚀 Initializing Gasless Bridge with advanced state management");
    
    // Initialize first admin
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.add_admin(caller());
        
        s.reserve.add_funds(10_000_000_000_000_000_000);
    });
    
    ic_cdk::println!("✅ Gasless Bridge initialization complete");
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("⚠️ Preparing for canister upgrade");
    // TODO: Serialize state to stable storage
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("🔄 Canister upgrade complete");
    // TODO: Deserialize state from stable storage
}

// === QUOTE GENERATION API ===

#[update]
async fn request_quote(
    amount: u64,
    destination_address: String,
    destination_chain: String,
) -> Result<Quote, String> {
    ic_cdk::println!("📋 Quote request: {} wei to {} on {}", amount, destination_address, destination_chain);
    
    // Validate using our config
    let (min_amount, max_amount, supported_chains) = STATE.with(|state| {
        let s = state.borrow();
        (s.config.min_quote_amount, s.config.max_quote_amount, s.config.supported_chains.clone())
    });
    
    // Input validation
    if amount < min_amount {
        return Err(format!("Amount too small, minimum {} wei", min_amount));
    }
    
    if amount > max_amount {
        return Err(format!("Amount too large, maximum {} wei", max_amount));
    }
    
    if !destination_address.starts_with("0x") || destination_address.len() != 42 {
        return Err("Invalid Ethereum address format".to_string());
    }
    
    if !supported_chains.contains(&destination_chain) {
        return Err(format!("Unsupported chain: {}, supported: {:?}", destination_chain, supported_chains));
    }
    
    // Check reserve capacity
    let can_fulfill = STATE.with(|state| {
        let s = state.borrow();
        s.reserve.can_lock(amount + 5_000_000_000_000_000) // amount + estimated gas
    });
    
    if !can_fulfill {
        return Err("Insufficient reserve capacity, please try a smaller amount".to_string());
    }
    
    // Get advanced gas estimation
    let gas_estimate = match estimate_gas_advanced().await {
        Ok(estimate) => {
            match validate_gas_estimate(&estimate) {
                Ok(_) => estimate,
                Err(e) => return Err(format!("Gas validation failed: {}", e)),
            }
        }
        Err(e) => {
            ic_cdk::println!("⚠️ Gas estimation failed: {}, using fallback", e);
            // Use fallback from gas_estimator
            crate::services::gas_estimator::get_fallback_estimate()
        }
    };
    
    // Generate quote ID
    let quote_id = format!("quote_{}_{}", 
        caller().to_text().chars().take(8).collect::<String>(),
        ic_cdk::api::time() / 1_000_000_000
    );
    
    // Create quote request
    let request = QuoteRequest {
        amount,
        destination_address,
        destination_chain,
    };
    
    // Create full quote using our advanced Quote struct
    let quote = Quote::new(
        quote_id,
        caller(),
        request,
        gas_estimate.total_cost,
        gas_estimate.base_fee,
        gas_estimate.priority_fee,
        15, // 15 minutes validity
    );
    
    // Store quote in our advanced state
    STATE.with(|state| {
        state.borrow_mut().add_quote(quote.clone());
    });
    
    ic_cdk::println!("✅ Generated quote {} - Amount: {} wei, Total cost: {} wei, Expires: {} seconds", 
        quote.id, quote.amount_requested, quote.total_cost, quote.time_remaining());
    
    Ok(quote)
}

#[query]
fn get_quote(quote_id: String) -> Option<Quote> {
    STATE.with(|state| {
        state.borrow().get_quote(&quote_id)
    })
}

#[query]
fn get_user_quotes() -> Vec<Quote> {
    STATE.with(|state| {
        state.borrow().get_quotes_by_user(&caller())
    })
}

// === VALIDATION & ESTIMATION ===

#[update]
async fn estimate_quote_cost(amount: u64) -> Result<String, String> {
    let gas_estimate = estimate_gas_advanced().await?;
    validate_gas_estimate(&gas_estimate)?;
    
    let total_cost = amount + gas_estimate.total_cost;
    
    Ok(format!(
        "💰 Advanced Quote Estimate:\n\
         📊 Requested: {} wei ({:.6} ETH)\n\
         ⛽ Base Fee: {} Gwei\n\
         🚀 Priority Fee: {} Gwei\n\
         🛡️ Safety Margin: {} wei\n\
         💸 Total Cost: {} wei ({:.6} ETH)\n\
         📈 Gas Overhead: {:.3}%",
        amount, amount as f64 / 1e18,
        gas_estimate.base_fee / 1_000_000_000,
        gas_estimate.priority_fee / 1_000_000_000,
        gas_estimate.safety_margin,
        total_cost, total_cost as f64 / 1e18,
        (gas_estimate.total_cost as f64 / amount as f64) * 100.0
    ))
}

// === ADMIN & STATUS ===

#[query]
fn health_check() -> String {
    STATE.with(|state| {
        let s = state.borrow();
        let quote_count = s.quotes.len();
        let available_balance = s.reserve.available_balance;
        let locked_balance = s.reserve.locked_balance;
        
        format!(
            "🟢 Gasless Bridge Status: Healthy\n\
             📊 Active Quotes: {}\n\
             💰 Available Reserve: {:.6} ETH\n\
             🔒 Locked Funds: {:.6} ETH\n\
             ⚠️ Reserve Status: {}",
            quote_count,
            available_balance as f64 / 1e18,
            locked_balance as f64 / 1e18,
            if s.reserve.is_below_critical() { "CRITICAL" }
            else if s.reserve.is_below_warning() { "WARNING" }
            else { "GOOD" }
        )
    })
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ReserveStatus {
    pub balance: u64,
    pub locked: u64,
    pub available: u64,
    pub threshold_warning: u64,
    pub threshold_critical: u64,
}

#[query]
fn get_reserve_status() -> ReserveStatus {
    STATE.with(|state| {
        let reserve = &state.borrow().reserve;
        ReserveStatus {
            balance: reserve.total_balance,
            locked: reserve.locked_balance,
            available: reserve.available_balance,
            threshold_warning: reserve.threshold_warning,
            threshold_critical: reserve.threshold_critical,
        }
    })
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DetailedReserveStatus {
    pub balance: u64,
    pub locked: u64,
    pub available: u64,
    pub threshold_warning: u64,
    pub threshold_critical: u64,
    pub daily_volume: u64,
    pub daily_limit: u64,
    pub pending_withdrawals: u64,
    pub utilization_percent: f64,    // locked / total * 100
    pub health_status: String,       // "GOOD", "WARNING", "CRITICAL"
    pub can_accept_quotes: bool,
    pub last_topup: u64,
}

#[query]
fn get_detailed_reserve_status() -> DetailedReserveStatus {
    STATE.with(|state| {
        let reserve = &state.borrow().reserve;
        let utilization = if reserve.total_balance > 0 {
            (reserve.locked_balance as f64 / reserve.total_balance as f64) * 100.0
        } else {
            0.0
        };
        
        let health_status = if reserve.is_below_critical() {
            "CRITICAL"
        } else if reserve.is_below_warning() {
            "WARNING"
        } else {
            "GOOD"
        };
        
        DetailedReserveStatus {
            balance: reserve.total_balance,
            locked: reserve.locked_balance,
            available: reserve.available_balance,
            threshold_warning: reserve.threshold_warning,
            threshold_critical: reserve.threshold_critical,
            daily_volume: reserve.daily_volume,
            daily_limit: reserve.daily_limit,
            pending_withdrawals: reserve.pending_withdrawals,
            utilization_percent: utilization,
            health_status: health_status.to_string(),
            can_accept_quotes: !reserve.is_below_critical(),
            last_topup: reserve.last_topup,
        }
    })
}

#[query]
fn get_reserve_status_formatted() -> String {
    STATE.with(|state| {
        let reserve = &state.borrow().reserve;
        format!(
            "💰 Reserve Status:\n\
             Total: {:.6} ETH\n\
             Available: {:.6} ETH\n\
             Locked: {:.6} ETH\n\
             Warning Threshold: {:.6} ETH\n\
             Critical Threshold: {:.6} ETH",
            reserve.total_balance as f64 / 1e18,
            reserve.available_balance as f64 / 1e18,
            reserve.locked_balance as f64 / 1e18,
            reserve.threshold_warning as f64 / 1e18,
            reserve.threshold_critical as f64 / 1e18
        )
    })
}

#[update]
fn add_admin(principal: candid::Principal) -> Result<String, String> {
    let caller_principal = caller();
    
    let is_admin = STATE.with(|state| {
        state.borrow().is_admin(&caller_principal)
    });
    
    if !is_admin {
        return Err("Unauthorized: Only admins can add new admins".to_string());
    }
    
    STATE.with(|state| {
        state.borrow_mut().add_admin(principal);
    });
    
    Ok(format!("✅ Admin {} added successfully", principal))
}

#[update]
fn admin_add_reserve_funds(amount_wei: u64) -> Result<String, String> {
    let caller_principal = caller();
    
    let is_admin = STATE.with(|state| {
        state.borrow().is_admin(&caller_principal)
    });
    
    if !is_admin {
        return Err("Unauthorized: Only admins can add reserve funds".to_string());
    }
    
    STATE.with(|state| {
        state.borrow_mut().reserve.add_funds(amount_wei);
    });
    
    Ok(format!("✅ Added {} wei ({:.6} ETH) to reserve", amount_wei, amount_wei as f64 / 1e18))
}

#[update]
fn admin_set_reserve_thresholds(warning_wei: u64, critical_wei: u64) -> Result<String, String> {
    let caller_principal = caller();
    
    let is_admin = STATE.with(|state| {
        state.borrow().is_admin(&caller_principal)
    });
    
    if !is_admin {
        return Err("Unauthorized: Only admins can set thresholds".to_string());
    }
    
    if critical_wei >= warning_wei {
        return Err("Critical threshold must be less than warning threshold".to_string());
    }
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.reserve.threshold_warning = warning_wei;
        s.reserve.threshold_critical = critical_wei;
    });
    
    Ok(format!(
        "✅ Thresholds updated - Warning: {:.6} ETH, Critical: {:.6} ETH",
        warning_wei as f64 / 1e18,
        critical_wei as f64 / 1e18
    ))
}

#[update]
fn admin_set_daily_limit(limit_wei: u64) -> Result<String, String> {
    let caller_principal = caller();
    
    let is_admin = STATE.with(|state| {
        state.borrow().is_admin(&caller_principal)
    });
    
    if !is_admin {
        return Err("Unauthorized: Only admins can set daily limits".to_string());
    }
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.reserve.daily_limit = limit_wei;
    });
    
    Ok(format!("✅ Daily limit set to {} wei ({:.6} ETH)", limit_wei, limit_wei as f64 / 1e18))
}

#[update]
fn admin_emergency_pause() -> Result<String, String> {
    let caller_principal = caller();
    
    let is_admin = STATE.with(|state| {
        state.borrow().is_admin(&caller_principal)
    });
    
    if !is_admin {
        return Err("Unauthorized: Only admins can emergency pause".to_string());
    }
    
    // Set critical threshold very high to effectively pause quote acceptance
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.reserve.threshold_critical = s.reserve.total_balance + 1;
    });
    
    Ok("🚨 EMERGENCY PAUSE ACTIVATED - No new quotes will be accepted".to_string())
}

#[update]
fn admin_emergency_unpause() -> Result<String, String> {
    let caller_principal = caller();
    
    let is_admin = STATE.with(|state| {
        state.borrow().is_admin(&caller_principal)
    });
    
    if !is_admin {
        return Err("Unauthorized: Only admins can unpause".to_string());
    }
    
    // Reset to default critical threshold
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.reserve.threshold_critical = 100_000_000_000_000_000; // 0.1 ETH
    });
    
    Ok("✅ Emergency pause lifted - Quote acceptance resumed".to_string())
}

#[query]
fn get_admin_status() -> Vec<candid::Principal> {
    STATE.with(|state| {
        state.borrow().admins.clone()
    })
}

// === SETTLEMENT LOGIC ===

#[update]
async fn settle_quote(quote_id: String, payment_proof: String) -> Result<Settlement, String> {
    ic_cdk::println!("🔄 Settlement request for quote: {} with proof: {}", quote_id, payment_proof);
    
    let caller_principal = caller();
    
    // 1. QUOTE VALIDATION
    let quote = STATE.with(|state| {
        state.borrow().get_quote(&quote_id)
    }).ok_or("Quote not found")?;
    
    // Check quote ownership
    if quote.user_principal != caller_principal {
        return Err("Unauthorized: Quote belongs to different user".to_string());
    }
    
    // Check quote expiry
    if quote.is_expired() {
        return Err(format!("Quote expired {} seconds ago", -quote.time_remaining()));
    }
    
    // Check quote status
    if !quote.is_valid() {
        return Err(format!("Quote is not valid, status: {:?}", quote.status));
    }
    
    // 2. IDEMPOTENCY CHECK
    let settlement_id = format!("settlement_{}_{}", quote_id, ic_cdk::api::time() / 1_000_000_000);
    
    // Check if quote already settled
    let existing_settlement = STATE.with(|state| {
        let settlements = &state.borrow().settlements;
        settlements.values().find(|s| s.quote_id == quote_id).cloned()
    });
    
    if let Some(existing) = existing_settlement {
        return Err(format!("Quote already settled with settlement ID: {}", existing.id));
    }
    
    // 3. PAYMENT PROOF VALIDATION (simplified for now)
    if payment_proof.is_empty() || payment_proof.len() < 10 {
        return Err("Invalid payment proof format".to_string());
    }
    
    // TODO: In production, verify payment proof against blockchain/ICP ledger
    ic_cdk::println!("💰 Payment proof validation passed (simplified): {}", payment_proof);
    
    // 4. RESERVE FUND LOCKING
    let lock_amount = quote.amount_out + quote.total_cost; // Amount to deliver + gas budget
    
    let lock_result = STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.reserve.lock_funds(lock_amount)
    });
    
    match lock_result {
        Ok(_) => {
            ic_cdk::println!("✅ Successfully locked {} wei from reserve", lock_amount);
        }
        Err(e) => {
            return Err(format!("Failed to lock reserve funds: {}", e));
        }
    }
    
    // 5. CREATE SETTLEMENT
    let settlement = Settlement::new(
        settlement_id.clone(),
        quote_id.clone(),
        caller_principal,
        quote.amount_in,           // Amount user paid
        quote.amount_out,          // Amount to deliver to destination
        payment_proof,
        quote.destination_address.clone(),
        quote.destination_chain.clone(),
        quote.total_cost,          // Gas budget
    );
    
    // 6. UPDATE STATE
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        
        // Mark quote as settled
        if let Some(q) = s.quotes.get_mut(&quote_id) {
            q.mark_settled();
        }
        
        // Store settlement
        s.settlements.insert(settlement_id.clone(), settlement.clone());
    });
    
    ic_cdk::println!("🎉 Settlement {} created successfully for quote {}", settlement_id, quote_id);
    
    Ok(settlement)
}

// Helper function to validate quote expiry
#[query]
fn check_quote_expiry(quote_id: String) -> Result<String, String> {
    let quote = STATE.with(|state| {
        state.borrow().get_quote(&quote_id)
    }).ok_or("Quote not found")?;
    
    if quote.is_expired() {
        return Err(format!("Quote expired {} seconds ago", -quote.time_remaining()));
    }
    
    Ok(format!("Quote valid for {} more seconds", quote.time_remaining()))
}

// Get settlement by ID
#[query]
fn get_settlement(settlement_id: String) -> Option<Settlement> {
    STATE.with(|state| {
        state.borrow().settlements.get(&settlement_id).cloned()
    })
}

// Get all settlements for a user
#[query]
fn get_user_settlements() -> Vec<Settlement> {
    let caller_principal = caller();
    STATE.with(|state| {
        state.borrow()
            .settlements
            .values()
            .filter(|s| s.user_principal == caller_principal)
            .cloned()
            .collect()
    })
}

// Get settlement by quote ID
#[query]
fn get_settlement_by_quote(quote_id: String) -> Option<Settlement> {
    STATE.with(|state| {
        state.borrow()
            .settlements
            .values()
            .find(|s| s.quote_id == quote_id)
            .cloned()
    })
}

#[update]
fn add_test_reserve_funds() -> String {
    // Quick function to add test funds (no auth check for development)
    let amount = 5_000_000_000_000_000_000u64; // 5 ETH
    
    STATE.with(|state| {
        state.borrow_mut().reserve.add_funds(amount);
    });
    
    format!("✅ Added {} wei ({:.6} ETH) to reserve for testing", amount, amount as f64 / 1e18)
}

// === RESERVE MONITORING & ALERTS ===

#[query]
fn check_reserve_health() -> String {
    STATE.with(|state| {
        let reserve = &state.borrow().reserve;
        let utilization = if reserve.total_balance > 0 {
            (reserve.locked_balance as f64 / reserve.total_balance as f64) * 100.0
        } else {
            0.0
        };
        
        let mut alerts = Vec::new();
        
        if reserve.is_below_critical() {
            alerts.push("🚨 CRITICAL: Reserve below critical threshold");
        } else if reserve.is_below_warning() {
            alerts.push("⚠️ WARNING: Reserve below warning threshold");
        }
        
        if utilization > 80.0 {
            alerts.push("⚠️ HIGH UTILIZATION: >80% of reserve locked");
        }
        
        if reserve.daily_volume > reserve.daily_limit * 90 / 100 {
            alerts.push("⚠️ DAILY LIMIT: >90% of daily volume used");
        }
        
        if alerts.is_empty() {
            format!(
                "✅ Reserve Health: GOOD\n\
                 💰 Available: {:.6} ETH\n\
                 📊 Utilization: {:.1}%\n\
                 📈 Daily Volume: {:.6} ETH",
                reserve.available_balance as f64 / 1e18,
                utilization,
                reserve.daily_volume as f64 / 1e18
            )
        } else {
            format!(
                "⚠️ Reserve Alerts:\n{}\n\n\
                 💰 Available: {:.6} ETH\n\
                 📊 Utilization: {:.1}%\n\
                 📈 Daily Volume: {:.6} ETH",
                alerts.join("\n"),
                reserve.available_balance as f64 / 1e18,
                utilization,
                reserve.daily_volume as f64 / 1e18
            )
        }
    })
}

#[query]
fn get_reserve_utilization() -> f64 {
    STATE.with(|state| {
        let reserve = &state.borrow().reserve;
        if reserve.total_balance > 0 {
            (reserve.locked_balance as f64 / reserve.total_balance as f64) * 100.0
        } else {
            0.0
        }
    })
}

#[query]
fn can_accept_new_quotes() -> bool {
    STATE.with(|state| {
        let reserve = &state.borrow().reserve;
        !reserve.is_below_critical()
    })
}

#[query]
fn estimate_reserve_runway() -> String {
    STATE.with(|state| {
        let reserve = &state.borrow().reserve;
        
        if reserve.daily_volume == 0 {
            return "📊 Reserve Runway: No daily volume data".to_string();
        }
        
        let avg_daily_consumption = reserve.daily_volume; // Simplified - would calculate actual average
        let days_remaining = if avg_daily_consumption > 0 {
            reserve.available_balance / avg_daily_consumption
        } else {
            0
        };
        
        if days_remaining > 30 {
            format!("✅ Reserve Runway: {}+ days (healthy)", days_remaining)
        } else if days_remaining > 7 {
            format!("⚠️ Reserve Runway: {} days (monitor)", days_remaining)
        } else {
            format!("🚨 Reserve Runway: {} days (urgent topup needed)", days_remaining)
        }
    })
}

// === TESTING & DEVELOPMENT ===

#[update]
async fn test_base_rpc() -> String {
    match estimate_gas_advanced().await {
        Ok(estimate) => {
            format!("✅ Base Sepolia RPC connected! Gas estimate: {} wei", estimate.total_cost)
        }
        Err(e) => format!("❌ RPC test failed: {}", e)
    }
}

#[update]
async fn test_gas_estimation() -> String {
    match estimate_gas_advanced().await {
        Ok(estimate) => {
            format!(
                "✅ Advanced Gas Estimation Test:\n\
                 Base Fee: {} Gwei\n\
                 Priority Fee: {} Gwei\n\
                 Max Fee: {} Gwei\n\
                 Gas Limit: {}\n\
                 Total Cost: {} wei\n\
                 Safety Margin: {} wei",
                estimate.base_fee / 1_000_000_000,
                estimate.priority_fee / 1_000_000_000,
                estimate.max_fee_per_gas / 1_000_000_000,
                estimate.gas_limit,
                estimate.total_cost,
                estimate.safety_margin
            )
        }
        Err(e) => format!("❌ Gas estimation test failed: {}", e)
    }
}

// === SETTLEMENT TESTING ===

#[update]
async fn test_settlement_flow() -> String {
    ic_cdk::println!("🧪 Testing complete settlement flow");
    
    // 1. Create a test quote
    let test_quote = match request_quote(
        1_000_000_000_000_000_000, // 1 ETH
        "0x742d35Cc6635C0532925a3b8D0A4C1234b8DbD5c".to_string(),
        "Base Sepolia".to_string(),
    ).await {
        Ok(quote) => quote,
        Err(e) => return format!("❌ Quote creation failed: {}", e),
    };
    
    ic_cdk::println!("✅ Test quote created: {}", test_quote.id);
    
    // 2. Wait a moment and then settle
    let settlement_result = settle_quote(
        test_quote.id.clone(),
        "test_payment_proof_tx_hash_123456789abcdef".to_string(),
    ).await;
    
    match settlement_result {
        Ok(settlement) => {
            format!(
                "✅ Settlement flow test completed!\n\
                 📋 Quote ID: {}\n\
                 🔒 Settlement ID: {}\n\
                 💰 Amount Locked: {} wei\n\
                 📊 Settlement Status: {:?}",
                test_quote.id,
                settlement.id,
                settlement.locked_reserve,
                settlement.status
            )
        }
        Err(e) => format!("❌ Settlement failed: {}", e),
    }
}

#[query]
fn get_settlement_statistics() -> String {
    STATE.with(|state| {
        let s = state.borrow();
        let total_settlements = s.settlements.len();
        let pending_settlements = s.settlements.values()
            .filter(|settlement| settlement.is_pending())
            .count();
        let completed_settlements = s.settlements.values()
            .filter(|settlement| settlement.is_completed())
            .count();
        
        let total_locked = s.reserve.locked_balance;
        
        format!(
            "📊 Settlement Statistics:\n\
             Total Settlements: {}\n\
             Pending: {}\n\
             Completed: {}\n\
             💰 Total Locked: {:.6} ETH",
            total_settlements,
            pending_settlements,
            completed_settlements,
            total_locked as f64 / 1e18
        )
    })
}

ic_cdk::export_candid!();