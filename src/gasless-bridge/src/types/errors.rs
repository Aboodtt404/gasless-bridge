use candid::{CandidType, Deserialize};
use thiserror::Error;

#[derive(Error, Debug, CandidType, Deserialize, Clone)]
pub enum GaslessBridgeError {
    #[error("RPC request failed: {message}")]
    RpcError { message: String },
    
    #[error("Insufficient reserves: need {required} ETH, have {available} ETH")]
    InsufficientReserves { required: u64, available: u64 },
    
    #[error("Invalid quote: {reason}")]
    InvalidQuote { reason: String },
    
    #[error("Quote expired: expired at {expiry}")]
    QuoteExpired { expiry: u64 },
    
    #[error("Settlement not found: {settlement_id}")]
    SettlementNotFound { settlement_id: String },
    
    #[error("Unauthorized access: {caller} not in admin list")]
    Unauthorized { caller: String },
    
    #[error("Gas estimation failed: {reason}")]
    GasEstimationFailed { reason: String },
    
    #[error("Invalid configuration: {field}")]
    ConfigurationError { field: String },
    
    #[error("Transaction failed: {tx_hash}")]
    TransactionFailed { tx_hash: String },
    
    #[error("Network error: {message}")]
    NetworkError { message: String },
}

pub type BridgeResult<T> = Result<T, GaslessBridgeError>;
