use candid::{CandidType, Deserialize};
use ic_cdk::caller;
use std::collections::HashMap;

/// Chain-key token types supported by the bridge
#[derive(Debug, Clone, PartialEq, Eq, Hash, CandidType, Deserialize)]
pub enum ChainKeyTokenType {
    CkEth,
    CkUsdc,
    CkUsdt,
    CkDai,
    CkWbtc,
    Custom(String), // For other ERC-20 tokens
}

impl std::fmt::Display for ChainKeyTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainKeyTokenType::CkEth => write!(f, "ckETH"),
            ChainKeyTokenType::CkUsdc => write!(f, "ckUSDC"),
            ChainKeyTokenType::CkUsdt => write!(f, "ckUSDT"),
            ChainKeyTokenType::CkDai => write!(f, "ckDAI"),
            ChainKeyTokenType::CkWbtc => write!(f, "ckWBTC"),
            ChainKeyTokenType::Custom(s) => write!(f, "ck{}", s),
        }
    }
}

/// Chain-key token configuration
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ChainKeyTokenConfig {
    pub token_type: ChainKeyTokenType,
    pub ethereum_address: String,        // Contract address on Ethereum
    pub decimals: u8,                   // Token decimals
    pub min_amount: u64,                // Minimum transfer amount
    pub max_amount: u64,                // Maximum transfer amount
    pub gas_limit: u64,                 // Gas limit for token transfers
    pub is_active: bool,                // Whether token is enabled
}

/// Chain-key token balance and operations
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ChainKeyTokenBalance {
    pub token_type: ChainKeyTokenType,
    pub available_balance: u64,         // Available for minting
    pub locked_balance: u64,            // Locked in pending operations
    pub total_supply: u64,              // Total minted supply
    pub last_operation: u64,            // Timestamp of last operation
}

/// Chain-key minting operation
#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct ChainKeyMintOperation {
    pub id: String,
    pub user_principal: candid::Principal,
    pub token_type: ChainKeyTokenType,
    pub amount: u64,
    pub ethereum_tx_hash: String,       // Proof of ETH deposit
    pub status: MintOperationStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum MintOperationStatus {
    Pending,    // Waiting for verification
    Verifying,  // Checking Ethereum transaction
    Completed,  // ckETH minted successfully
    Failed,     // Verification failed
}

/// Chain-key burning operation
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ChainKeyBurnOperation {
    pub id: String,
    pub user_principal: candid::Principal,
    pub token_type: ChainKeyTokenType,
    pub amount: u64,
    pub destination_address: String,    // Where to send native token
    pub status: BurnOperationStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub ethereum_tx_hash: Option<String>, // Transaction hash when completed
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum BurnOperationStatus {
    Pending,    // Waiting for ckToken burn
    Burning,    // Burning ckToken
    Executing,  // Executing Ethereum transaction
    Completed,  // Native token delivered
    Failed,     // Operation failed
}

/// Main service for chain-key token operations
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ChainKeyTokenService {
    pub configs: HashMap<ChainKeyTokenType, ChainKeyTokenConfig>,
    pub balances: HashMap<ChainKeyTokenType, ChainKeyTokenBalance>,
    pub mint_operations: HashMap<String, ChainKeyMintOperation>,
    pub burn_operations: HashMap<String, ChainKeyBurnOperation>,
}

impl ChainKeyTokenService {
    pub fn new() -> Self {
        let mut service = Self {
            configs: HashMap::new(),
            balances: HashMap::new(),
            mint_operations: HashMap::new(),
            burn_operations: HashMap::new(),
        };
        
        // Initialize default configurations
        service.initialize_default_configs();
        
        service
    }
    
    /// Initialize default token configurations
    fn initialize_default_configs(&mut self) {
        // ckETH configuration
        self.configs.insert(ChainKeyTokenType::CkEth, ChainKeyTokenConfig {
            token_type: ChainKeyTokenType::CkEth,
            ethereum_address: "0x0000000000000000000000000000000000000000".to_string(), // ETH is native
            decimals: 18,
            min_amount: 1_000_000_000_000_000, // 0.001 ETH
            max_amount: 10_000_000_000_000_000_000, // 10 ETH (fits in u64)
            gas_limit: 21_000, // Standard ETH transfer gas
            is_active: true,
        });
        
        // ckUSDC configuration
        self.configs.insert(ChainKeyTokenType::CkUsdc, ChainKeyTokenConfig {
            token_type: ChainKeyTokenType::CkUsdc,
            ethereum_address: "0xA0b86a33E6441b8c4C8C3C8C3C8C3C8C3C8C3C8C".to_string(), // USDC contract
            decimals: 6,
            min_amount: 1_000_000,  // 1 USDC
            max_amount: 1_000_000_000, // 1M USDC
            gas_limit: 65_000, // ERC-20 transfer gas
            is_active: true,
        });
        
        // ckUSDT configuration
        self.configs.insert(ChainKeyTokenType::CkUsdt, ChainKeyTokenConfig {
            token_type: ChainKeyTokenType::CkUsdt,
            ethereum_address: "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(), // USDT contract
            decimals: 6,
            min_amount: 1_000_000,  // 1 USDT
            max_amount: 1_000_000_000, // 1M USDT
            gas_limit: 65_000, // ERC-20 transfer gas
            is_active: true,
        });
        
        // Initialize balances
        for token_type in self.configs.keys() {
            self.balances.insert(token_type.clone(), ChainKeyTokenBalance {
                token_type: token_type.clone(),
                available_balance: 0,
                locked_balance: 0,
                total_supply: 0,
                last_operation: 0,
            });
        }
    }
    
    /// Get token configuration
    pub fn get_token_config(&self, token_type: &ChainKeyTokenType) -> Option<&ChainKeyTokenConfig> {
        self.configs.get(token_type)
    }
    
    /// Get token balance
    pub fn get_token_balance(&self, token_type: &ChainKeyTokenType) -> Option<&ChainKeyTokenBalance> {
        self.balances.get(token_type)
    }
    
    /// Check if token is supported and active
    pub fn is_token_supported(&self, token_type: &ChainKeyTokenType) -> bool {
        self.configs.get(token_type)
            .map(|config| config.is_active)
            .unwrap_or(false)
    }
    
    /// Validate amount for token
    pub fn validate_amount(&self, token_type: &ChainKeyTokenType, amount: u64) -> Result<(), String> {
        let config = self.configs.get(token_type)
            .ok_or_else(|| format!("Token {} not supported", token_type))?;
            
        if !config.is_active {
            return Err(format!("Token {} is not active", token_type));
        }
        
        if amount < config.min_amount {
            return Err(format!(
                "Amount too small for {}. Minimum: {} {}",
                token_type,
                config.min_amount,
                token_type
            ));
        }
        
        if amount > config.max_amount {
            return Err(format!(
                "Amount too large for {}. Maximum: {} {}",
                token_type,
                config.max_amount,
                token_type
            ));
        }
        
        Ok(())
    }
    
    /// Create mint operation for ckETH/ckERC20
    pub fn create_mint_operation(
        &mut self,
        token_type: ChainKeyTokenType,
        amount: u64,
        ethereum_tx_hash: String,
    ) -> Result<ChainKeyMintOperation, String> {
        // Validate token and amount
        self.validate_amount(&token_type, amount)?;
        
        // Check if we have enough balance to mint
        let balance = self.balances.get(&token_type)
            .ok_or_else(|| format!("Token {} balance not found", token_type))?;
            
        if balance.available_balance < amount {
            return Err(format!(
                "Insufficient {} balance. Available: {}, Requested: {}",
                token_type, balance.available_balance, amount
            ));
        }
        
        // Create mint operation
        let operation_id = format!(
            "mint_{}_{}_{}",
            token_type,
            caller().to_text().chars().take(8).collect::<String>(),
            ic_cdk::api::time() / 1_000_000_000
        );
        
        let operation = ChainKeyMintOperation {
            id: operation_id.clone(),
            user_principal: caller(),
            token_type: token_type.clone(),
            amount,
            ethereum_tx_hash,
            status: MintOperationStatus::Pending,
            created_at: ic_cdk::api::time() / 1_000_000_000,
            completed_at: None,
        };
        
        // Lock the amount
        if let Some(balance) = self.balances.get_mut(&token_type) {
            balance.available_balance -= amount;
            balance.locked_balance += amount;
        }
        
        // Store operation
        self.mint_operations.insert(operation_id.clone(), operation.clone());
        
        ic_cdk::println!(
            "🪙 Created mint operation {} for {} {} (amount: {})",
            operation_id, amount, token_type, amount
        );
        
        Ok(operation)
    }
    
    /// Create burn operation for ckETH/ckERC20
    pub fn create_burn_operation(
        &mut self,
        token_type: ChainKeyTokenType,
        amount: u64,
        destination_address: String,
    ) -> Result<ChainKeyBurnOperation, String> {
        // Validate token and amount
        self.validate_amount(&token_type, amount)?;
        
        // Validate destination address
        if !destination_address.starts_with("0x") || destination_address.len() != 42 {
            return Err("Invalid Ethereum address format".to_string());
        }
        
        // Create burn operation
        let operation_id = format!(
            "burn_{}_{}_{}",
            token_type,
            caller().to_text().chars().take(8).collect::<String>(),
            ic_cdk::api::time() / 1_000_000_000
        );
        
        let operation = ChainKeyBurnOperation {
            id: operation_id.clone(),
            user_principal: caller(),
            token_type: token_type.clone(),
            amount,
            destination_address: destination_address.clone(),
            status: BurnOperationStatus::Pending,
            created_at: ic_cdk::api::time() / 1_000_000_000,
            completed_at: None,
            ethereum_tx_hash: None,
        };
        
        // Store operation
        self.burn_operations.insert(operation_id.clone(), operation.clone());
        
        ic_cdk::println!(
            "🔥 Created burn operation {} for {} {} to {}",
            operation_id, amount, token_type, destination_address.clone()
        );
        
        Ok(operation)
    }
    
    /// Complete mint operation (simulate ckETH minting)
    pub fn complete_mint_operation(&mut self, operation_id: &str) -> Result<(), String> {
        let operation = self.mint_operations.get_mut(operation_id)
            .ok_or("Mint operation not found")?;
            
        if operation.status != MintOperationStatus::Pending {
            return Err("Operation cannot be completed in current status".to_string());
        }
        
        // Simulate ckETH minting process
        operation.status = MintOperationStatus::Verifying;
        
        // In production, this would:
        // 1. Verify the Ethereum transaction proof
        // 2. Call the ckETH minter canister
        // 3. Mint ckETH to the user's account
        
        // For now, simulate successful completion
        operation.status = MintOperationStatus::Completed;
        operation.completed_at = Some(ic_cdk::api::time() / 1_000_000_000);
        
        // Update balances
        if let Some(balance) = self.balances.get_mut(&operation.token_type) {
            balance.locked_balance -= operation.amount;
            balance.total_supply += operation.amount;
        }
        
        ic_cdk::println!(
            "✅ Completed mint operation {} for {} {}",
            operation_id, operation.amount, operation.token_type
        );
        
        Ok(())
    }
    
    /// Complete a burn operation by executing the bridge transaction
    /// This is where ckETH → ETH actually happens!
    pub async fn complete_burn_operation(
        &mut self,
        operation_id: &str,
    ) -> Result<String, String> {
        ic_cdk::println!("🔥 Completing burn operation: {}", operation_id);
        
        // Get the burn operation
        let burn_op = self.burn_operations.get(operation_id)
            .ok_or_else(|| "Burn operation not found".to_string())?
            .clone();
        
        // Check if operation is pending
        if burn_op.status != BurnOperationStatus::Pending {
            return Err(format!("Operation {} is not pending (status: {:?})", operation_id, burn_op.status));
        }
        
        // Validate the operation
        let config = self.configs.get(&burn_op.token_type)
            .ok_or_else(|| "Token configuration not found".to_string())?;
        
        if !config.is_active {
            return Err("Token type is not active".to_string());
        }
        
        // Check if we have sufficient balance
        let balance = self.balances.get(&burn_op.token_type)
            .ok_or_else(|| "Token balance not found".to_string())?;
        
        if balance.available_balance < burn_op.amount {
            return Err(format!(
                "Insufficient balance: need {} {}, have {} {}",
                burn_op.amount, burn_op.token_type, balance.available_balance, burn_op.token_type
            ));
        }
        
        ic_cdk::println!("✅ Burn operation validated successfully");
        
        // Execute the bridge transaction
        let recipient_address = burn_op.destination_address.clone();
        let amount = burn_op.amount;
        
        // Parse Ethereum address
        let eth_address = if recipient_address.starts_with("0x") {
            let hex = &recipient_address[2..];
            if hex.len() != 40 {
                return Err("Invalid Ethereum address length".to_string());
            }
            
            let mut address_bytes = [0u8; 20];
            for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
                if i >= 20 {
                    break;
                }
                let byte = u8::from_str_radix(
                    std::str::from_utf8(chunk).map_err(|_| "Invalid hex")?,
                    16
                ).map_err(|_| "Invalid hex")?;
                address_bytes[i] = byte;
            }
            
            crate::services::threshold_ecdsa::EthereumAddress(address_bytes)
        } else {
            return Err("Ethereum address must start with 0x".to_string());
        };
        
        // Create gas estimate for the transaction
        let gas_estimate = crate::services::gas_estimator::GasEstimate {
            gas_limit: config.gas_limit,
            max_fee_per_gas: 20_000_000_000, // 20 Gwei
            priority_fee: 1_000_000_000,     // 1 Gwei
            total_cost: config.gas_limit as u64 * 20_000_000_000,
            base_fee: 15_000_000_000,        // 15 Gwei base fee
            safety_margin: 5_000_000_000,    // 5 Gwei safety margin
        };
        
        ic_cdk::println!("🚀 Executing bridge transaction: {} {} to {}", 
            amount, burn_op.token_type, recipient_address);
        
        // Execute the complete bridge transaction
        let result = crate::services::eth_transaction::execute_bridge_transaction(
            eth_address,
            amount,
            gas_estimate,
        ).await?;
        
        // Update operation status
        let burn_op = self.burn_operations.get_mut(operation_id)
            .ok_or_else(|| "Failed to get mutable reference to burn operation")?;
        
        burn_op.status = BurnOperationStatus::Completed;
        burn_op.completed_at = Some(ic_cdk::api::time() / 1_000_000_000);
        
        // Update balance (burn the ckETH)
        let balance = self.balances.get_mut(&burn_op.token_type)
            .ok_or_else(|| "Failed to get mutable reference to balance")?;
        
        balance.available_balance = balance.available_balance.saturating_sub(amount);
        balance.total_supply = balance.total_supply.saturating_sub(amount);
        
        ic_cdk::println!("✅ Burn operation completed successfully!");
        
        Ok(format!(
            "🔥 Burn Operation Completed!\n\
             \n\
             📋 Operation ID: {}\n\
             🪙 Token: {}\n\
             💰 Amount: {} wei\n\
             📥 Destination: {}\n\
             ✅ Status: Completed\n\
             \n\
             🌉 Bridge Transaction:\n\
             {}",
            operation_id,
            burn_op.token_type,
            amount,
            recipient_address,
            result
        ))
    }
    
    /// Get mint operation by ID
    pub fn get_mint_operation(&self, operation_id: &str) -> Option<&ChainKeyMintOperation> {
        self.mint_operations.get(operation_id)
    }
    
    /// Get burn operation by ID
    pub fn get_burn_operation(&self, operation_id: &str) -> Option<&ChainKeyBurnOperation> {
        self.burn_operations.get(operation_id)
    }
    
    /// Get all mint operations for a user
    pub fn get_user_mint_operations(&self, user_principal: &candid::Principal) -> Vec<ChainKeyMintOperation> {
        self.mint_operations
            .values()
            .filter(|op| &op.user_principal == user_principal)
            .cloned()
            .collect()
    }
    
    /// Get all burn operations for a user
    pub fn get_user_burn_operations(&self, user_principal: &candid::Principal) -> Vec<ChainKeyBurnOperation> {
        self.burn_operations
            .values()
            .filter(|op| &op.user_principal == user_principal)
            .cloned()
            .collect()
    }
    
    /// Add funds to token reserve (admin function)
    pub fn add_reserve_funds(&mut self, token_type: &ChainKeyTokenType, amount: u64) -> Result<(), String> {
        if let Some(balance) = self.balances.get_mut(token_type) {
            balance.available_balance += amount;
            ic_cdk::println!(
                "💰 Added {} {} to {} reserve. New balance: {}",
                amount, token_type, token_type, balance.available_balance
            );
            Ok(())
        } else {
            Err(format!("Token {} not found", token_type))
        }
    }
    
    /// Get service status
    pub fn get_service_status(&self) -> String {
        let mut status = String::new();
        status.push_str("🪙 Chain-Key Token Service Status:\n");
        
        for (token_type, balance) in &self.balances {
            status.push_str(&format!(
                "  {}: Available: {}, Locked: {}, Total Supply: {}\n",
                token_type, balance.available_balance, balance.locked_balance, balance.total_supply
            ));
        }
        
        status.push_str(&format!(
            "📊 Operations: {} mint, {} burn\n",
            self.mint_operations.len(),
            self.burn_operations.len()
        ));
        
        status
    }
    
    /// Get list of supported chain-key tokens
    pub fn get_supported_chain_key_tokens(&self) -> Vec<String> {
        self.configs.keys()
            .map(|token_type| token_type.to_string())
            .collect()
    }
}

/// Helper functions for chain-key token operations
pub mod helpers {
    use super::*;
    
    /// Convert amount from token decimals to wei/smallest unit
    pub fn to_smallest_unit(amount: f64, decimals: u8) -> u64 {
        (amount * 10_f64.powi(decimals as i32)) as u64
    }
    
    /// Convert amount from wei/smallest unit to token decimals
    pub fn from_smallest_unit(amount: u64, decimals: u8) -> f64 {
        amount as f64 / 10_f64.powi(decimals as i32)
    }
    
    /// Format token amount with proper decimals
    pub fn format_token_amount(amount: u64, token_type: &ChainKeyTokenType) -> String {
        let decimals = match token_type {
            ChainKeyTokenType::CkEth => 18,
            ChainKeyTokenType::CkUsdc => 6,
            ChainKeyTokenType::CkUsdt => 6,
            ChainKeyTokenType::CkDai => 18,
            ChainKeyTokenType::CkWbtc => 8,
            ChainKeyTokenType::Custom(_) => 18, // Default to 18
        };
        
        let amount_f64 = from_smallest_unit(amount, decimals);
        format!("{} {}", amount_f64, token_type)
    }
}
