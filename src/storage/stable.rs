use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, StableCell, Storable,
};
use std::borrow::Cow;
use std::cell::RefCell;
use candid::{CandidType, Deserialize, Encode, Decode};
use crate::types::{Quote, Settlement, Transfer};
use crate::storage::state::{ReserveState, BridgeConfig};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Memory IDs for different data types
const QUOTES_MEMORY_ID: MemoryId = MemoryId::new(0);
const SETTLEMENTS_MEMORY_ID: MemoryId = MemoryId::new(1);
const TRANSFERS_MEMORY_ID: MemoryId = MemoryId::new(2);
const RESERVE_MEMORY_ID: MemoryId = MemoryId::new(3);
const CONFIG_MEMORY_ID: MemoryId = MemoryId::new(4);
const ADMINS_MEMORY_ID: MemoryId = MemoryId::new(5);
const AUDIT_LOG_MEMORY_ID: MemoryId = MemoryId::new(6);

// Implement Storable for our types
impl Storable for Quote {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for Settlement {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for Transfer {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for ReserveState {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for BridgeConfig {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AdminList {
    pub admins: Vec<candid::Principal>,
}

impl Storable for AdminList {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AuditLogEntry {
    pub id: u64,
    pub timestamp: u64,
    pub event_type: String,
    pub details: String,
    pub user_principal: Option<candid::Principal>,
}

impl Storable for AuditLogEntry {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// Global memory manager and stable collections
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Stable storage collections
    static QUOTES: RefCell<StableBTreeMap<String, Quote, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(QUOTES_MEMORY_ID)),
        )
    );

    static SETTLEMENTS: RefCell<StableBTreeMap<String, Settlement, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(SETTLEMENTS_MEMORY_ID)),
        )
    );

    static TRANSFERS: RefCell<StableBTreeMap<String, Transfer, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(TRANSFERS_MEMORY_ID)),
        )
    );

    static RESERVE_STATE: RefCell<StableCell<ReserveState, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(RESERVE_MEMORY_ID)),
            ReserveState::new()
        ).unwrap()
    );

    static BRIDGE_CONFIG: RefCell<StableCell<BridgeConfig, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(CONFIG_MEMORY_ID)),
            BridgeConfig::default()
        ).unwrap()
    );

    static ADMIN_LIST: RefCell<StableCell<AdminList, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(ADMINS_MEMORY_ID)),
            AdminList { admins: Vec::new() }
        ).unwrap()
    );

    static AUDIT_LOG: RefCell<StableBTreeMap<u64, AuditLogEntry, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(AUDIT_LOG_MEMORY_ID)),
        )
    );
}

// Stable storage interface functions
pub mod stable_storage {
    use super::*;

    // Quote operations
    pub fn insert_quote(quote: Quote) {
        let quote_id = quote.id.clone();
        let amount_requested = quote.amount_requested;
        let user_principal = quote.user_principal;
        
        QUOTES.with(|quotes| {
            quotes.borrow_mut().insert(quote_id.clone(), quote);
        });
        
        // Add audit log entry
        add_audit_log_entry("QUOTE_CREATED", &format!("Quote {} created for {} wei", 
            quote_id, amount_requested), Some(user_principal));
    }

    pub fn get_quote(quote_id: &str) -> Option<Quote> {
        QUOTES.with(|quotes| {
            quotes.borrow().get(&quote_id.to_string())
        })
    }

    pub fn get_all_quotes() -> Vec<Quote> {
        QUOTES.with(|quotes| {
            quotes.borrow().iter().map(|(_, quote)| quote).collect()
        })
    }

    // Settlement operations
    pub fn insert_settlement(settlement: Settlement) {
        let settlement_id = settlement.id.clone();
        let quote_id = settlement.quote_id.clone();
        let user_principal = settlement.user_principal;
        
        SETTLEMENTS.with(|settlements| {
            settlements.borrow_mut().insert(settlement_id.clone(), settlement);
        });
        
        add_audit_log_entry("SETTLEMENT_CREATED", &format!("Settlement {} created for quote {}", 
            settlement_id, quote_id), Some(user_principal));
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

    // Transfer operations
    pub fn insert_transfer(transfer: Transfer) {
        let transfer_id = transfer.id.clone();
        let settlement_id = transfer.settlement_id.clone();
        
        TRANSFERS.with(|transfers| {
            transfers.borrow_mut().insert(transfer_id.clone(), transfer);
        });
        
        add_audit_log_entry("TRANSFER_CREATED", &format!("Transfer {} created for settlement {}", 
            transfer_id, settlement_id), None);
    }

    pub fn get_transfer(transfer_id: &str) -> Option<Transfer> {
        TRANSFERS.with(|transfers| {
            transfers.borrow().get(&transfer_id.to_string())
        })
    }

    // Reserve operations
    pub fn get_reserve_state() -> ReserveState {
        RESERVE_STATE.with(|reserve| {
            reserve.borrow().get().clone()
        })
    }

    pub fn update_reserve_state<F>(f: F) 
    where 
        F: FnOnce(&mut ReserveState),
    {
        RESERVE_STATE.with(|reserve| {
            let mut state = reserve.borrow().get().clone();
            f(&mut state);
            reserve.borrow_mut().set(state).unwrap();
        });
        
        add_audit_log_entry("RESERVE_UPDATED", "Reserve state modified", None);
    }

    // Config operations
    pub fn get_bridge_config() -> BridgeConfig {
        BRIDGE_CONFIG.with(|config| {
            config.borrow().get().clone()
        })
    }

    pub fn update_bridge_config(new_config: BridgeConfig) {
        BRIDGE_CONFIG.with(|config| {
            config.borrow_mut().set(new_config).unwrap();
        });
        
        add_audit_log_entry("CONFIG_UPDATED", "Bridge configuration updated", None);
    }

    // Admin operations
    pub fn get_admins() -> Vec<candid::Principal> {
        ADMIN_LIST.with(|admins| {
            admins.borrow().get().admins.clone()
        })
    }

    pub fn add_admin(principal: candid::Principal) {
        ADMIN_LIST.with(|admins| {
            let mut admin_list = admins.borrow().get().clone();
            if !admin_list.admins.contains(&principal) {
                admin_list.admins.push(principal);
                admins.borrow_mut().set(admin_list).unwrap();
            }
        });
        
        add_audit_log_entry("ADMIN_ADDED", &format!("New admin added: {}", principal), None);
    }

    pub fn is_admin(principal: &candid::Principal) -> bool {
        ADMIN_LIST.with(|admins| {
            admins.borrow().get().admins.contains(principal)
        })
    }

    // Audit log operations
    pub fn add_audit_log_entry(event_type: &str, details: &str, user_principal: Option<candid::Principal>) {
        let entry = AuditLogEntry {
            id: ic_cdk::api::time(),
            timestamp: ic_cdk::api::time() / 1_000_000_000,
            event_type: event_type.to_string(),
            details: details.to_string(),
            user_principal,
        };

        AUDIT_LOG.with(|log| {
            log.borrow_mut().insert(entry.id, entry);
        });
    }

    pub fn get_audit_log_entries(limit: usize) -> Vec<AuditLogEntry> {
        AUDIT_LOG.with(|log| {
            log.borrow()
                .iter()
                .rev()
                .take(limit)
                .map(|(_, entry)| entry)
                .collect()
        })
    }

    // Initialize stable storage with existing data (for migration)
    pub fn initialize_from_memory_state(
        quotes: std::collections::HashMap<String, Quote>,
        settlements: std::collections::HashMap<String, Settlement>,
        transfers: std::collections::HashMap<String, Transfer>,
        reserve: ReserveState,
        config: BridgeConfig,
        admins: Vec<candid::Principal>,
    ) {
        // Migrate quotes
        for (id, quote) in quotes {
            QUOTES.with(|q| {
                q.borrow_mut().insert(id, quote);
            });
        }

        // Migrate settlements
        for (id, settlement) in settlements {
            SETTLEMENTS.with(|s| {
                s.borrow_mut().insert(id, settlement);
            });
        }

        // Migrate transfers
        for (id, transfer) in transfers {
            TRANSFERS.with(|t| {
                t.borrow_mut().insert(id, transfer);
            });
        }

        // Set reserve state
        RESERVE_STATE.with(|r| {
            r.borrow_mut().set(reserve).unwrap();
        });

        // Set config
        BRIDGE_CONFIG.with(|c| {
            c.borrow_mut().set(config).unwrap();
        });

        // Set admins
        ADMIN_LIST.with(|a| {
            a.borrow_mut().set(AdminList { admins }).unwrap();
        });

        add_audit_log_entry("SYSTEM_MIGRATION", "State migrated to stable storage", None);
    }
}
