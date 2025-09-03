use candid::{Principal, CandidType, Deserialize};
use ic_cdk::api::call;
use crate::services::price_feeds::PriceFeedService;

use std::collections::HashMap;

// ICP Ledger Canister ID (mainnet)
const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AccountBalanceArgs {
    pub account: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub to: String,
    pub fee: u64,
    pub amount: u64,
    pub memo: u64,
    pub from_subaccount: Option<[u8; 32]>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferResult {
    Ok(u64), // Block index
    Err(TransferError),
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferError {
    pub message: String,
    pub kind: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AccountBalance {
    pub e8s: u64,
}

// Professional ICP Ledger Service
pub struct IcpLedgerService;

impl IcpLedgerService {
    /// Get ICP ledger canister principal
    pub fn get_ledger_canister() -> Principal {
        Principal::from_text(ICP_LEDGER_CANISTER_ID)
            .expect("Invalid ICP ledger canister ID")
    }

    /// Get account balance in e8s (smallest ICP unit)
    pub async fn get_account_balance(account: &str) -> Result<u64, String> {
        let ledger_canister = Self::get_ledger_canister();
        
        let args = AccountBalanceArgs {
            account: account.to_string(),
        };

        match call::call::<(AccountBalanceArgs,), (AccountBalance,)>(ledger_canister, "account_balance", (args,)).await {
            Ok((balance,)) => Ok(balance.e8s),
            Err(e) => Err(format!("Failed to get account balance: {:?}", e)),
        }
    }

    /// Transfer ICP tokens
    pub async fn transfer_icp(
        to: &str,
        amount_e8s: u64,
        memo: u64,
        from_subaccount: Option<[u8; 32]>,
    ) -> Result<u64, String> {
        let ledger_canister = Self::get_ledger_canister();
        
        let args = TransferArgs {
            to: to.to_string(),
            fee: 10_000, // Standard ICP transfer fee (0.0001 ICP)
            amount: amount_e8s,
            memo,
            from_subaccount,
            created_at_time: Some(ic_cdk::api::time()),
        };

        match call::call::<(TransferArgs,), (TransferResult,)>(ledger_canister, "transfer", (args,)).await {
            Ok((result,)) => match result {
                TransferResult::Ok(block_index) => Ok(block_index),
                TransferResult::Err(e) => Err(format!("Transfer failed: {} - {}", e.kind, e.message)),
            },
            Err(e) => Err(format!("Failed to call transfer: {:?}", e)),
        }
    }

    /// Get current ICP price in USD (using real price feeds)
    pub async fn get_icp_price_usd() -> Result<f64, String> {
        PriceFeedService::get_icp_price_with_fallback().await
    }

    /// Get current ETH price in USD (using real price feeds)
    pub async fn get_eth_price_usd() -> Result<f64, String> {
        PriceFeedService::get_eth_price_with_fallback().await
    }

    /// Calculate ICP cost for given ETH amount
    pub async fn calculate_icp_cost_for_eth(eth_amount: u64) -> Result<u64, String> {
        let eth_amount_f64 = eth_amount as f64 / 1e18; // Convert wei to ETH
        
        let icp_price = Self::get_icp_price_usd().await?;
        let eth_price = Self::get_eth_price_usd().await?;
        
        // Calculate: (ETH_amount * ETH_price) / ICP_price
        let icp_amount = (eth_amount_f64 * eth_price) / icp_price;
        
        // Convert to e8s (smallest ICP unit)
        let icp_e8s = (icp_amount * 1e8) as u64;
        
        ic_cdk::println!("💰 Price conversion: {} ETH (${:.2}) = {:.6} ICP ({} e8s)", 
            eth_amount_f64, 
            eth_amount_f64 * eth_price,
            icp_amount,
            icp_e8s
        );
        
        Ok(icp_e8s)
    }

    /// Calculate ETH amount for given ICP amount
    pub async fn calculate_eth_amount_for_icp(icp_e8s: u64) -> Result<u64, String> {
        let icp_amount = icp_e8s as f64 / 1e8; // Convert e8s to ICP
        
        let icp_price = Self::get_icp_price_usd().await?;
        let eth_price = Self::get_eth_price_usd().await?;
        
        // Calculate: (ICP_amount * ICP_price) / ETH_price
        let eth_amount = (icp_amount * icp_price) / eth_price;
        
        // Convert to wei (smallest ETH unit)
        let eth_wei = (eth_amount * 1e18) as u64;
        
        Ok(eth_wei)
    }

    /// Get conversion rate (ICP per ETH) using real price feeds
    pub async fn get_conversion_rate() -> Result<f64, String> {
        PriceFeedService::get_conversion_rate().await
    }

    /// Validate ICP payment (check if user has sufficient balance)
    pub async fn validate_icp_payment(user_principal: &Principal, required_amount_e8s: u64) -> Result<bool, String> {
        let account = Self::principal_to_account_id(user_principal);
        let balance = Self::get_account_balance(&account).await?;
        
        let has_sufficient_balance = balance >= required_amount_e8s;
        
        ic_cdk::println!("💳 ICP Payment validation: User has {} e8s, needs {} e8s, sufficient: {}", 
            balance, required_amount_e8s, has_sufficient_balance);
        
        Ok(has_sufficient_balance)
    }

    /// Convert principal to account ID (ICP ledger format)
    pub fn principal_to_account_id(principal: &Principal) -> String {
        // This is a simplified version - in production, you'd use proper account derivation
        format!("{}", principal)
    }

    /// Process automatic ICP payment
    pub async fn process_automatic_icp_payment(
        user_principal: &Principal,
        amount_e8s: u64,
        memo: u64,
    ) -> Result<u64, String> {
        // 1. Validate user has sufficient balance
        let has_balance = Self::validate_icp_payment(user_principal, amount_e8s).await?;
        if !has_balance {
            return Err("Insufficient ICP balance".to_string());
        }

        // 2. Transfer ICP to bridge account
        let bridge_account = Self::principal_to_account_id(&ic_cdk::id());
        let block_index = Self::transfer_icp(&bridge_account, amount_e8s, memo, None).await?;

        ic_cdk::println!("✅ Automatic ICP payment processed: {} e8s, block: {}", amount_e8s, block_index);

        Ok(block_index)
    }
}

// Price feed cache for performance
thread_local! {
    static PRICE_CACHE: std::cell::RefCell<HashMap<String, (f64, u64)>> = 
        std::cell::RefCell::new(HashMap::new());
}

impl IcpLedgerService {
    /// Get cached price with TTL
    pub async fn get_cached_price(asset: &str, ttl_seconds: u64) -> Result<f64, String> {
        let now = ic_cdk::api::time() / 1_000_000_000; // Convert to seconds
        
        PRICE_CACHE.with(|cache| {
            let mut cache_ref = cache.borrow_mut();
            
            if let Some((price, timestamp)) = cache_ref.get(asset) {
                if now - timestamp < ttl_seconds {
                    return Ok(*price);
                }
            }
            
            // Cache miss or expired - would fetch from external API in production
            let price = match asset {
                "ICP" => 12.50,
                "ETH" => 3500.0,
                _ => return Err(format!("Unknown asset: {}", asset)),
            };
            
            cache_ref.insert(asset.to_string(), (price, now));
            Ok(price)
        })
    }
}
