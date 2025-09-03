use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use ic_stable_structures::storable::Storable;
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UserTransaction {
    pub id: String,
    pub user_principal: Principal,
    pub amount_icp: u64,
    pub amount_eth: u64,
    pub destination_address: String,
    pub destination_chain: String,
    pub status: TransactionStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub transaction_hash: Option<String>,
    pub gas_sponsored: u64,
    pub icp_payment_id: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransactionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
}

// Implement Storable for UserTransaction
impl Storable for UserTransaction {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
    
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }
}

// Implement Storable for TransactionStatus
impl Storable for TransactionStatus {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
    
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }
}
