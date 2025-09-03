use candid::{CandidType, Deserialize};
use std::collections::HashMap;
use crate::types::{Quote, Settlement, Transfer};
use crate::services::chain_key_tokens::ChainKeyTokenService;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BridgeState {
    pub quotes: HashMap<String, Quote>,
    pub settlements: HashMap<String, Settlement>,
    pub transfers: HashMap<String, Transfer>,
    pub reserve: ReserveState,
    pub admins: Vec<candid::Principal>,
    pub config: BridgeConfig,
    pub chain_key_service: ChainKeyTokenService, // 🪙 Chain-key token service
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ReserveState {
    pub total_balance: u64,           // Total ETH in reserve (wei)
    pub locked_balance: u64,          // Currently locked for settlements (wei)
    pub available_balance: u64,       // Available for new settlements (wei)
    pub threshold_warning: u64,       // Warn when reserve below this (wei)
    pub threshold_critical: u64,      // Stop accepting quotes below this (wei)
    pub daily_volume: u64,           // Volume processed today (wei)
    pub daily_limit: u64,            // Maximum daily volume (wei)
    pub last_topup: u64,             // Last time reserve was topped up
    pub pending_withdrawals: u64,     // Funds pending withdrawal
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BridgeConfig {
    pub max_quote_amount: u64,        // Maximum single quote amount
    pub min_quote_amount: u64,        // Minimum single quote amount
    pub quote_validity_minutes: u64,  // How long quotes remain valid
    pub max_gas_price: u64,          // Maximum gas price we'll pay
    pub safety_margin_percent: u32,  // Safety margin for gas estimates
    pub supported_chains: Vec<String>, // Supported destination chains
}

impl BridgeState {
    pub fn new() -> Self {
        BridgeState {
            quotes: HashMap::new(),
            settlements: HashMap::new(),
            transfers: HashMap::new(),
            reserve: ReserveState::new(),
            admins: Vec::new(),
            config: BridgeConfig::default(),
            chain_key_service: ChainKeyTokenService::new(), // Initialize the new field
        }
    }
    
    // Quote management
    pub fn add_quote(&mut self, quote: Quote) {
        self.quotes.insert(quote.id.clone(), quote);
    }
    
    pub fn get_quote(&self, quote_id: &str) -> Option<Quote> {
        self.quotes.get(quote_id).cloned()
    }
    
    pub fn get_quotes_by_user(&self, user_principal: &candid::Principal) -> Vec<Quote> {
        self.quotes
            .values()
            .filter(|quote| &quote.user_principal == user_principal)
            .cloned()
            .collect()
    }
    
    // Settlement management
    pub fn add_settlement(&mut self, settlement: Settlement) {
        self.settlements.insert(settlement.id.clone(), settlement);
    }
    
    pub fn get_settlement(&self, settlement_id: &str) -> Option<Settlement> {
        self.settlements.get(settlement_id).cloned()
    }
    
    pub fn get_settlements_by_user(&self, user_principal: &candid::Principal) -> Vec<Settlement> {
        self.settlements
            .values()
            .filter(|settlement| &settlement.user_principal == user_principal)
            .cloned()
            .collect()
    }
    
    // Admin management
    pub fn add_admin(&mut self, principal: candid::Principal) {
        if !self.admins.contains(&principal) {
            self.admins.push(principal);
        }
    }
    
    pub fn is_admin(&self, principal: &candid::Principal) -> bool {
        self.admins.contains(principal)
    }
}

impl ReserveState {
    pub fn new() -> Self {
        ReserveState {
            total_balance: 0,
            locked_balance: 0,
            available_balance: 0,
            threshold_warning: 500_000_000_000_000_000,  // 0.5 ETH
            threshold_critical: 100_000_000_000_000_000, // 0.1 ETH
            daily_volume: 0,
            daily_limit: 10_000_000_000_000_000_000,    // 10 ETH per day
            last_topup: 0,
            pending_withdrawals: 0,
        }
    }
    
    pub fn can_lock(&self, amount: u64) -> bool {
        self.available_balance >= amount && 
        self.available_balance - amount >= self.threshold_critical
    }
    
    pub fn lock_funds(&mut self, amount: u64) -> Result<(), String> {
        if !self.can_lock(amount) {
            return Err("Insufficient reserve funds".to_string());
        }
        
        self.locked_balance += amount;
        self.available_balance = self.total_balance.saturating_sub(self.locked_balance);
        Ok(())
    }
    
    pub fn unlock_funds(&mut self, amount: u64) {
        self.locked_balance = self.locked_balance.saturating_sub(amount);
        self.available_balance = self.total_balance.saturating_sub(self.locked_balance);
    }
    
    pub fn add_funds(&mut self, amount: u64) {
        self.total_balance += amount;
        self.available_balance = self.total_balance.saturating_sub(self.locked_balance);
        self.last_topup = ic_cdk::api::time() / 1_000_000_000;
    }
    
    pub fn is_below_warning(&self) -> bool {
        self.available_balance < self.threshold_warning
    }
    
    pub fn is_below_critical(&self) -> bool {
        self.available_balance < self.threshold_critical
    }
    
    /// Lock funds for gasless delivery (amount + gas subsidy)
    /// This is the key function for the gasless model!
    pub fn lock_gasless_funds(&mut self, delivery_amount: u64, gas_subsidy: u64) -> Result<(), String> {
        let total_required = delivery_amount + gas_subsidy; // Bridge pays both!
        
        if !self.can_lock(total_required) {
            return Err(format!(
                "Insufficient reserve for gasless delivery. Need: {:.6} ETH, Available: {:.6} ETH",
                total_required as f64 / 1e18,
                self.available_balance as f64 / 1e18
            ));
        }
        
        self.locked_balance += total_required;
        self.available_balance = self.total_balance.saturating_sub(self.locked_balance);
        
        // Track daily gas subsidies for analytics
        self.daily_volume += gas_subsidy;
        
        ic_cdk::println!(
            "🚀 Gasless funds locked! Delivery: {:.6} ETH, Gas Subsidy: {:.6} ETH, Total: {:.6} ETH",
            delivery_amount as f64 / 1e18,
            gas_subsidy as f64 / 1e18,
            total_required as f64 / 1e18
        );
        
        Ok(())
    }
    
    /// Check if bridge can afford to subsidize a gasless transaction
    pub fn can_subsidize_gasless(&self, delivery_amount: u64, gas_subsidy: u64) -> bool {
        let total_cost = delivery_amount + gas_subsidy;
        self.can_lock(total_cost)
    }
    
    /// Get daily gas subsidy spending (for profitability analytics)
    pub fn get_daily_gas_subsidy(&self) -> u64 {
        self.daily_volume // We're reusing daily_volume to track gas subsidies
    }
    
    /// Calculate gasless bridge profitability metrics
    pub fn get_gasless_metrics(&self) -> String {
        format!(
            "💰 Gasless Bridge Economics:\n\
            📊 Daily Gas Subsidies: {:.6} ETH\n\
            💪 Reserve Capacity: {:.6} ETH\n\
            🎯 Gasless Transactions Affordable: ~{} more\n\
            📈 Operational Efficiency: {}%",
            self.daily_volume as f64 / 1e18,
            self.available_balance as f64 / 1e18,
            if self.daily_volume > 0 { self.available_balance / (self.daily_volume / 100) } else { 0 },
            if self.total_balance > 0 { (self.available_balance * 100) / self.total_balance } else { 0 }
        )
    }
}

impl Default for BridgeConfig {
    fn default() -> Self {
        BridgeConfig {
            max_quote_amount: 1_000_000_000_000_000_000, // 1 ETH
            min_quote_amount: 1_000_000_000_000_000,     // 0.001 ETH
            quote_validity_minutes: 15,                   // 15 minutes
            max_gas_price: 200_000_000_000,              // 200 Gwei
            safety_margin_percent: 20,                   // 20% safety margin
            supported_chains: vec!["Base Sepolia".to_string()],
        }
    }
}