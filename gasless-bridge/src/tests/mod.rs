// Comprehensive Testing Suite for HyperBridge
// Phase 5.1: Production Readiness Testing

pub mod unit_tests;
pub mod integration_tests;
pub mod security_tests;
pub mod edge_case_tests;
pub mod performance_tests;
pub mod chain_key_tests; // 🪙 Chain-key token tests

use candid::Principal;
use crate::types::{Quote, QuoteStatus, Settlement, SettlementStatus};
use crate::storage::state::ReserveState;

/// Test result wrapper for comprehensive reporting
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub message: String,
    pub duration_ms: u64,
    pub category: TestCategory,
}

#[derive(Debug, Clone, Copy)]
pub enum TestCategory {
    Unit,
    Integration,
    Security,
    Performance,
    EdgeCase,
}

/// Test suite runner for all production readiness tests
pub struct TestSuite {
    pub results: Vec<TestResult>,
    pub total_tests: u32,
    pub passed_tests: u32,
    pub failed_tests: u32,
}

impl TestSuite {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        self.total_tests += 1;
        if result.passed {
            self.passed_tests += 1;
        } else {
            self.failed_tests += 1;
        }
        self.results.push(result);
    }

    pub fn get_summary(&self) -> String {
        let pass_rate = if self.total_tests > 0 {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        } else {
            0.0
        };

        format!(
            "🧪 **COMPREHENSIVE TEST SUITE RESULTS** 🧪\n\
            \n\
            📊 **OVERALL RESULTS:**\n\
            • Total Tests: {}\n\
            • Passed: {} ✅\n\
            • Failed: {} ❌\n\
            • Pass Rate: {:.1}%\n\
            \n\
            📈 **TEST CATEGORIES:**\n\
            {}",
            self.total_tests,
            self.passed_tests,
            self.failed_tests,
            pass_rate,
            self.get_category_breakdown()
        )
    }

    fn get_category_breakdown(&self) -> String {
        let mut breakdown = String::new();
        
        for test_category in [
            TestCategory::Unit,
            TestCategory::Integration,
            TestCategory::Security,
            TestCategory::Performance,
            TestCategory::EdgeCase,
        ] {
            let category_results: Vec<&TestResult> = self.results.iter()
                .filter(|r| std::mem::discriminant(&r.category) == std::mem::discriminant(&test_category))
                .collect();
            
            let passed = category_results.iter().filter(|r| r.passed).count();
            let total = category_results.len();
            
            if total > 0 {
                breakdown.push_str(&format!(
                    "• {:?}: {}/{} passed\n",
                    test_category, passed, total
                ));
            }
        }
        
        breakdown
    }

    pub fn get_detailed_report(&self) -> String {
        let mut report = self.get_summary();
        report.push_str("\n\n🔍 **DETAILED RESULTS:**\n");
        
        for result in &self.results {
            let status = if result.passed { "✅" } else { "❌" };
            report.push_str(&format!(
                "{} [{:?}] {} ({}ms): {}\n",
                status,
                result.category,
                result.test_name,
                result.duration_ms,
                result.message
            ));
        }
        
        report
    }
}

/// Helper macros for testing
#[macro_export]
macro_rules! test_assert {
    ($condition:expr, $test_name:expr, $category:expr) => {{
        let start_time = ic_cdk::api::time();
        let passed = $condition;
        let duration = (ic_cdk::api::time() - start_time) / 1_000_000;
        
        TestResult {
            test_name: $test_name.to_string(),
            passed,
            message: if passed {
                "Test passed successfully".to_string()
            } else {
                format!("Assertion failed: {}", stringify!($condition))
            },
            duration_ms: duration,
            category: $category,
        }
    }};
}

#[macro_export]
macro_rules! test_expect {
    ($result:expr, $test_name:expr, $category:expr) => {{
        let start_time = ic_cdk::api::time();
        let duration = (ic_cdk::api::time() - start_time) / 1_000_000;
        
        match $result {
            Ok(_) => TestResult {
                test_name: $test_name.to_string(),
                passed: true,
                message: "Test completed successfully".to_string(),
                duration_ms: duration,
                category: $category,
            },
            Err(e) => TestResult {
                test_name: $test_name.to_string(),
                passed: false,
                message: format!("Test failed: {}", e),
                duration_ms: duration,
                category: $category,
            },
        }
    }};
}

/// Test data generators
pub struct TestDataGenerator;

impl TestDataGenerator {
    pub fn generate_test_principal() -> Principal {
        // Use anonymous principal for testing
        Principal::anonymous()
    }

    pub fn generate_test_quote(amount: u64) -> Quote {
        let current_time = ic_cdk::api::time() / 1_000_000_000;
        
        Quote {
            id: "test_quote_123".to_string(),
            user_principal: Self::generate_test_principal(),
            amount_in: amount,
            amount_out: amount,
            amount_requested: amount,
            total_cost: 0, // Gasless model
            gas_estimate: 21_000,
            destination_address: "0x742d35Cc6Bb06Aa0B89f114EFc1aAd7Be20986a4".to_string(),
            source_chain: "ICP".to_string(),
            destination_chain: "Base Sepolia".to_string(),
            created_at: current_time,
            expires_at: current_time + 900, // 15 minutes
            base_fee: 50_000_000_000,
            priority_fee: 2_000_000_000,
            max_fee_per_gas: 52_000_000_000,
            safety_margin: 343_980_000_000_000,
            status: QuoteStatus::Active,
        }
    }

    pub fn generate_test_settlement(quote_id: &str) -> Settlement {
        Settlement {
            id: format!("test_settlement_{}", ic_cdk::api::time()),
            quote_id: quote_id.to_string(),
            user_principal: Self::generate_test_principal(),
            amount: 1_000_000_000_000_000_000, // 1 ETH
            payment_proof: "test_payment_proof".to_string(),
            destination_address: "0x742d35Cc6Bb06Aa0B89f114EFc1aAd7Be20986a4".to_string(),
            destination_chain: "Base Sepolia".to_string(),
            created_at: ic_cdk::api::time() / 1_000_000_000,
            status: SettlementStatus::Pending,
            gas_used: None,
            transaction_hash: None,
            retry_count: 0,
            last_error: None,
        }
    }

    pub fn generate_test_reserve_state() -> ReserveState {
        ReserveState {
            total_balance: 10_000_000_000_000_000_000, // 10 ETH
            locked_balance: 1_000_000_000_000_000_000,  // 1 ETH
            available_balance: 9_000_000_000_000_000_000, // 9 ETH
            threshold_warning: 2_000_000_000_000_000_000, // 2 ETH
            threshold_critical: 500_000_000_000_000_000,  // 0.5 ETH
            daily_volume: 500_000_000_000_000_000,       // 0.5 ETH
            daily_limit: 5_000_000_000_000_000_000,      // 5 ETH
            last_topup: ic_cdk::api::time() / 1_000_000_000, // Current time
            pending_withdrawals: 0,                       // No pending withdrawals
        }
    }
}
