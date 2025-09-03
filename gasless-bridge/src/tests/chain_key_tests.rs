use crate::services::chain_key_tokens::{
    ChainKeyTokenService, ChainKeyTokenType, MintOperationStatus, BurnOperationStatus
};
use candid::Principal;

/// Comprehensive testing suite for chain-key token operations
pub struct ChainKeyTokenTestSuite;

impl ChainKeyTokenTestSuite {
    /// Run all chain-key token tests
    pub async fn run_all_tests() -> String {
        let mut results = Vec::new();
        
        // Test token configuration
        results.push(Self::test_token_configuration());
        results.push(Self::test_token_validation());
        results.push(Self::test_mint_operations());
        results.push(Self::test_burn_operations().await);
        results.push(Self::test_balance_management());
        results.push(Self::test_error_handling());
        results.push(Self::test_integration_flow().await);
        
        // Format results
        let mut output = String::new();
        output.push_str("🪙 Chain-Key Token Test Suite Results:\n");
        output.push_str("=====================================\n\n");
        
        for (i, result) in results.iter().enumerate() {
            output.push_str(&format!("Test {}: {}\n", i + 1, result));
        }
        
        // Summary
        let passed = results.iter().filter(|r| r.contains("✅")).count();
        let total = results.len();
        
        output.push_str(&format!("\n📊 Summary: {}/{} tests passed\n", passed, total));
        
        if passed == total {
            output.push_str("🎉 All chain-key token tests passed!\n");
        } else {
            output.push_str("❌ Some tests failed. Check logs for details.\n");
        }
        
        output
    }
    
    /// Test token configuration initialization
    fn test_token_configuration() -> String {
        let service = ChainKeyTokenService::new();
        
        // Test ckETH configuration
        let cketh_config = service.get_token_config(&ChainKeyTokenType::CkEth);
        if cketh_config.is_none() {
            return "❌ ckETH configuration not found".to_string();
        }
        
        let cketh_config = cketh_config.unwrap();
        if cketh_config.decimals != 18 {
            return format!("❌ ckETH decimals incorrect: expected 18, got {}", cketh_config.decimals);
        }
        
        // Test ckUSDC configuration
        let ckusdc_config = service.get_token_config(&ChainKeyTokenType::CkUsdc);
        if ckusdc_config.is_none() {
            return "❌ ckUSDC configuration not found".to_string();
        }
        
        let ckusdc_config = ckusdc_config.unwrap();
        if ckusdc_config.decimals != 6 {
            return format!("❌ ckUSDC decimals incorrect: expected 6, got {}", ckusdc_config.decimals);
        }
        
        // Test supported tokens
        let supported = service.get_supported_chain_key_tokens();
        if supported.len() < 3 {
            return format!("❌ Expected at least 3 supported tokens, got {}", supported.len());
        }
        
        "✅ Token configuration test passed".to_string()
    }
    
    /// Test token validation logic
    fn test_token_validation() -> String {
        let service = ChainKeyTokenService::new();
        
        // Test valid amounts
        let valid_amount = 1_000_000_000_000_000_000; // 1 ETH
        let result = service.validate_amount(&ChainKeyTokenType::CkEth, valid_amount);
        if result.is_err() {
            return format!("❌ Valid amount validation failed: {}", result.unwrap_err());
        }
        
        // Test amount too small
        let small_amount = 100_000_000_000_000; // 0.0001 ETH (below 0.001 minimum)
        let result = service.validate_amount(&ChainKeyTokenType::CkEth, small_amount);
        if result.is_ok() {
            return "❌ Small amount should have failed validation".to_string();
        }
        
        // Test amount too large
        let large_amount = 15_000_000_000_000_000_000; // 15 ETH (above 10 maximum, fits in u64)
        let result = service.validate_amount(&ChainKeyTokenType::CkEth, large_amount);
        if result.is_ok() {
            return "❌ Large amount should have failed validation".to_string();
        }
        
        // Test unsupported token
        let custom_token = ChainKeyTokenType::Custom("UNSUPPORTED".to_string());
        let result = service.validate_amount(&custom_token, valid_amount);
        if result.is_ok() {
            return "❌ Unsupported token should have failed validation".to_string();
        }
        
        "✅ Token validation test passed".to_string()
    }
    
    /// Test mint operations
    fn test_mint_operations() -> String {
        let mut service = ChainKeyTokenService::new();
        
        // Add funds to ckETH reserve
        let add_result = service.add_reserve_funds(&ChainKeyTokenType::CkEth, 10_000_000_000_000_000_000); // 10 ETH
        if add_result.is_err() {
            return format!("❌ Failed to add ckETH reserve funds: {}", add_result.unwrap_err());
        }
        
        // Test mint operation creation
        let mint_amount = 1_000_000_000_000_000_000; // 1 ETH
        let ethereum_tx = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        
        let mint_op = service.create_mint_operation(
            ChainKeyTokenType::CkEth,
            mint_amount,
            ethereum_tx.to_string(),
        );
        
        if mint_op.is_err() {
            return format!("❌ Failed to create mint operation: {}", mint_op.unwrap_err());
        }
        
        let mint_op = mint_op.unwrap();
        
        // Verify operation details
        if mint_op.amount != mint_amount {
            return format!("❌ Mint operation amount mismatch: expected {}, got {}", mint_amount, mint_op.amount);
        }
        
        if mint_op.status != MintOperationStatus::Pending {
            return format!("❌ Mint operation status should be Pending, got {:?}", mint_op.status);
        }
        
        // Test mint operation completion
        let complete_result = service.complete_mint_operation(&mint_op.id);
        if complete_result.is_err() {
            return format!("❌ Failed to complete mint operation: {}", complete_result.unwrap_err());
        }
        
        // Verify completion
        let completed_op = service.get_mint_operation(&mint_op.id);
        if completed_op.is_none() {
            return "❌ Completed mint operation not found".to_string();
        }
        
        let completed_op = completed_op.unwrap();
        if completed_op.status != MintOperationStatus::Completed {
            return format!("❌ Mint operation should be Completed, got {:?}", completed_op.status);
        }
        
        if completed_op.completed_at.is_none() {
            return "❌ Mint operation completion timestamp missing".to_string();
        }
        
        "✅ Mint operations test passed".to_string()
    }
    
    /// Test burn operations
    async fn test_burn_operations() -> String {
        let mut service = ChainKeyTokenService::new();
        
        // Test burn operation creation
        let burn_amount = 500_000_000_000_000_000; // 0.5 ETH
        let destination = "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6";
        
        let burn_op = service.create_burn_operation(
            ChainKeyTokenType::CkEth,
            burn_amount,
            destination.to_string(),
        );
        
        if burn_op.is_err() {
            return format!("❌ Failed to create burn operation: {}", burn_op.unwrap_err());
        }
        
        let burn_op = burn_op.unwrap();
        
        // Verify operation details
        if burn_op.amount != burn_amount {
            return format!("❌ Burn operation amount mismatch: expected {}, got {}", burn_amount, burn_op.amount);
        }
        
        if burn_op.destination_address != destination {
            return format!("❌ Burn operation destination mismatch: expected {}, got {}", destination, burn_op.destination_address);
        }
        
        if burn_op.status != BurnOperationStatus::Pending {
            return format!("❌ Burn operation status should be Pending, got {:?}", burn_op.status);
        }
        
        // Test burn operation completion
        let ethereum_tx = "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321";
        let complete_result = service.complete_burn_operation(&burn_op.id).await;
        if complete_result.is_err() {
            return format!("❌ Failed to complete burn operation: {}", complete_result.unwrap_err());
        }
        
        // Verify completion
        let completed_op = service.get_burn_operation(&burn_op.id);
        if completed_op.is_none() {
            return "❌ Completed burn operation not found".to_string();
        }
        
        let completed_op = completed_op.unwrap();
        if completed_op.status != BurnOperationStatus::Completed {
            return format!("❌ Burn operation should be Completed, got {:?}", completed_op.status);
        }
        
        if completed_op.ethereum_tx_hash.is_none() {
            return "❌ Burn operation transaction hash missing".to_string();
        }
        
        if completed_op.ethereum_tx_hash.as_ref().unwrap() != ethereum_tx {
            return format!("❌ Burn operation transaction hash mismatch: expected {}, got {}", 
                ethereum_tx, completed_op.ethereum_tx_hash.as_ref().unwrap());
        }
        
        "✅ Burn operations test passed".to_string()
    }
    
    /// Test balance management
    fn test_balance_management() -> String {
        let mut service = ChainKeyTokenService::new();
        
        // Test initial balances
        let initial_balance = service.get_token_balance(&ChainKeyTokenType::CkEth);
        if initial_balance.is_none() {
            return "❌ Initial ckETH balance not found".to_string();
        }
        
        let initial_balance = initial_balance.unwrap();
        if initial_balance.available_balance != 0 {
            return format!("❌ Initial available balance should be 0, got {}", initial_balance.available_balance);
        }
        
        // Add funds
        let add_amount = 5_000_000_000_000_000_000; // 5 ETH
        let add_result = service.add_reserve_funds(&ChainKeyTokenType::CkEth, add_amount);
        if add_result.is_err() {
            return format!("❌ Failed to add funds: {}", add_result.unwrap_err());
        }
        
        // Verify balance update
        let updated_balance = service.get_token_balance(&ChainKeyTokenType::CkEth);
        if updated_balance.is_none() {
            return "❌ Updated ckETH balance not found".to_string();
        }
        
        let updated_balance = updated_balance.unwrap();
        if updated_balance.available_balance != add_amount {
            return format!("❌ Available balance should be {}, got {}", add_amount, updated_balance.available_balance);
        }
        
        // Test minting affects balance
        let mint_amount = 1_000_000_000_000_000_000; // 1 ETH
        let mint_op = service.create_mint_operation(
            ChainKeyTokenType::CkEth,
            mint_amount,
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        );
        
        if mint_op.is_err() {
            return format!("❌ Failed to create mint operation: {}", mint_op.unwrap_err());
        }
        
        // Check that amount is locked
        let locked_balance = service.get_token_balance(&ChainKeyTokenType::CkEth);
        if locked_balance.is_none() {
            return "❌ Locked balance not found".to_string();
        }
        
        let locked_balance = locked_balance.unwrap();
        if locked_balance.locked_balance != mint_amount {
            return format!("❌ Locked balance should be {}, got {}", mint_amount, locked_balance.locked_balance);
        }
        
        if locked_balance.available_balance != add_amount - mint_amount {
            return format!("❌ Available balance should be {}, got {}", 
                add_amount - mint_amount, locked_balance.available_balance);
        }
        
        "✅ Balance management test passed".to_string()
    }
    
    /// Test error handling
    fn test_error_handling() -> String {
        let mut service = ChainKeyTokenService::new();
        
        // Test minting without funds
        let mint_result = service.create_mint_operation(
            ChainKeyTokenType::CkEth,
            1_000_000_000_000_000_000, // 1 ETH
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        );
        
        if mint_result.is_ok() {
            return "❌ Mint operation should fail without funds".to_string();
        }
        
        // Test invalid amount
        let small_amount = 100_000_000_000_000; // 0.0001 ETH (below minimum)
        let mint_result = service.create_mint_operation(
            ChainKeyTokenType::CkEth,
            small_amount,
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        );
        
        if mint_result.is_ok() {
            return "❌ Mint operation should fail with invalid amount".to_string();
        }
        
        // Test invalid destination address
        let burn_result = service.create_burn_operation(
            ChainKeyTokenType::CkEth,
            1_000_000_000_000_000_000, // 1 ETH
            "invalid_address".to_string(),
        );
        
        if burn_result.is_ok() {
            return "❌ Burn operation should fail with invalid address".to_string();
        }
        
        // Test completing non-existent operation
        let complete_result = service.complete_mint_operation("non_existent_id");
        if complete_result.is_ok() {
            return "❌ Completing non-existent operation should fail".to_string();
        }
        
        "✅ Error handling test passed".to_string()
    }
    
    /// Test complete integration flow
    async fn test_integration_flow() -> String {
        let mut service = ChainKeyTokenService::new();
        
        // 1. Add funds to reserve
        let add_result = service.add_reserve_funds(&ChainKeyTokenType::CkEth, 10_000_000_000_000_000_000); // 10 ETH
        if add_result.is_err() {
            return format!("❌ Failed to add funds: {}", add_result.unwrap_err());
        }
        
        // 2. Create mint operation
        let mint_amount = 2_000_000_000_000_000_000; // 2 ETH
        let mint_op = service.create_mint_operation(
            ChainKeyTokenType::CkEth,
            mint_amount,
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        );
        
        if mint_op.is_err() {
            return format!("❌ Failed to create mint operation: {}", mint_op.unwrap_err());
        }
        
        let mint_op = mint_op.unwrap();
        
        // 3. Complete mint operation
        let complete_mint = service.complete_mint_operation(&mint_op.id);
        if complete_mint.is_err() {
            return format!("❌ Failed to complete mint: {}", complete_mint.unwrap_err());
        }
        
        // 4. Create burn operation
        let burn_amount = 1_000_000_000_000_000_000; // 1 ETH
        let burn_op = service.create_burn_operation(
            ChainKeyTokenType::CkEth,
            burn_amount,
            "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6".to_string(),
        );
        
        if burn_op.is_err() {
            return format!("❌ Failed to create burn operation: {}", burn_op.unwrap_err());
        }
        
        let burn_op = burn_op.unwrap();
        
        // 5. Complete burn operation
        let complete_burn = service.complete_burn_operation(&burn_op.id).await;
        
        if complete_burn.is_err() {
            return format!("❌ Failed to complete burn: {}", complete_burn.unwrap_err());
        }
        
        // 6. Verify final state
        let final_balance = service.get_token_balance(&ChainKeyTokenType::CkEth);
        if final_balance.is_none() {
            return "❌ Final balance not found".to_string();
        }
        
        let final_balance = final_balance.unwrap();
        let expected_available = 10_000_000_000_000_000_000 - 2_000_000_000_000_000_000 + 1_000_000_000_000_000_000; // 10 - 2 + 1 = 9 ETH
        
        if final_balance.available_balance != expected_available {
            return format!("❌ Final available balance incorrect: expected {}, got {}", 
                expected_available, final_balance.available_balance);
        }
        
        // 7. Check operation counts
        let mint_ops = service.get_user_mint_operations(&Principal::anonymous());
        let burn_ops = service.get_user_burn_operations(&Principal::anonymous());
        
        if mint_ops.len() != 1 {
            return format!("❌ Expected 1 mint operation, got {}", mint_ops.len());
        }
        
        if burn_ops.len() != 1 {
            return format!("❌ Expected 1 burn operation, got {}", burn_ops.len());
        }
        
        "✅ Integration flow test passed".to_string()
    }
    
    /// Get supported tokens list
    pub fn get_supported_tokens() -> Vec<String> {
        let service = ChainKeyTokenService::new();
        service.get_supported_chain_key_tokens()
    }
    
    /// Get service status
    pub fn get_service_status() -> String {
        let service = ChainKeyTokenService::new();
        service.get_service_status()
    }
}
