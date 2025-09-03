use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use ic_stable_structures::storable::Storable;
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct IcpPayment {
    pub amount_e8s: u64,
    pub user_principal: Principal,
    pub payment_id: String,
    pub timestamp: u64,
    pub status: PaymentStatus,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum PaymentStatus {
    Pending,
    Confirmed,
    Failed,
}

// Implement Storable for IcpPayment
impl Storable for IcpPayment {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
    
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }
}

// Implement Storable for PaymentStatus
impl Storable for PaymentStatus {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
    
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }
}
