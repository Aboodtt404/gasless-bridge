use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Settlement {
    pub id: String,
    pub quote_id: String,
    pub user_principal: candid::Principal,
    pub amount: u64,                   // Amount to deliver to destination
    pub destination_address: String,   // Where to send funds
    pub destination_chain: String,     // Target blockchain
    pub payment_proof: String,         // Transaction hash or payment proof
    pub created_at: u64,              // When settlement was created
    pub status: SettlementStatus,
    pub gas_used: Option<u64>,        // Gas actually used
    pub transaction_hash: Option<String>, // Ethereum transaction hash
    pub retry_count: u32,             // Number of execution attempts
    pub last_error: Option<String>,   // Error details if failed
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
        _amount_paid: u64,
        amount_to_deliver: u64,
        payment_proof: String,
        destination_address: String,
        destination_chain: String,
        _gas_budget: u64,
    ) -> Self {
        Settlement {
            id,
            quote_id,
            user_principal,
            amount: amount_to_deliver,
            destination_address,
            destination_chain,
            payment_proof,
            created_at: ic_cdk::api::time() / 1_000_000_000,
            status: SettlementStatus::Pending,
            gas_used: None,
            transaction_hash: None,
            retry_count: 0,
            last_error: None,
        }
    }
    
    pub fn mark_executing(&mut self) {
        self.status = SettlementStatus::Executing;
    }
    
    pub fn mark_completed(&mut self, gas_used: u64, transaction_hash: String) {
        self.status = SettlementStatus::Completed;
        self.gas_used = Some(gas_used);
        self.transaction_hash = Some(transaction_hash);
        ic_cdk::println!("Settlement {} completed, gas used: {}", self.id, gas_used);
    }
    
    pub fn mark_failed(&mut self, reason: String, retry_count: u32) {
        self.status = SettlementStatus::Failed;
        self.last_error = Some(reason);
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