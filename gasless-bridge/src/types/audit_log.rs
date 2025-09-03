use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use ic_stable_structures::storable::Storable;
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AuditLogEntry {
    pub id: String,
    pub event_type: String,
    pub details: String,
    pub user_principal: Option<Principal>,
    pub amount_eth: Option<u64>,
    pub amount_icp: Option<u64>,
    pub transaction_hash: Option<String>,
    pub timestamp: u64,
}

// Implement Storable for AuditLogEntry
impl Storable for AuditLogEntry {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
    
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }
}
