use std::cell::RefCell;
use candid::{Principal, CandidType, Deserialize};
use ic_cdk::api::time;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableVec,
    storable::Storable,
};
use serde::Serialize;
use std::borrow::Cow;

use crate::types::{
    settlement::Settlement,
    quote::Quote,
    user_transaction::{UserTransaction, TransactionStatus},
    audit_log::AuditLogEntry,
    // sponsorship::SponsorshipStatus, // Temporarily disabled
    icp_payment::IcpPayment,
};

// Memory IDs following OISY pattern
const CONFIG_MEMORY_ID: MemoryId = MemoryId::new(0);
const USER_TRANSACTIONS_MEMORY_ID: MemoryId = MemoryId::new(1);
const SETTLEMENTS_MEMORY_ID: MemoryId = MemoryId::new(2);
const QUOTES_MEMORY_ID: MemoryId = MemoryId::new(3);
const AUDIT_LOGS_MEMORY_ID: MemoryId = MemoryId::new(4);
const ICP_PAYMENTS_MEMORY_ID: MemoryId = MemoryId::new(5);
const RESERVE_STATE_MEMORY_ID: MemoryId = MemoryId::new(6);

// Professional state management following OISY patterns
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = 
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    
    // User transactions storage - key: (principal, transaction_id)
    static USER_TRANSACTIONS: RefCell<StableBTreeMap<(Principal, String), UserTransaction, VirtualMemory<DefaultMemoryImpl>>> = 
        MEMORY_MANAGER.with(|mm| RefCell::new(StableBTreeMap::new(
            mm.borrow().get(USER_TRANSACTIONS_MEMORY_ID)
        )));
    
    // Settlements storage
    static SETTLEMENTS: RefCell<StableBTreeMap<String, Settlement, VirtualMemory<DefaultMemoryImpl>>> = 
        MEMORY_MANAGER.with(|mm| RefCell::new(StableBTreeMap::new(
            mm.borrow().get(SETTLEMENTS_MEMORY_ID)
        )));
    
    // Quotes storage
    static QUOTES: RefCell<StableBTreeMap<String, Quote, VirtualMemory<DefaultMemoryImpl>>> = 
        MEMORY_MANAGER.with(|mm| RefCell::new(StableBTreeMap::new(
            mm.borrow().get(QUOTES_MEMORY_ID)
        )));
    
    // Audit logs storage
    static AUDIT_LOGS: RefCell<StableVec<AuditLogEntry, VirtualMemory<DefaultMemoryImpl>>> = 
        MEMORY_MANAGER.with(|mm| RefCell::new(StableVec::new(
            mm.borrow().get(AUDIT_LOGS_MEMORY_ID)
        ).unwrap()));
    
    // ICP payments storage
    static ICP_PAYMENTS: RefCell<StableBTreeMap<String, IcpPayment, VirtualMemory<DefaultMemoryImpl>>> = 
        MEMORY_MANAGER.with(|mm| RefCell::new(StableBTreeMap::new(
            mm.borrow().get(ICP_PAYMENTS_MEMORY_ID)
        )));
    
    // Reserve state storage
    static RESERVE_STATE: RefCell<StableBTreeMap<String, ReserveState, VirtualMemory<DefaultMemoryImpl>>> = 
        MEMORY_MANAGER.with(|mm| RefCell::new(StableBTreeMap::new(
            mm.borrow().get(RESERVE_STATE_MEMORY_ID)
        )));
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ReserveState {
    pub available_balance: u64,
    pub locked_balance: u64,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub last_updated: u64,
    pub health_status: String, // "Healthy", "Warning", "Critical"
    pub daily_limit: u64,
    pub daily_used: u64,
    pub last_reset: u64,
}

impl Default for ReserveState {
    fn default() -> Self {
        Self {
            available_balance: 0,
            locked_balance: 0,
            total_deposited: 0,
            total_withdrawn: 0,
            last_updated: time(),
            health_status: "Healthy".to_string(),
            daily_limit: 100_000_000_000_000_000, // 100 ETH daily limit (in wei)
            daily_used: 0,
            last_reset: time(),
        }
    }
}

// Implement Storable for ReserveState
impl Storable for ReserveState {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
    
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }
}

// Professional state management functions
pub struct ProfessionalStateManager;

impl ProfessionalStateManager {
    // === USER TRANSACTIONS ===
    
    pub fn store_user_transaction(principal: Principal, transaction: UserTransaction) -> Result<(), String> {
        USER_TRANSACTIONS.with(|transactions| {
            let key = (principal, transaction.id.clone());
            transactions.borrow_mut().insert(key, transaction);
        });
        Ok(())
    }
    
    pub fn get_user_transaction(principal: Principal, transaction_id: &str) -> Option<UserTransaction> {
        USER_TRANSACTIONS.with(|transactions| {
            let key = (principal, transaction_id.to_string());
            transactions.borrow().get(&key)
        })
    }
    
    pub fn get_user_transactions(principal: Principal) -> Vec<UserTransaction> {
        USER_TRANSACTIONS.with(|transactions| {
            transactions.borrow()
                .iter()
                .filter(|((p, _), _)| *p == principal)
                .map(|(_, transaction)| transaction)
                .collect()
        })
    }
    
    pub fn update_user_transaction_status(
        principal: Principal, 
        transaction_id: &str, 
        status: TransactionStatus,
        transaction_hash: Option<String>,
        completed_at: Option<u64>
    ) -> Result<(), String> {
        USER_TRANSACTIONS.with(|transactions| {
            let key = (principal, transaction_id.to_string());
            if let Some(mut transaction) = transactions.borrow().get(&key) {
                transaction.status = status;
                if let Some(hash) = transaction_hash {
                    transaction.transaction_hash = Some(hash);
                }
                if let Some(completed) = completed_at {
                    transaction.completed_at = Some(completed);
                }
                transactions.borrow_mut().insert(key, transaction);
                Ok(())
            } else {
                Err("Transaction not found".to_string())
            }
        })
    }
    
    // === SETTLEMENTS ===
    
    pub fn store_settlement(settlement: Settlement) -> Result<(), String> {
        SETTLEMENTS.with(|settlements| {
            settlements.borrow_mut().insert(settlement.id.clone(), settlement);
        });
        Ok(())
    }
    
    pub fn get_settlement(settlement_id: &str) -> Option<Settlement> {
        SETTLEMENTS.with(|settlements| {
            settlements.borrow().get(&settlement_id.to_string())
        })
    }
    
    pub fn get_all_settlements() -> Vec<Settlement> {
        SETTLEMENTS.with(|settlements| {
            settlements.borrow().iter().map(|(_, settlement)| settlement).collect()
        })
    }
    
    // === QUOTES ===
    
    pub fn store_quote(quote: Quote) -> Result<(), String> {
        QUOTES.with(|quotes| {
            quotes.borrow_mut().insert(quote.id.clone(), quote);
        });
        Ok(())
    }
    
    pub fn get_quote(quote_id: &str) -> Option<Quote> {
        QUOTES.with(|quotes| {
            quotes.borrow().get(&quote_id.to_string())
        })
    }
    
    pub fn get_user_quotes(principal: Principal) -> Vec<Quote> {
        QUOTES.with(|quotes| {
            quotes.borrow()
                .iter()
                .filter(|(_, quote)| quote.user_principal == principal)
                .map(|(_, quote)| quote)
                .collect()
        })
    }
    
    // === AUDIT LOGGING ===
    
    pub fn log_audit_event(
        event_type: &str,
        details: &str,
        user_principal: Option<Principal>,
        amount_eth: Option<u64>,
        amount_icp: Option<u64>,
        transaction_hash: Option<String>,
    ) -> Result<(), String> {
        let audit_entry = AuditLogEntry {
            id: format!("audit_{}_{}", event_type, time()),
            event_type: event_type.to_string(),
            details: details.to_string(),
            user_principal,
            amount_eth,
            amount_icp,
            transaction_hash,
            timestamp: time(),
        };
        
        AUDIT_LOGS.with(|logs| {
            let _ = logs.borrow_mut().push(&audit_entry);
        });
        
        Ok(())
    }
    
    pub fn get_audit_logs(limit: Option<usize>) -> Vec<AuditLogEntry> {
        AUDIT_LOGS.with(|logs| {
            let logs_ref = logs.borrow();
            let total_logs = logs_ref.len();
            let limit = limit.unwrap_or(total_logs as usize);
            let start = if total_logs > limit as u64 { total_logs - limit as u64 } else { 0 };
            
            (start..total_logs)
                .filter_map(|i| logs_ref.get(i))
                .collect()
        })
    }
    
    // === ICP PAYMENTS ===
    
    pub fn store_icp_payment(payment: IcpPayment) -> Result<(), String> {
        ICP_PAYMENTS.with(|payments| {
            payments.borrow_mut().insert(payment.payment_id.clone(), payment);
        });
        Ok(())
    }
    
    pub fn get_icp_payment(payment_id: &str) -> Option<IcpPayment> {
        ICP_PAYMENTS.with(|payments| {
            payments.borrow().get(&payment_id.to_string())
        })
    }
    
    // === RESERVE STATE ===
    
    pub fn get_reserve_state() -> ReserveState {
        RESERVE_STATE.with(|state| {
            state.borrow().get(&"main".to_string()).unwrap_or_default()
        })
    }
    
    pub fn update_reserve_state<F>(f: F) -> Result<(), String> 
    where 
        F: FnOnce(&mut ReserveState) -> Result<(), String>
    {
        RESERVE_STATE.with(|state| {
            let mut current_state = state.borrow().get(&"main".to_string()).unwrap_or_default();
            f(&mut current_state)?;
            current_state.last_updated = time();
            state.borrow_mut().insert("main".to_string(), current_state);
            Ok(())
        })
    }
    
    pub fn lock_reserve_funds(amount: u64) -> Result<(), String> {
        Self::update_reserve_state(|state| {
            if state.available_balance < amount {
                return Err("Insufficient reserve balance".to_string());
            }
            
            state.available_balance -= amount;
            state.locked_balance += amount;
            
            // Update health status
            state.health_status = if state.available_balance < 1_000_000_000_000_000_000 { // 1 ETH
                "Critical".to_string()
            } else if state.available_balance < 5_000_000_000_000_000_000 { // 5 ETH
                "Warning".to_string()
            } else {
                "Healthy".to_string()
            };
            
            Ok(())
        })
    }
    
    pub fn unlock_reserve_funds(amount: u64) -> Result<(), String> {
        Self::update_reserve_state(|state| {
            if state.locked_balance < amount {
                return Err("Insufficient locked balance".to_string());
            }
            
            state.locked_balance -= amount;
            state.available_balance += amount;
            
            // Update health status
            state.health_status = if state.available_balance < 1_000_000_000_000_000_000 { // 1 ETH
                "Critical".to_string()
            } else if state.available_balance < 5_000_000_000_000_000_000 { // 5 ETH
                "Warning".to_string()
            } else {
                "Healthy".to_string()
            };
            
            Ok(())
        })
    }
    
    pub fn add_reserve_funds(amount: u64) -> Result<(), String> {
        Self::update_reserve_state(|state| {
            state.available_balance += amount;
            state.total_deposited += amount;
            
            // Update health status
            state.health_status = if state.available_balance < 1_000_000_000_000_000_000 { // 1 ETH
                "Critical".to_string()
            } else if state.available_balance < 5_000_000_000_000_000_000 { // 5 ETH
                "Warning".to_string()
            } else {
                "Healthy".to_string()
            };
            
            Ok(())
        })
    }
    
    // === STATISTICS AND MONITORING ===
    
    pub fn get_bridge_statistics() -> BridgeStatistics {
        let reserve_state = Self::get_reserve_state();
        let total_transactions = USER_TRANSACTIONS.with(|transactions| {
            transactions.borrow().len()
        });
        let total_settlements = SETTLEMENTS.with(|settlements| {
            settlements.borrow().len()
        });
        
        BridgeStatistics {
            total_transactions: total_transactions as u64,
            total_settlements: total_settlements as u64,
            reserve_balance: reserve_state.available_balance,
            locked_balance: reserve_state.locked_balance,
            health_status: reserve_state.health_status,
            daily_used: reserve_state.daily_used,
            daily_limit: reserve_state.daily_limit,
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct BridgeStatistics {
    pub total_transactions: u64,
    pub total_settlements: u64,
    pub reserve_balance: u64,
    pub locked_balance: u64,
    pub health_status: String,
    pub daily_used: u64,
    pub daily_limit: u64,
}
