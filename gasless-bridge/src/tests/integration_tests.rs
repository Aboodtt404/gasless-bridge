// Integration Tests for End-to-End Workflows
// Phase 5.1: Testing Component Interactions

use super::{TestResult, TestCategory, TestSuite, TestDataGenerator};
use crate::services::gas_estimator::estimate_gas_advanced;
use crate::services::threshold_ecdsa::get_canister_ethereum_address;

/// Run all integration tests
pub async fn run_integration_tests() -> TestSuite {
    let mut suite = TestSuite::new();
    
    ic_cdk::println!("🔗 Running Integration Tests...");
    
    // Test complete quote-to-settlement flow
    suite.add_result(test_quote_settlement_integration().await);
    
    // Test RPC and gas estimation integration
    suite.add_result(test_rpc_gas_integration().await);
    
    // Test ECDSA integration
    suite.add_result(test_ecdsa_integration().await);
    
    // Test reserve and settlement integration
    suite.add_result(test_reserve_settlement_integration().await);
    
    // Test multi-component error handling
    suite.add_result(test_error_propagation().await);
    
    // Test state consistency
    suite.add_result(test_state_consistency().await);
    
    ic_cdk::println!("✅ Integration Tests Complete: {}/{} passed", suite.passed_tests, suite.total_tests);
    suite
}

async fn test_quote_settlement_integration() -> TestResult {
    ic_cdk::println!("Testing Quote-Settlement Integration...");
    
    // This tests the full flow from quote creation to settlement
    let start_time = ic_cdk::api::time();
    
    // 1. Create a quote
    let quote = TestDataGenerator::generate_test_quote(1_000_000_000_000_000_000);
    let quote_valid = quote.is_valid() && !quote.is_expired();
    
    // 2. Create settlement from quote
    let settlement = TestDataGenerator::generate_test_settlement(&quote.id);
    let settlement_valid = settlement.quote_id == quote.id && 
                          settlement.amount == quote.amount_out;
    
    // 3. Test gasless economics
    let bridge_cost = quote.get_total_bridge_cost();
    let user_pays = quote.amount_in;
    let user_gets = quote.amount_out;
    let gasless_works = user_pays == user_gets && bridge_cost > user_gets;
    
    let duration = (ic_cdk::api::time() - start_time) / 1_000_000;
    
    TestResult {
        test_name: "Quote-Settlement Integration".to_string(),
        passed: quote_valid && settlement_valid && gasless_works,
        message: if quote_valid && settlement_valid && gasless_works {
            "Complete quote-settlement flow working correctly".to_string()
        } else {
            format!("Integration failed: quote_valid={}, settlement_valid={}, gasless_works={}", 
                   quote_valid, settlement_valid, gasless_works)
        },
        duration_ms: duration,
        category: TestCategory::Integration,
    }
}

async fn test_rpc_gas_integration() -> TestResult {
    ic_cdk::println!("Testing RPC-Gas Integration...");
    
    let start_time = ic_cdk::api::time();
    
    // Test that gas estimation works with our RPC system
    let gas_result = estimate_gas_advanced().await;
    
    let duration = (ic_cdk::api::time() - start_time) / 1_000_000;
    
    match gas_result {
        Ok(estimate) => {
            let reasonable_gas = estimate.gas_limit >= 21_000 && estimate.gas_limit <= 100_000;
            let reasonable_price = estimate.max_fee_per_gas > 0 && estimate.max_fee_per_gas < 1_000_000_000_000; // < 1000 Gwei
            
            TestResult {
                test_name: "RPC-Gas Integration".to_string(),
                passed: reasonable_gas && reasonable_price,
                message: format!(
                    "Gas estimation successful: {} gas at {:.2} Gwei", 
                    estimate.gas_limit, 
                    estimate.max_fee_per_gas as f64 / 1e9
                ),
                duration_ms: duration,
                category: TestCategory::Integration,
            }
        }
        Err(e) => TestResult {
            test_name: "RPC-Gas Integration".to_string(),
            passed: false,
            message: format!("Gas estimation failed: {}", e),
            duration_ms: duration,
            category: TestCategory::Integration,
        }
    }
}

async fn test_ecdsa_integration() -> TestResult {
    ic_cdk::println!("Testing ECDSA Integration...");
    
    let start_time = ic_cdk::api::time();
    
    // Test ECDSA address generation
    let address_result = get_canister_ethereum_address().await;
    
    let duration = (ic_cdk::api::time() - start_time) / 1_000_000;
    
    match address_result {
        Ok(address) => {
            let valid_address = format!("{}", address).starts_with("0x") && 
                               format!("{}", address).len() == 42;
            
            TestResult {
                test_name: "ECDSA Integration".to_string(),
                passed: valid_address,
                message: format!("ECDSA address generated: {}", address),
                duration_ms: duration,
                category: TestCategory::Integration,
            }
        }
        Err(e) => TestResult {
            test_name: "ECDSA Integration".to_string(),
            passed: false,
            message: format!("ECDSA address generation failed: {}", e),
            duration_ms: duration,
            category: TestCategory::Integration,
        }
    }
}

async fn test_reserve_settlement_integration() -> TestResult {
    ic_cdk::println!("Testing Reserve-Settlement Integration...");
    
    let start_time = ic_cdk::api::time();
    
    // Test that reserve can handle settlement requirements
    let mut reserve = TestDataGenerator::generate_test_reserve_state();
    let quote = TestDataGenerator::generate_test_quote(1_000_000_000_000_000_000);
    
    let initial_available = reserve.available_balance;
    
    // Test gasless fund locking for settlement
    let delivery_amount = quote.amount_out;
    let gas_subsidy = quote.get_bridge_subsidy();
    
    let lock_result = reserve.lock_gasless_funds(delivery_amount, gas_subsidy);
    let funds_locked = lock_result.is_ok();
    let balance_decreased = reserve.available_balance < initial_available;
    
    // Test capacity check
    let can_handle_more = reserve.can_subsidize_gasless(delivery_amount, gas_subsidy);
    
    let duration = (ic_cdk::api::time() - start_time) / 1_000_000;
    
    TestResult {
        test_name: "Reserve-Settlement Integration".to_string(),
        passed: funds_locked && balance_decreased,
        message: format!(
            "Reserve integration: locked={}, balance_ok={}, can_handle_more={}", 
            funds_locked, balance_decreased, can_handle_more
        ),
        duration_ms: duration,
        category: TestCategory::Integration,
    }
}

async fn test_error_propagation() -> TestResult {
    ic_cdk::println!("Testing Error Propagation...");
    
    let start_time = ic_cdk::api::time();
    
    // Test that errors propagate correctly through the system
    let mut reserve = TestDataGenerator::generate_test_reserve_state();
    
    // Force an error by trying to lock more than available
    reserve.available_balance = 100_000_000_000_000_000; // 0.1 ETH
    
    let large_delivery = 1_000_000_000_000_000_000; // 1 ETH
    let gas_subsidy = 50_000_000_000_000_000;       // 0.05 ETH
    
    let error_result = reserve.lock_gasless_funds(large_delivery, gas_subsidy);
    let error_caught = error_result.is_err();
    
    // Test error message quality
    let error_message = error_result.unwrap_err();
    let error_informative = error_message.contains("Insufficient") && 
                           error_message.contains("ETH");
    
    let duration = (ic_cdk::api::time() - start_time) / 1_000_000;
    
    TestResult {
        test_name: "Error Propagation".to_string(),
        passed: error_caught && error_informative,
        message: format!("Error handling: caught={}, informative={}", error_caught, error_informative),
        duration_ms: duration,
        category: TestCategory::Integration,
    }
}

async fn test_state_consistency() -> TestResult {
    ic_cdk::println!("Testing State Consistency...");
    
    let start_time = ic_cdk::api::time();
    
    // Test that state remains consistent across operations
    let mut reserve = TestDataGenerator::generate_test_reserve_state();
    
    let initial_total = reserve.total_balance;
    let initial_locked = reserve.locked_balance;
    let initial_available = reserve.available_balance;
    
    // Verify initial consistency
    let initial_consistent = initial_total == initial_locked + initial_available;
    
    // Perform operations
    let lock_amount = 500_000_000_000_000_000; // 0.5 ETH
    let gas_subsidy = 25_000_000_000_000_000;  // 0.025 ETH
    
    let _lock_result = reserve.lock_gasless_funds(lock_amount, gas_subsidy);
    
    // Check consistency after operation
    let final_consistent = reserve.total_balance == reserve.locked_balance + reserve.available_balance;
    let balance_math = reserve.total_balance == initial_total; // Total should remain same
    
    let duration = (ic_cdk::api::time() - start_time) / 1_000_000;
    
    TestResult {
        test_name: "State Consistency".to_string(),
        passed: initial_consistent && final_consistent && balance_math,
        message: format!(
            "State consistency: initial={}, final={}, balance_preserved={}", 
            initial_consistent, final_consistent, balance_math
        ),
        duration_ms: duration,
        category: TestCategory::Integration,
    }
}
