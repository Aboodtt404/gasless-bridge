use candid::{CandidType, Deserialize};
use std::collections::HashMap;
use crate::types::{Quote, Settlement, Transfer};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BridgeState {
    pub quotes: HashMap<String, Quote>,
    pub settlements: HashMap<String, Settlement>,
    pub transfers: HashMap<String, Transfer>,
    pub reserve: ReserveState,
    pub admins: Vec<candid::Principal>,
    pub config: BridgeConfig,
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