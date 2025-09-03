import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AuditLogEntry {
  'id' : string,
  'user_principal' : [] | [Principal],
  'transaction_hash' : [] | [string],
  'timestamp' : bigint,
  'details' : string,
  'amount' : [] | [bigint],
  'event_type' : string,
  'admin_principal' : [] | [Principal],
}
export interface BridgeConfig {
  'max_quote_amount' : bigint,
  'min_quote_amount' : bigint,
  'quote_validity_minutes' : bigint,
  'max_gas_price' : bigint,
  'safety_margin_percent' : number,
  'supported_chains' : Array<string>,
}
export interface BridgeStatistics {
  'total_settlements' : bigint,
  'reserve_balance' : bigint,
  'locked_balance' : bigint,
  'daily_used' : bigint,
  'health_status' : string,
  'total_transactions' : bigint,
  'daily_limit' : bigint,
}
export type BurnOperationStatus = { 'Failed' : null } |
  { 'Executing' : null } |
  { 'Burning' : null } |
  { 'Completed' : null } |
  { 'Pending' : null };
export interface ChainKeyBurnOperation {
  'id' : string,
  'destination_address' : string,
  'status' : BurnOperationStatus,
  'user_principal' : Principal,
  'created_at' : bigint,
  'ethereum_tx_hash' : [] | [string],
  'completed_at' : [] | [bigint],
  'amount' : bigint,
  'token_type' : ChainKeyTokenType,
}
export interface ChainKeyMintOperation {
  'id' : string,
  'status' : MintOperationStatus,
  'user_principal' : Principal,
  'created_at' : bigint,
  'ethereum_tx_hash' : string,
  'completed_at' : [] | [bigint],
  'amount' : bigint,
  'token_type' : ChainKeyTokenType,
}
export interface ChainKeyTokenBalance {
  'available_balance' : bigint,
  'locked_balance' : bigint,
  'last_operation' : bigint,
  'token_type' : ChainKeyTokenType,
  'total_supply' : bigint,
}
export interface ChainKeyTokenConfig {
  'decimals' : number,
  'min_amount' : bigint,
  'ethereum_address' : string,
  'gas_limit' : bigint,
  'is_active' : boolean,
  'max_amount' : bigint,
  'token_type' : ChainKeyTokenType,
}
export type ChainKeyTokenType = { 'ckDAI' : null } |
  { 'ckETH' : null } |
  { 'ckUSDC' : null } |
  { 'ckUSDT' : null } |
  { 'ckWBTC' : null } |
  { 'Custom' : string };
export interface DetailedReserveStatus {
  'utilization_percent' : number,
  'threshold_warning' : bigint,
  'balance' : bigint,
  'daily_volume' : bigint,
  'pending_withdrawals' : bigint,
  'locked' : bigint,
  'can_accept_quotes' : boolean,
  'available' : bigint,
  'health_status' : string,
  'last_topup' : bigint,
  'threshold_critical' : bigint,
  'daily_limit' : bigint,
}
export interface IcpPayment {
  'status' : PaymentStatus,
  'user_principal' : Principal,
  'amount_e8s' : bigint,
  'timestamp' : bigint,
  'payment_id' : string,
}
export type MintOperationStatus = { 'Failed' : null } |
  { 'Completed' : null } |
  { 'Verifying' : null } |
  { 'Pending' : null };
export type PaymentStatus = { 'Failed' : null } |
  { 'Confirmed' : null } |
  { 'Pending' : null };
export interface PriceData {
  'asset' : string,
  'source' : string,
  'timestamp' : bigint,
  'confidence' : number,
  'price_usd' : number,
}
export interface PriceFeedStatus {
  'cache_status' : string,
  'last_updated' : bigint,
  'icp_sources' : Array<PriceSource>,
  'eth_sources' : Array<PriceSource>,
}
export interface PriceSource {
  'status' : string,
  'name' : string,
  'confidence' : number,
  'price_usd' : number,
}
export interface Quote {
  'id' : string,
  'destination_address' : string,
  'source_chain' : string,
  'status' : QuoteStatus,
  'base_fee' : bigint,
  'user_principal' : Principal,
  'destination_chain' : string,
  'safety_margin' : bigint,
  'amount_requested' : bigint,
  'priority_fee' : bigint,
  'gas_estimate' : bigint,
  'total_cost' : bigint,
  'created_at' : bigint,
  'max_fee_per_gas' : bigint,
  'amount_out' : bigint,
  'amount_in' : bigint,
  'expires_at' : bigint,
}
export type QuoteStatus = { 'Active' : null } |
  { 'Expired' : null } |
  { 'Settled' : null };
export interface ReserveState {
  'available_balance' : bigint,
  'total_deposited' : bigint,
  'locked_balance' : bigint,
  'last_updated' : bigint,
  'daily_used' : bigint,
  'last_reset' : bigint,
  'health_status' : string,
  'daily_limit' : bigint,
  'total_withdrawn' : bigint,
}
export type ReserveStatus = { 'Healthy' : null } |
  { 'Critical' : null } |
  { 'Emergency' : null } |
  { 'Warning' : null };
export type Result = { 'Ok' : string } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : Quote } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Settlement } |
  { 'Err' : string };
export interface Settlement {
  'id' : string,
  'destination_address' : string,
  'last_error' : [] | [string],
  'status' : SettlementStatus,
  'user_principal' : Principal,
  'transaction_hash' : [] | [string],
  'destination_chain' : string,
  'retry_count' : number,
  'created_at' : bigint,
  'payment_proof' : string,
  'quote_id' : string,
  'gas_used' : [] | [bigint],
  'amount' : bigint,
}
export type SettlementStatus = { 'Failed' : null } |
  { 'Executing' : null } |
  { 'Completed' : null } |
  { 'Pending' : null };
export interface SponsorshipStatus {
  'can_sponsor' : boolean,
  'reserve_health' : string,
  'gas_coverage' : string,
  'estimated_cost_eth' : bigint,
  'estimated_cost_icp' : bigint,
}
export type TransactionStatus = { 'Failed' : null } |
  { 'Refunded' : null } |
  { 'Processing' : null } |
  { 'Completed' : null } |
  { 'Pending' : null };
export interface UserTransaction {
  'id' : string,
  'destination_address' : string,
  'status' : TransactionStatus,
  'user_principal' : Principal,
  'transaction_hash' : [] | [string],
  'destination_chain' : string,
  'icp_payment_id' : string,
  'created_at' : bigint,
  'amount_eth' : bigint,
  'amount_icp' : bigint,
  'gas_sponsored' : bigint,
  'completed_at' : [] | [bigint],
}
export interface _SERVICE {
  'add_admin' : ActorMethod<
    [Principal],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'add_reserve_funds' : ActorMethod<
    [bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'add_test_reserve_funds' : ActorMethod<[], string>,
  'admin_add_reserve_funds' : ActorMethod<
    [bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'admin_emergency_pause' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'admin_emergency_unpause' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'admin_set_daily_limit' : ActorMethod<
    [bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'admin_set_reserve_thresholds' : ActorMethod<
    [bigint, bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'bridge_assets' : ActorMethod<
    [bigint, string, string],
    { 'Ok' : Settlement } |
      { 'Err' : string }
  >,
  'calculate_icp_cost_for_eth' : ActorMethod<
    [bigint],
    { 'Ok' : bigint } |
      { 'Err' : string }
  >,
  'can_accept_new_quotes' : ActorMethod<[], boolean>,
  'check_quote_expiry' : ActorMethod<
    [string],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'check_reserve_health' : ActorMethod<[], string>,
  'clear_rpc_cache' : ActorMethod<[], string>,
  'complete_cketh_burn_operation' : ActorMethod<
    [string],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'complete_cketh_mint_operation' : ActorMethod<
    [string],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'create_cketh_burn_operation' : ActorMethod<
    [bigint, string],
    { 'Ok' : ChainKeyBurnOperation } |
      { 'Err' : string }
  >,
  'create_cketh_mint_operation' : ActorMethod<
    [bigint, string],
    { 'Ok' : ChainKeyMintOperation } |
      { 'Err' : string }
  >,
  'create_icp_payment' : ActorMethod<
    [bigint, string, string],
    { 'Ok' : UserTransaction } |
      { 'Err' : string }
  >,
  'estimate_quote_cost' : ActorMethod<
    [bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'estimate_reserve_runway' : ActorMethod<[], string>,
  'get_admin_status' : ActorMethod<[], Array<Principal>>,
  'get_audit_logs' : ActorMethod<[number], Array<AuditLogEntry>>,
  'get_best_eth_price' : ActorMethod<
    [],
    { 'Ok' : PriceData } |
      { 'Err' : string }
  >,
  'get_best_icp_price' : ActorMethod<
    [],
    { 'Ok' : PriceData } |
      { 'Err' : string }
  >,
  'get_bridge_ethereum_address' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'get_bridge_statistics' : ActorMethod<[], BridgeStatistics>,
  'get_bridge_status' : ActorMethod<[], { 'Ok' : string } | { 'Err' : string }>,
  'get_chain_key_service_status' : ActorMethod<[], string>,
  'get_cketh_burn_operation' : ActorMethod<
    [string],
    { 'Ok' : ChainKeyBurnOperation } |
      { 'Err' : string }
  >,
  'get_cketh_mint_operation' : ActorMethod<
    [string],
    { 'Ok' : ChainKeyMintOperation } |
      { 'Err' : string }
  >,
  'get_config' : ActorMethod<[], BridgeConfig>,
  'get_conversion_rate' : ActorMethod<
    [],
    { 'Ok' : number } |
      { 'Err' : string }
  >,
  'get_detailed_reserve_status' : ActorMethod<[], DetailedReserveStatus>,
  'get_eth_price_usd' : ActorMethod<[], { 'Ok' : number } | { 'Err' : string }>,
  'get_icp_price_usd' : ActorMethod<[], { 'Ok' : number } | { 'Err' : string }>,
  'get_price_feed_status' : ActorMethod<
    [],
    { 'Ok' : PriceFeedStatus } |
      { 'Err' : string }
  >,
  'get_professional_reserve_status' : ActorMethod<[], ReserveState>,
  'get_quote' : ActorMethod<[string], [] | [Quote]>,
  'get_reserve_status' : ActorMethod<[], ReserveStatus>,
  'get_reserve_status_formatted' : ActorMethod<[], string>,
  'get_reserve_utilization' : ActorMethod<[], number>,
  'get_rpc_cache_stats' : ActorMethod<[], string>,
  'get_settlement' : ActorMethod<[string], [] | [Settlement]>,
  'get_settlement_by_quote' : ActorMethod<[string], [] | [Settlement]>,
  'get_settlement_statistics' : ActorMethod<[], string>,
  'get_sponsorship_status' : ActorMethod<
    [bigint, string],
    { 'Ok' : SponsorshipStatus } |
      { 'Err' : string }
  >,
  'get_supported_chain_key_tokens' : ActorMethod<[], string>,
  'get_user_cketh_operations' : ActorMethod<
    [],
    {
      'mint_operations' : Array<ChainKeyMintOperation>,
      'burn_operations' : Array<ChainKeyBurnOperation>,
    }
  >,
  'get_user_icp_balance' : ActorMethod<
    [],
    { 'Ok' : bigint } |
      { 'Err' : string }
  >,
  'get_user_quotes' : ActorMethod<[], Array<Quote>>,
  'get_user_settlements' : ActorMethod<[], Array<Settlement>>,
  'get_user_transaction' : ActorMethod<[string], [] | [UserTransaction]>,
  'get_user_transactions' : ActorMethod<[], Array<UserTransaction>>,
  'health_check' : ActorMethod<[], string>,
  'invalidate_gas_cache' : ActorMethod<[], string>,
  'request_quote' : ActorMethod<
    [bigint, string, string],
    { 'Ok' : Quote } |
      { 'Err' : string }
  >,
  'run_chain_key_token_tests' : ActorMethod<[], string>,
  'run_comprehensive_test_suite' : ActorMethod<[], string>,
  'run_edge_case_tests' : ActorMethod<[], string>,
  'run_integration_tests' : ActorMethod<[], string>,
  'run_performance_tests' : ActorMethod<[], string>,
  'run_security_tests' : ActorMethod<[], string>,
  'run_unit_tests' : ActorMethod<[], string>,
  'settle_quote' : ActorMethod<
    [string, string],
    { 'Ok' : Settlement } |
      { 'Err' : string }
  >,
  'test_base_rpc' : ActorMethod<[], string>,
  'test_complete_bridge_flow' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'test_complete_gasless_settlement' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'test_enhanced_rpc_client' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'test_gasless_bridge_demo' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'test_rpc_health_monitoring' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'test_settlement_flow' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'test_threshold_ecdsa_integration' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'test_transaction_building' : ActorMethod<
    [],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'update_config' : ActorMethod<
    [BridgeConfig],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
