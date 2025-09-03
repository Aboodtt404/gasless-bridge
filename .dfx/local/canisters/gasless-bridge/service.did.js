export const idlFactory = ({ IDL }) => {
  const SettlementStatus = IDL.Variant({
    'Failed' : IDL.Null,
    'Executing' : IDL.Null,
    'Completed' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const Settlement = IDL.Record({
    'id' : IDL.Text,
    'destination_address' : IDL.Text,
    'last_error' : IDL.Opt(IDL.Text),
    'status' : SettlementStatus,
    'user_principal' : IDL.Principal,
    'transaction_hash' : IDL.Opt(IDL.Text),
    'destination_chain' : IDL.Text,
    'retry_count' : IDL.Nat32,
    'created_at' : IDL.Nat64,
    'payment_proof' : IDL.Text,
    'quote_id' : IDL.Text,
    'gas_used' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat64,
  });
  const BurnOperationStatus = IDL.Variant({
    'Failed' : IDL.Null,
    'Executing' : IDL.Null,
    'Burning' : IDL.Null,
    'Completed' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const ChainKeyTokenType = IDL.Variant({
    'ckDAI' : IDL.Null,
    'ckETH' : IDL.Null,
    'ckUSDC' : IDL.Null,
    'ckUSDT' : IDL.Null,
    'ckWBTC' : IDL.Null,
    'Custom' : IDL.Text,
  });
  const ChainKeyBurnOperation = IDL.Record({
    'id' : IDL.Text,
    'destination_address' : IDL.Text,
    'status' : BurnOperationStatus,
    'user_principal' : IDL.Principal,
    'created_at' : IDL.Nat64,
    'ethereum_tx_hash' : IDL.Opt(IDL.Text),
    'completed_at' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat64,
    'token_type' : ChainKeyTokenType,
  });
  const MintOperationStatus = IDL.Variant({
    'Failed' : IDL.Null,
    'Completed' : IDL.Null,
    'Verifying' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const ChainKeyMintOperation = IDL.Record({
    'id' : IDL.Text,
    'status' : MintOperationStatus,
    'user_principal' : IDL.Principal,
    'created_at' : IDL.Nat64,
    'ethereum_tx_hash' : IDL.Text,
    'completed_at' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat64,
    'token_type' : ChainKeyTokenType,
  });
  const TransactionStatus = IDL.Variant({
    'Failed' : IDL.Null,
    'Refunded' : IDL.Null,
    'Processing' : IDL.Null,
    'Completed' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const UserTransaction = IDL.Record({
    'id' : IDL.Text,
    'destination_address' : IDL.Text,
    'status' : TransactionStatus,
    'user_principal' : IDL.Principal,
    'transaction_hash' : IDL.Opt(IDL.Text),
    'destination_chain' : IDL.Text,
    'icp_payment_id' : IDL.Text,
    'created_at' : IDL.Nat64,
    'amount_eth' : IDL.Nat64,
    'amount_icp' : IDL.Nat64,
    'gas_sponsored' : IDL.Nat64,
    'completed_at' : IDL.Opt(IDL.Nat64),
  });
  const AuditLogEntry = IDL.Record({
    'id' : IDL.Text,
    'user_principal' : IDL.Opt(IDL.Principal),
    'transaction_hash' : IDL.Opt(IDL.Text),
    'timestamp' : IDL.Nat64,
    'details' : IDL.Text,
    'amount' : IDL.Opt(IDL.Nat64),
    'event_type' : IDL.Text,
    'admin_principal' : IDL.Opt(IDL.Principal),
  });
  const PriceData = IDL.Record({
    'asset' : IDL.Text,
    'source' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'confidence' : IDL.Float64,
    'price_usd' : IDL.Float64,
  });
  const BridgeStatistics = IDL.Record({
    'total_settlements' : IDL.Nat64,
    'reserve_balance' : IDL.Nat64,
    'locked_balance' : IDL.Nat64,
    'daily_used' : IDL.Nat64,
    'health_status' : IDL.Text,
    'total_transactions' : IDL.Nat64,
    'daily_limit' : IDL.Nat64,
  });
  const BridgeConfig = IDL.Record({
    'max_quote_amount' : IDL.Nat64,
    'min_quote_amount' : IDL.Nat64,
    'quote_validity_minutes' : IDL.Nat64,
    'max_gas_price' : IDL.Nat64,
    'safety_margin_percent' : IDL.Nat32,
    'supported_chains' : IDL.Vec(IDL.Text),
  });
  const DetailedReserveStatus = IDL.Record({
    'utilization_percent' : IDL.Float64,
    'threshold_warning' : IDL.Nat64,
    'balance' : IDL.Nat64,
    'daily_volume' : IDL.Nat64,
    'pending_withdrawals' : IDL.Nat64,
    'locked' : IDL.Nat64,
    'can_accept_quotes' : IDL.Bool,
    'available' : IDL.Nat64,
    'health_status' : IDL.Text,
    'last_topup' : IDL.Nat64,
    'threshold_critical' : IDL.Nat64,
    'daily_limit' : IDL.Nat64,
  });
  const PriceSource = IDL.Record({
    'status' : IDL.Text,
    'name' : IDL.Text,
    'confidence' : IDL.Float64,
    'price_usd' : IDL.Float64,
  });
  const PriceFeedStatus = IDL.Record({
    'cache_status' : IDL.Text,
    'last_updated' : IDL.Nat64,
    'icp_sources' : IDL.Vec(PriceSource),
    'eth_sources' : IDL.Vec(PriceSource),
  });
  const ReserveState = IDL.Record({
    'available_balance' : IDL.Nat64,
    'total_deposited' : IDL.Nat64,
    'locked_balance' : IDL.Nat64,
    'last_updated' : IDL.Nat64,
    'daily_used' : IDL.Nat64,
    'last_reset' : IDL.Nat64,
    'health_status' : IDL.Text,
    'daily_limit' : IDL.Nat64,
    'total_withdrawn' : IDL.Nat64,
  });
  const QuoteStatus = IDL.Variant({
    'Active' : IDL.Null,
    'Expired' : IDL.Null,
    'Settled' : IDL.Null,
  });
  const Quote = IDL.Record({
    'id' : IDL.Text,
    'destination_address' : IDL.Text,
    'source_chain' : IDL.Text,
    'status' : QuoteStatus,
    'base_fee' : IDL.Nat64,
    'user_principal' : IDL.Principal,
    'destination_chain' : IDL.Text,
    'safety_margin' : IDL.Nat64,
    'amount_requested' : IDL.Nat64,
    'priority_fee' : IDL.Nat64,
    'gas_estimate' : IDL.Nat64,
    'total_cost' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'max_fee_per_gas' : IDL.Nat64,
    'amount_out' : IDL.Nat64,
    'amount_in' : IDL.Nat64,
    'expires_at' : IDL.Nat64,
  });
  const ReserveStatus = IDL.Variant({
    'Healthy' : IDL.Null,
    'Critical' : IDL.Null,
    'Emergency' : IDL.Null,
    'Warning' : IDL.Null,
  });
  const SponsorshipStatus = IDL.Record({
    'can_sponsor' : IDL.Bool,
    'reserve_health' : IDL.Text,
    'gas_coverage' : IDL.Text,
    'estimated_cost_eth' : IDL.Nat64,
    'estimated_cost_icp' : IDL.Nat64,
  });
  return IDL.Service({
    'add_admin' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'add_reserve_funds' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'add_test_reserve_funds' : IDL.Func([], [IDL.Text], []),
    'admin_add_reserve_funds' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'admin_emergency_pause' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'admin_emergency_unpause' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'admin_set_daily_limit' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'admin_set_reserve_thresholds' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'bridge_assets' : IDL.Func(
        [IDL.Nat64, IDL.Text, IDL.Text],
        [IDL.Variant({ 'Ok' : Settlement, 'Err' : IDL.Text })],
        [],
      ),
    'calculate_icp_cost_for_eth' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text })],
        [],
      ),
    'can_accept_new_quotes' : IDL.Func([], [IDL.Bool], []),
    'check_quote_expiry' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'check_reserve_health' : IDL.Func([], [IDL.Text], []),
    'clear_rpc_cache' : IDL.Func([], [IDL.Text], []),
    'complete_cketh_burn_operation' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'complete_cketh_mint_operation' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'create_cketh_burn_operation' : IDL.Func(
        [IDL.Nat64, IDL.Text],
        [IDL.Variant({ 'Ok' : ChainKeyBurnOperation, 'Err' : IDL.Text })],
        [],
      ),
    'create_cketh_mint_operation' : IDL.Func(
        [IDL.Nat64, IDL.Text],
        [IDL.Variant({ 'Ok' : ChainKeyMintOperation, 'Err' : IDL.Text })],
        [],
      ),
    'create_icp_payment' : IDL.Func(
        [IDL.Nat64, IDL.Text, IDL.Text],
        [IDL.Variant({ 'Ok' : UserTransaction, 'Err' : IDL.Text })],
        [],
      ),
    'estimate_quote_cost' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'estimate_reserve_runway' : IDL.Func([], [IDL.Text], []),
    'get_admin_status' : IDL.Func([], [IDL.Vec(IDL.Principal)], []),
    'get_audit_logs' : IDL.Func([IDL.Nat32], [IDL.Vec(AuditLogEntry)], []),
    'get_best_eth_price' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : PriceData, 'Err' : IDL.Text })],
        [],
      ),
    'get_best_icp_price' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : PriceData, 'Err' : IDL.Text })],
        [],
      ),
    'get_bridge_ethereum_address' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'get_bridge_statistics' : IDL.Func([], [BridgeStatistics], []),
    'get_bridge_status' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'get_chain_key_service_status' : IDL.Func([], [IDL.Text], []),
    'get_cketh_burn_operation' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : ChainKeyBurnOperation, 'Err' : IDL.Text })],
        [],
      ),
    'get_cketh_mint_operation' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : ChainKeyMintOperation, 'Err' : IDL.Text })],
        [],
      ),
    'get_config' : IDL.Func([], [BridgeConfig], []),
    'get_conversion_rate' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Float64, 'Err' : IDL.Text })],
        [],
      ),
    'get_detailed_reserve_status' : IDL.Func([], [DetailedReserveStatus], []),
    'get_eth_price_usd' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Float64, 'Err' : IDL.Text })],
        [],
      ),
    'get_icp_price_usd' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Float64, 'Err' : IDL.Text })],
        [],
      ),
    'get_price_feed_status' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : PriceFeedStatus, 'Err' : IDL.Text })],
        [],
      ),
    'get_professional_reserve_status' : IDL.Func([], [ReserveState], []),
    'get_quote' : IDL.Func([IDL.Text], [IDL.Opt(Quote)], []),
    'get_reserve_status' : IDL.Func([], [ReserveStatus], []),
    'get_reserve_status_formatted' : IDL.Func([], [IDL.Text], []),
    'get_reserve_utilization' : IDL.Func([], [IDL.Float64], []),
    'get_rpc_cache_stats' : IDL.Func([], [IDL.Text], []),
    'get_settlement' : IDL.Func([IDL.Text], [IDL.Opt(Settlement)], []),
    'get_settlement_by_quote' : IDL.Func([IDL.Text], [IDL.Opt(Settlement)], []),
    'get_settlement_statistics' : IDL.Func([], [IDL.Text], []),
    'get_sponsorship_status' : IDL.Func(
        [IDL.Nat64, IDL.Text],
        [IDL.Variant({ 'Ok' : SponsorshipStatus, 'Err' : IDL.Text })],
        [],
      ),
    'get_supported_chain_key_tokens' : IDL.Func([], [IDL.Text], []),
    'get_user_cketh_operations' : IDL.Func(
        [],
        [
          IDL.Record({
            'mint_operations' : IDL.Vec(ChainKeyMintOperation),
            'burn_operations' : IDL.Vec(ChainKeyBurnOperation),
          }),
        ],
        [],
      ),
    'get_user_icp_balance' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text })],
        [],
      ),
    'get_user_quotes' : IDL.Func([], [IDL.Vec(Quote)], []),
    'get_user_settlements' : IDL.Func([], [IDL.Vec(Settlement)], []),
    'get_user_transaction' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(UserTransaction)],
        [],
      ),
    'get_user_transactions' : IDL.Func([], [IDL.Vec(UserTransaction)], []),
    'health_check' : IDL.Func([], [IDL.Text], []),
    'invalidate_gas_cache' : IDL.Func([], [IDL.Text], []),
    'request_quote' : IDL.Func(
        [IDL.Nat64, IDL.Text, IDL.Text],
        [IDL.Variant({ 'Ok' : Quote, 'Err' : IDL.Text })],
        [],
      ),
    'run_chain_key_token_tests' : IDL.Func([], [IDL.Text], []),
    'run_comprehensive_test_suite' : IDL.Func([], [IDL.Text], []),
    'run_edge_case_tests' : IDL.Func([], [IDL.Text], []),
    'run_integration_tests' : IDL.Func([], [IDL.Text], []),
    'run_performance_tests' : IDL.Func([], [IDL.Text], []),
    'run_security_tests' : IDL.Func([], [IDL.Text], []),
    'run_unit_tests' : IDL.Func([], [IDL.Text], []),
    'settle_quote' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Variant({ 'Ok' : Settlement, 'Err' : IDL.Text })],
        [],
      ),
    'test_base_rpc' : IDL.Func([], [IDL.Text], []),
    'test_complete_bridge_flow' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'test_complete_gasless_settlement' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'test_enhanced_rpc_client' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'test_gasless_bridge_demo' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'test_rpc_health_monitoring' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'test_settlement_flow' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'test_threshold_ecdsa_integration' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'test_transaction_building' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'update_config' : IDL.Func(
        [BridgeConfig],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
