use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Quote {
    pub id: String,
    pub user_principal: candid::Principal,
    pub amount_requested: u64,         // Amount user wants to receive
    pub amount_in: u64,               // Total amount user needs to pay
    pub amount_out: u64,              // Exact amount delivered (= amount_requested)
    pub total_cost: u64,              // Total sponsor cost (gas + fees)
    pub destination_address: String,   // Where funds go on destination chain
    pub source_chain: String,         // Source blockchain (e.g., "ICP")
    pub destination_chain: String,     // Destination blockchain (e.g., "Base Sepolia")
    pub created_at: u64,              // Unix timestamp when quote created
    pub expires_at: u64,              // Unix timestamp when quote expires
    pub gas_estimate: u64,            // Estimated gas cost in wei
    pub base_fee: u64,                // EIP-1559 base fee per gas
    pub priority_fee: u64,            // EIP-1559 priority fee per gas
    pub max_fee_per_gas: u64,         // Maximum fee per gas willing to pay
    pub safety_margin: u64,           // Additional buffer for gas price volatility
    pub status: QuoteStatus,          // Current status of the quote
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteStatus {
    Active,    // Quote is valid and can be settled
    Settled,   // Quote has been settled (payment received)
    Expired,   // Quote has expired
    Failed,    // Quote failed for some reason
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct QuoteRequest {
    pub amount: u64,
    pub destination_address: String,
    pub destination_chain: String,
}

impl Quote {
    pub fn new(
        id: String,
        user_principal: candid::Principal,
        request: QuoteRequest,
        gas_estimate: u64,
        base_fee: u64,
        priority_fee: u64,
        validity_minutes: u64,
    ) -> Self {
        let now = ic_cdk::api::time() / 1_000_000_000; // Convert nanoseconds to seconds
        let expires_at = now + (validity_minutes * 60);
        
        // Calculate fees with safety margin
        let safety_margin = gas_estimate * 20 / 100; // 20% safety margin
        let _total_cost = gas_estimate + safety_margin; // Unused in gasless model
        let max_fee_per_gas = base_fee + priority_fee;
        
        Quote {
            id,
            user_principal,
            amount_in: request.amount,              // 🚀 GASLESS: User pays EXACTLY what they specify!
            amount_out: request.amount,             // 🎯 Receiver gets EXACTLY what user intended!
            amount_requested: request.amount,
            total_cost: 0,                          // 🌟 ZERO COST TO USER - Bridge subsidizes everything!
            gas_estimate,
            destination_address: request.destination_address,
            source_chain: "ICP".to_string(),
            destination_chain: request.destination_chain,
            created_at: now,
            expires_at,
            base_fee,
            priority_fee,
            max_fee_per_gas,
            safety_margin,
            status: QuoteStatus::Active,
        }
    }
    
    pub fn is_valid(&self) -> bool {
        let now = ic_cdk::api::time() / 1_000_000_000;
        matches!(self.status, QuoteStatus::Active) && now < self.expires_at
    }
    
    pub fn is_expired(&self) -> bool {
        let now = ic_cdk::api::time() / 1_000_000_000;
        now >= self.expires_at
    }
    
    pub fn mark_settled(&mut self) {
        self.status = QuoteStatus::Settled;
    }
    
    pub fn mark_expired(&mut self) {
        self.status = QuoteStatus::Expired;
    }
    
    pub fn time_remaining(&self) -> i64 {
        let now = ic_cdk::api::time() / 1_000_000_000;
        (self.expires_at as i64) - (now as i64)
    }
    
    /// Get the gas cost that the bridge will subsidize (for internal accounting)
    /// This is the revolutionary part - bridge covers ALL gas costs!
    pub fn get_bridge_subsidy(&self) -> u64 {
        // Bridge pays: gas_estimate already contains the total cost (gas_limit * max_fee_per_gas + safety_margin)
        // No need to multiply again - that was the bug!
        self.gas_estimate
    }
    
    /// Get total amount bridge needs to lock (delivery amount + gas subsidy)
    /// This is what the bridge reserves need to cover
    pub fn get_total_bridge_cost(&self) -> u64 {
        self.amount_out + self.get_bridge_subsidy()
    }
    
    /// Check if this quote uses the gasless model (zero cost to user)
    pub fn is_gasless(&self) -> bool {
        self.total_cost == 0
    }
    
    /// Get user-friendly description of the gasless benefit
    pub fn get_gasless_savings(&self) -> String {
        let gas_savings = self.get_bridge_subsidy();
        format!(
            "💰 Gas Savings: {:.6} ETH\n\
            🎯 You Pay: {:.6} ETH\n\
            🎁 You Get: {:.6} ETH delivered\n\
            🚀 Bridge Covers: {:.6} ETH in gas fees",
            gas_savings as f64 / 1e18,
            self.amount_in as f64 / 1e18,
            self.amount_out as f64 / 1e18,
            gas_savings as f64 / 1e18
        )
    }
}