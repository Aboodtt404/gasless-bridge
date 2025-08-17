use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub network: NetworkConfig,
    pub rpc: RpcConfig,
    pub gas: GasConfig,
    pub reserves: ReserveConfig,
    pub quotes: QuoteConfig,
}

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub name: String,
    pub chain_id: u64,
    pub confirmation_blocks: u64,
}

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct RpcConfig {
    pub primary_url: String,
    pub fallback_url: String,
    pub public_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct GasConfig {
    pub safety_margin_percent: u32,
    pub max_priority_fee_gwei: f64,
    pub max_base_fee_gwei: f64,
}

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct ReserveConfig {
    pub minimum_balance_eth: f64,
    pub warning_threshold_eth: f64,
    pub critical_threshold_eth: f64,
}

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct QuoteConfig {
    pub validity_duration_minutes: u64,
    pub max_amount_eth: f64,
    pub min_amount_eth: f64,
}

impl BridgeConfig {
    pub fn development() -> Self {
        Self {
            network: NetworkConfig {
                name: "Base Sepolia".to_string(),
                chain_id: 84532,
                confirmation_blocks: 1,
            },
            rpc: RpcConfig {
                primary_url: "https://base-sepolia.g.alchemy.com/v2/keg5qPpXALLYHHhXJBKuL".to_string(),
                fallback_url: "https://fluent-empty-meadow.base-sepolia.quiknode.pro/a9ebb15ae2a849c069efd78d24022e2e60c1be1f/".to_string(),
                public_url: "https://sepolia.base.org".to_string(),
                timeout_seconds: 30,
                max_retries: 3,
            },
            gas: GasConfig {
                safety_margin_percent: 20,
                max_priority_fee_gwei: 2.0,
                max_base_fee_gwei: 100.0,
            },
            reserves: ReserveConfig {
                minimum_balance_eth: 0.01,
                warning_threshold_eth: 0.05,
                critical_threshold_eth: 0.02,
            },
            quotes: QuoteConfig {
                validity_duration_minutes: 10,
                max_amount_eth: 1.0,
                min_amount_eth: 0.001,
            },
        }
    }
}
