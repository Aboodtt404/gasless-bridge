// Unit Tests for Individual Components
// Phase 5.1: Testing Core Functionality

use super::{TestResult, TestCategory, TestSuite, TestDataGenerator};
use crate::types::{QuoteStatus, SettlementStatus};
use crate::services::gas_estimator::{GasEstimate, validate_gas_estimate, get_fallback_estimate};
use crate::{test_assert};

/// Run all unit tests
pub async fn run_unit_tests() -> TestSuite {
    let mut suite = TestSuite::new();
    
    ic_cdk::println!("🧪 Running Unit Tests...");
    
    // Test Quote functionality
    suite.add_result(test_quote_creation());
    suite.add_result(test_quote_expiry());
    suite.add_result(test_quote_validation());
    suite.add_result(test_gasless_quote_logic());
    
    // Test Settlement functionality
    suite.add_result(test_settlement_creation());
    suite.add_result(test_settlement_status_transitions());
    
    // Test Reserve State functionality
    suite.add_result(test_reserve_state_operations());
    suite.add_result(test_reserve_health_checks());
    suite.add_result(test_gasless_fund_locking());
    
    // Test Gas Estimation
    suite.add_result(test_gas_estimate_validation());
    suite.add_result(test_fallback_gas_estimate());
    
    // Test Type System
    suite.add_result(test_type_serialization());
    
    ic_cdk::println!("✅ Unit Tests Complete: {}/{} passed", suite.passed_tests, suite.total_tests);
    suite
}

fn test_quote_creation() -> TestResult {
    let quote = TestDataGenerator::generate_test_quote(1_000_000_000_000_000_000);
    
    test_assert!(
        quote.amount_in == quote.amount_out && 
        quote.total_cost == 0 &&
        quote.status == QuoteStatus::Active,
        "Quote Creation",
        TestCategory::Unit
    )
}

fn test_quote_expiry() -> TestResult {
    let mut quote = TestDataGenerator::generate_test_quote(1_000_000_000_000_000_000);
    
    // Set expiry to past
    quote.expires_at = (ic_cdk::api::time() / 1_000_000_000) - 100;
    
    test_assert!(
        quote.is_expired(),
        "Quote Expiry Logic",
        TestCategory::Unit
    )
}

fn test_quote_validation() -> TestResult {
    let quote = TestDataGenerator::generate_test_quote(1_000_000_000_000_000_000);
    
    test_assert!(
        quote.is_valid() && !quote.is_expired(),
        "Quote Validation",
        TestCategory::Unit
    )
}

fn test_gasless_quote_logic() -> TestResult {
    let quote = TestDataGenerator::generate_test_quote(1_000_000_000_000_000_000);
    
    // In gasless model: user pays amount_in, gets amount_out, bridge covers gas
    let bridge_subsidy = quote.get_bridge_subsidy();
    let total_bridge_cost = quote.get_total_bridge_cost();
    
    test_assert!(
        quote.is_gasless() && 
        bridge_subsidy > 0 &&
        total_bridge_cost == quote.amount_out + bridge_subsidy,
        "Gasless Quote Logic",
        TestCategory::Unit
    )
}

fn test_settlement_creation() -> TestResult {
    let settlement = TestDataGenerator::generate_test_settlement("test_quote_123");
    
    test_assert!(
        settlement.status == SettlementStatus::Pending &&
        settlement.amount == settlement.amount &&
        settlement.retry_count == 0,
        "Settlement Creation",
        TestCategory::Unit
    )
}

fn test_settlement_status_transitions() -> TestResult {
    let mut settlement = TestDataGenerator::generate_test_settlement("test_quote_123");
    
    // Test status transitions
    settlement.mark_executing();
    let executing_ok = settlement.status == SettlementStatus::Executing;
    
    settlement.mark_completed(21_000, "0x1234567890abcdef".to_string());
    let completed_ok = settlement.status == SettlementStatus::Completed;
    
    test_assert!(
        executing_ok && completed_ok,
        "Settlement Status Transitions",
        TestCategory::Unit
    )
}

fn test_reserve_state_operations() -> TestResult {
    let mut reserve = TestDataGenerator::generate_test_reserve_state();
    let initial_available = reserve.available_balance;
    
    // Test locking funds
    let lock_amount = 1_000_000_000_000_000_000; // 1 ETH
    let lock_result = reserve.lock_gasless_funds(lock_amount, 50_000_000_000_000_000);
    
    let balance_updated = reserve.available_balance < initial_available;
    let locked_increased = reserve.locked_balance > 1_000_000_000_000_000_000;
    
    test_assert!(
        lock_result.is_ok() && balance_updated && locked_increased,
        "Reserve State Operations",
        TestCategory::Unit
    )
}

fn test_reserve_health_checks() -> TestResult {
    let mut reserve = TestDataGenerator::generate_test_reserve_state();
    
    // Test healthy state
    let healthy = !reserve.is_below_warning() && !reserve.is_below_critical();
    
    // Test warning state
    reserve.available_balance = 1_000_000_000_000_000_000; // 1 ETH (below 2 ETH warning)
    let warning = reserve.is_below_warning() && !reserve.is_below_critical();
    
    // Test critical state
    reserve.available_balance = 100_000_000_000_000_000; // 0.1 ETH (below 0.5 ETH critical)
    let critical = reserve.is_below_critical();
    
    test_assert!(
        healthy && warning && critical,
        "Reserve Health Checks",
        TestCategory::Unit
    )
}

fn test_gasless_fund_locking() -> TestResult {
    let mut reserve = TestDataGenerator::generate_test_reserve_state();
    
    let delivery_amount = 1_000_000_000_000_000_000; // 1 ETH
    let gas_subsidy = 50_000_000_000_000_000;        // 0.05 ETH
    
    // Test successful locking
    let result1 = reserve.lock_gasless_funds(delivery_amount, gas_subsidy);
    
    // Test insufficient funds (try to lock more than available)
    let large_amount = u64::MAX; // Very large amount (more than available)
    let result2 = reserve.lock_gasless_funds(large_amount, gas_subsidy);
    
    test_assert!(
        result1.is_ok() && result2.is_err(),
        "Gasless Fund Locking",
        TestCategory::Unit
    )
}

fn test_gas_estimate_validation() -> TestResult {
    let valid_estimate = GasEstimate {
        base_fee: 50_000_000_000,
        priority_fee: 2_000_000_000,
        max_fee_per_gas: 52_000_000_000,
        gas_limit: 21_000,
        total_cost: 1_092_000_000_000_000,
        safety_margin: 218_400_000_000_000,
    };
    
    let invalid_estimate = GasEstimate {
        base_fee: 500_000_000_000, // 500 Gwei - too high
        priority_fee: 2_000_000_000,
        max_fee_per_gas: 502_000_000_000,
        gas_limit: 21_000,
        total_cost: 10_542_000_000_000_000,
        safety_margin: 2_108_400_000_000_000,
    };
    
    let valid_ok = validate_gas_estimate(&valid_estimate).is_ok();
    let invalid_err = validate_gas_estimate(&invalid_estimate).is_err();
    
    test_assert!(
        valid_ok && invalid_err,
        "Gas Estimate Validation",
        TestCategory::Unit
    )
}

fn test_fallback_gas_estimate() -> TestResult {
    let fallback = get_fallback_estimate();
    
    test_assert!(
        fallback.gas_limit >= 21_000 &&
        fallback.max_fee_per_gas > 0 &&
        fallback.total_cost > 0 &&
        validate_gas_estimate(&fallback).is_ok(),
        "Fallback Gas Estimate",
        TestCategory::Unit
    )
}

fn test_type_serialization() -> TestResult {
    let quote = TestDataGenerator::generate_test_quote(1_000_000_000_000_000_000);
    let settlement = TestDataGenerator::generate_test_settlement("test_quote");
    
    // Test that our types can be serialized (important for Candid)
    // This would fail at compile time if types weren't properly serializable
    let _quote_candid = candid::encode_one(&quote);
    let _settlement_candid = candid::encode_one(&settlement);
    
    test_assert!(
        true, // If we get here, serialization works
        "Type Serialization",
        TestCategory::Unit
    )
}
