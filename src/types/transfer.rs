use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Transfer {
    pub id: String,
    pub settlement_id: String,
    pub transaction_hash: Option<String>, // Blockchain transaction hash
    pub to_address: String,              // Recipient address
    pub amount: u64,                     // Amount transferred (in wei)
    pub gas_used: Option<u64>,           // Actual gas used
    pub gas_price: Option<u64>,          // Actual gas price paid
    pub block_number: Option<u64>,       // Block number where tx was mined
    pub confirmations: u32,              // Number of confirmations
    pub created_at: u64,                // When transfer was initiated
    pub completed_at: Option<u64>,      // When transfer was confirmed
    pub status: TransferStatus,
    pub receipt: Option<TransferReceipt>, // Full transaction receipt
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferStatus {
    Pending,     // Transaction created but not yet broadcast
    Broadcast,   // Transaction sent to network
    Confirmed,   // Transaction confirmed on blockchain
    Failed,      // Transaction failed
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferReceipt {
    pub transaction_hash: String,
    pub block_number: u64,
    pub gas_used: u64,
    pub effective_gas_price: u64,
    pub status: u64,                    // 1 = success, 0 = failure
    pub logs: Vec<String>,              // Event logs (simplified)
}

impl Transfer {
    pub fn new(id: String, settlement_id: String) -> Self {
        Transfer {
            id,
            settlement_id,
            transaction_hash: None,
            to_address: String::new(),
            amount: 0,
            gas_used: None,
            gas_price: None,
            block_number: None,
            confirmations: 0,
            created_at: ic_cdk::api::time() / 1_000_000_000,
            completed_at: None,
            status: TransferStatus::Pending,
            receipt: None,
        }
    }
    
    pub fn set_transaction_hash(&mut self, tx_hash: String) {
        self.transaction_hash = Some(tx_hash);
        self.status = TransferStatus::Broadcast;
    }
    
    pub fn mark_confirmed(&mut self, receipt: TransferReceipt) {
        self.status = TransferStatus::Confirmed;
        self.completed_at = Some(ic_cdk::api::time() / 1_000_000_000);
        self.gas_used = Some(receipt.gas_used);
        self.gas_price = Some(receipt.effective_gas_price);
        self.block_number = Some(receipt.block_number);
        
        self.transaction_hash = Some(receipt.transaction_hash.clone());
        
        self.receipt = Some(receipt);
    }
    
    pub fn mark_failed(&mut self, reason: String, error_code: Option<i32>) {
        self.status = TransferStatus::Failed;
        ic_cdk::println!("Transfer {} failed: {} (code: {:?})", self.id, reason, error_code);
    }
    
    pub fn update_confirmations(&mut self, confirmations: u32) {
        self.confirmations = confirmations;
    }
    
    pub fn is_pending(&self) -> bool {
        matches!(self.status, TransferStatus::Pending)
    }
}