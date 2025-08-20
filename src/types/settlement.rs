use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Settlement {
    pub id: String,
    pub quote_id: String,
    pub user_principal: candid::Principal,
    pub amount_paid: u64,              // Amount user actually paid
    pub amount_to_deliver: u64,        // Amount to deliver to destination
    pub payment_proof: String,         // Transaction hash or payment proof
    pub destination_address: String,   // Where to send funds
    pub destination_chain: String,     // Target blockchain
    pub created_at: u64,              // When settlement was created
    pub locked_reserve: u64,          // Amount locked from reserve
    pub gas_budget: u64,              // Gas budget for execution
    pub retry_count: u32,             // Number of execution attempts
    pub status: SettlementStatus,
    pub error_message: Option<String>, // Error details if failed
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlementStatus {
    Pending,    // Settlement created, waiting for execution
    Executing,  // Transaction being broadcast
    Completed,  // Successfully delivered
    Failed,     // Failed after retries
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SettlementRequest {
    pub quote_id: String,
    pub payment_proof: String,
}

impl Settlement {
    pub fn new(
        id: String,
        quote_id: String,
        user_principal: candid::Principal,
        amount_paid: u64,
        amount_to_deliver: u64,
        payment_proof: String,
        destination_address: String,
        destination_chain: String,
        gas_budget: u64,
    ) -> Self {
        Settlement {
            id,
            quote_id,
            user_principal,
            amount_paid,
            amount_to_deliver,
            payment_proof,
            destination_address,
            destination_chain,
            created_at: ic_cdk::api::time() / 1_000_000_000,
            locked_reserve: amount_to_deliver + gas_budget,
            gas_budget,
            retry_count: 0,
            status: SettlementStatus::Pending,
            error_message: None,
        }
    }
    
    pub fn mark_executing(&mut self) {
        self.status = SettlementStatus::Executing;
    }
    
    pub fn mark_completed(&mut self, gas_used: u64) {
        self.status = SettlementStatus::Completed;
        // Note: In production, update reserve with unused gas
        let unused_gas = self.gas_budget.saturating_sub(gas_used);
        ic_cdk::println!("Settlement {} completed, unused gas: {}", self.id, unused_gas);
    }
    
    pub fn mark_failed(&mut self, reason: String, retry_count: u32) {
        self.status = SettlementStatus::Failed;
        self.error_message = Some(reason);
        self.retry_count = retry_count;
    }
    
    pub fn is_pending(&self) -> bool {
        matches!(self.status, SettlementStatus::Pending)
    }
    
    pub fn is_completed(&self) -> bool {
        matches!(self.status, SettlementStatus::Completed)
    }
    
    pub fn can_retry(&self) -> bool {
        matches!(self.status, SettlementStatus::Failed) && self.retry_count < 3
    }
}