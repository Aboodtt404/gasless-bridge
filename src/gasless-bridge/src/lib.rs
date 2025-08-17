mod types;
mod utils;
mod services;
mod handlers;
mod storage;

use candid::export_candid;
use ic_cdk::{caller, init, post_upgrade, pre_upgrade, query, update};

// Re-export types for external use
pub use types::*;
pub use utils::config::BridgeConfig;

// Import modules
use handlers::*;
use storage::state::STATE;
use utils::logging::Logger;

#[init]
fn init() {
    Logger::info("Initializing Gasless Bridge canister");
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.initialize_with_config(BridgeConfig::development());
        s.add_admin(caller());
    });
    
    Logger::info("Gasless Bridge initialization complete");
}

#[pre_upgrade]
fn pre_upgrade() {
    Logger::info("Preparing for canister upgrade");
    // State is automatically preserved with ic-stable-structures
}

#[post_upgrade]
fn post_upgrade() {
    Logger::info("Canister upgrade complete");
}

// Health and admin functions
#[query]
fn health_check() -> String {
    handlers::admin_handler::health_check()
}

#[query]
fn get_config() -> BridgeConfig {
    handlers::admin_handler::get_config()
}

#[update]
fn update_config(config: BridgeConfig) -> Result<String, String> {
    handlers::admin_handler::update_config(config)
}

// RPC testing functions
#[update]
async fn test_base_rpc() -> String {
    handlers::admin_handler::test_base_rpc().await
}

#[update]
async fn estimate_gas(to_address: String, amount: u64) -> String {
    handlers::admin_handler::estimate_gas(to_address, amount).await
}

// Export Candid interface
export_candid!();
