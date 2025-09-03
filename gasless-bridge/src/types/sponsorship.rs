use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SponsorshipStatus {
    pub can_sponsor: bool,
    pub estimated_cost_icp: u64,
    pub estimated_cost_eth: u64,
    pub gas_coverage: String,
    pub reserve_health: String,
}
