import { create } from 'zustand'
import { Actor, HttpAgent, AnonymousIdentity } from '@dfinity/agent'
import { Principal } from '@dfinity/principal'

// Types for our backend canister
export interface Quote {
  id: string
  user_principal: Principal
  amount_in: bigint
  amount_out: bigint
  amount_requested: bigint
  total_cost: bigint
  gas_estimate: bigint
  destination_address: string
  source_chain: string
  destination_chain: string
  created_at: bigint
  expires_at: bigint
  base_fee: bigint
  priority_fee: bigint
  max_fee_per_gas: bigint
  safety_margin: bigint
  status: { Active: null } | { Settled: null } | { Expired: null } | { Failed: null }
}

export interface Settlement {
  id: string
  quote_id: string
  user_principal: Principal
  amount: bigint
  destination_address: string
  destination_chain: string
  payment_proof: string
  created_at: bigint
  status: { Pending: null } | { Executing: null } | { Completed: null } | { Failed: null }
  gas_used: bigint | null
  transaction_hash: string | null
  retry_count: number
  last_error: string | null
}

export interface ReserveStatus {
  balance: bigint
  locked: bigint
  available: bigint
  threshold_warning: bigint
  threshold_critical: bigint
  daily_volume: bigint
  daily_limit: bigint
  pending_withdrawals: bigint
  utilization_percent: number
  health_status: string
  can_accept_quotes: boolean
  last_topup: bigint
}

// New types for ICP payments and sponsorship


export interface SponsorshipStatus {
  can_sponsor: boolean
  estimated_cost_icp: bigint
  estimated_cost_eth: bigint
  gas_coverage: string
  reserve_health: string
}

export interface UserTransaction {
  id: string
  user_principal: Principal
  amount_icp: bigint
  amount_eth: bigint
  destination_address: string
  destination_chain: string
  status: { Pending: null } | { Processing: null } | { Completed: null } | { Failed: null } | { Refunded: null }
  created_at: bigint
  completed_at: bigint | null
  transaction_hash: string | null
  gas_sponsored: bigint
  icp_payment_id: string
}

// Canister interface
export interface GaslessBridgeCanister {
  request_quote: (amount: bigint, destination_address: string, destination_chain: string) => Promise<{ Ok: Quote } | { Err: string }>
  bridge_assets: (amount: bigint, destination_address: string, destination_chain: string) => Promise<{ Ok: Settlement } | { Err: string }>
  settle_quote: (quote_id: string, payment_proof: string) => Promise<{ Ok: Settlement } | { Err: string }>
  get_quote_by_id: (quote_id: string) => Promise<{ Ok: Quote } | { Err: string }>
  get_settlement_by_id: (settlement_id: string) => Promise<{ Ok: Settlement } | { Err: string }>
  get_detailed_reserve_status: () => Promise<ReserveStatus>
  get_rpc_cache_stats: () => Promise<{ Ok: string } | { Err: string }>
  clear_rpc_cache: () => Promise<{ Ok: string } | { Err: string }>
  invalidate_gas_cache: () => Promise<{ Ok: string } | { Err: string }>
  run_comprehensive_test_suite: () => Promise<{ Ok: string } | { Err: string }>
  
  // New ICP payment methods
  create_icp_payment: (amount: bigint, destination_address: string, destination_chain: string) => Promise<{ Ok: UserTransaction } | { Err: string }>
  get_sponsorship_status: (amount: bigint, destination_chain: string) => Promise<{ Ok: SponsorshipStatus } | { Err: string }>
  get_user_transactions: () => Promise<UserTransaction[]>
  get_user_transaction: (transaction_id: string) => Promise<UserTransaction | null>
}

interface CanisterStore {
  // Connection state
  agent: HttpAgent | null
  actor: GaslessBridgeCanister | null
  isConnected: boolean
  isLoading: boolean
  error: string | null
  retryCount: number

  // Bridge state
  quotes: Quote[]
  settlements: Settlement[]
  reserveStatus: ReserveStatus | null
  
  // New ICP payment state
  userTransactions: UserTransaction[]
  sponsorshipStatus: SponsorshipStatus | null
  
  // Cache stats
  cacheStats: string | null

  // Actions
  initializeCanister: () => Promise<void>
  retryConnection: () => Promise<void>
  requestQuote: (amount: bigint, destinationAddress: string, destinationChain: string) => Promise<Quote | null>
  bridgeAssets: (amount: bigint, destinationAddress: string, destinationChain: string) => Promise<Settlement | null>
  settleQuote: (quoteId: string, paymentProof: string) => Promise<Settlement | null>
  getQuote: (quoteId: string) => Promise<Quote | null>
  getSettlement: (settlementId: string) => Promise<Settlement | null>
  refreshReserveStatus: () => Promise<void>
  getCacheStats: () => Promise<void>
  clearCache: () => Promise<void>
  invalidateGasCache: () => Promise<void>
  runTests: () => Promise<string | null>
  setError: (error: string | null) => void
  clearError: () => void
  
  // New ICP payment actions
  createIcpPayment: (amount: bigint, destinationAddress: string, destinationChain: string) => Promise<UserTransaction | null>
  getSponsorshipStatus: (amount: bigint, destinationChain: string) => Promise<SponsorshipStatus | null>
  getUserTransactions: () => Promise<UserTransaction[]>
  getUserTransaction: (transactionId: string) => Promise<UserTransaction | null>
}

const CANISTER_ID = 'uxrrr-q7777-77774-qaaaq-cai'
const HOST = process.env.DFX_NETWORK === 'local' ? 'http://localhost:4943' : 'https://ic0.app'

// Candid interface definition (generated from .did file)
const idlFactory = ({ IDL }: any) => {
  const QuoteStatus = IDL.Variant({
    'Active': IDL.Null,
    'Settled': IDL.Null,
    'Expired': IDL.Null,
    'Failed': IDL.Null,
  })
  
  const Quote = IDL.Record({
    'id': IDL.Text,
    'user_principal': IDL.Principal,
    'amount_in': IDL.Nat64,
    'amount_out': IDL.Nat64,
    'amount_requested': IDL.Nat64,
    'total_cost': IDL.Nat64,
    'gas_estimate': IDL.Nat64,
    'destination_address': IDL.Text,
    'source_chain': IDL.Text,
    'destination_chain': IDL.Text,
    'created_at': IDL.Nat64,
    'expires_at': IDL.Nat64,
    'base_fee': IDL.Nat64,
    'priority_fee': IDL.Nat64,
    'max_fee_per_gas': IDL.Nat64,
    'safety_margin': IDL.Nat64,
    'status': QuoteStatus,
  })

  const SettlementStatus = IDL.Variant({
    'Pending': IDL.Null,
    'Executing': IDL.Null,
    'Completed': IDL.Null,
    'Failed': IDL.Null,
  })

  const Settlement = IDL.Record({
    'id': IDL.Text,
    'quote_id': IDL.Text,
    'user_principal': IDL.Principal,
    'amount': IDL.Nat64,
    'destination_address': IDL.Text,
    'destination_chain': IDL.Text,
    'payment_proof': IDL.Text,
    'created_at': IDL.Nat64,
    'status': SettlementStatus,
    'gas_used': IDL.Opt(IDL.Nat64),
    'transaction_hash': IDL.Opt(IDL.Text),
    'retry_count': IDL.Nat32,
    'last_error': IDL.Opt(IDL.Text),
  })

  const ReserveStatus = IDL.Record({
    'balance': IDL.Nat64,
    'locked': IDL.Nat64,
    'available': IDL.Nat64,
    'threshold_warning': IDL.Nat64,
    'threshold_critical': IDL.Nat64,
    'daily_volume': IDL.Nat64,
    'daily_limit': IDL.Nat64,
    'pending_withdrawals': IDL.Nat64,
    'utilization_percent': IDL.Float64,
    'health_status': IDL.Text,
    'can_accept_quotes': IDL.Bool,
    'last_topup': IDL.Nat64,
  })

  const Result = IDL.Variant({ 'Ok': IDL.Text, 'Err': IDL.Text })
  const Result_1 = IDL.Variant({ 'Ok': Quote, 'Err': IDL.Text })
  const Result_2 = IDL.Variant({ 'Ok': Settlement, 'Err': IDL.Text })

  // New types for ICP payments




  const SponsorshipStatus = IDL.Record({
    'can_sponsor': IDL.Bool,
    'estimated_cost_icp': IDL.Nat64,
    'estimated_cost_eth': IDL.Nat64,
    'gas_coverage': IDL.Text,
    'reserve_health': IDL.Text,
  })

  const TransactionStatus = IDL.Variant({
    'Pending': IDL.Null,
    'Processing': IDL.Null,
    'Completed': IDL.Null,
    'Failed': IDL.Null,
    'Refunded': IDL.Null,
  })

  const UserTransaction = IDL.Record({
    'id': IDL.Text,
    'user_principal': IDL.Principal,
    'amount_icp': IDL.Nat64,
    'amount_eth': IDL.Nat64,
    'destination_address': IDL.Text,
    'destination_chain': IDL.Text,
    'status': TransactionStatus,
    'created_at': IDL.Nat64,
    'completed_at': IDL.Opt(IDL.Nat64),
    'transaction_hash': IDL.Opt(IDL.Text),
    'gas_sponsored': IDL.Nat64,
    'icp_payment_id': IDL.Text,
  })

  const Result_4 = IDL.Variant({ 'Ok': UserTransaction, 'Err': IDL.Text })
  const Result_5 = IDL.Variant({ 'Ok': SponsorshipStatus, 'Err': IDL.Text })

  return IDL.Service({
    'request_quote': IDL.Func([IDL.Nat64, IDL.Text, IDL.Text], [Result_1], []),
    'bridge_assets': IDL.Func([IDL.Nat64, IDL.Text, IDL.Text], [Result_2], []),
    'settle_quote': IDL.Func([IDL.Text, IDL.Text], [Result_2], []),
    'get_quote_by_id': IDL.Func([IDL.Text], [Result_1], ['query']),
    'get_settlement_by_id': IDL.Func([IDL.Text], [Result_2], ['query']),
    'get_detailed_reserve_status': IDL.Func([], [ReserveStatus], ['query']),
    'get_rpc_cache_stats': IDL.Func([], [Result], ['query']),
    'clear_rpc_cache': IDL.Func([], [Result], []),
    'invalidate_gas_cache': IDL.Func([], [Result], []),
    'run_comprehensive_test_suite': IDL.Func([], [Result], []),
    
    // New ICP payment methods
    'create_icp_payment': IDL.Func([IDL.Nat64, IDL.Text, IDL.Text], [Result_4], []),
    'get_sponsorship_status': IDL.Func([IDL.Nat64, IDL.Text], [Result_5], []),
    'get_user_transactions': IDL.Func([], [IDL.Vec(UserTransaction)], ['query']),
    'get_user_transaction': IDL.Func([IDL.Text], [IDL.Opt(UserTransaction)], ['query']),
  })
}

export const useCanisterStore = create<CanisterStore>((set, get) => ({
  // Initial state
  agent: null,
  actor: null,
  isConnected: false,
  isLoading: false,
  error: null,
  retryCount: 0,
  quotes: [],
  settlements: [],
  reserveStatus: null,
  userTransactions: [],
  sponsorshipStatus: null,
  cacheStats: null,

  // Initialize canister connection
  initializeCanister: async () => {
    try {
      set({ isLoading: true, error: null })

      console.log('Initializing HyperBridge connection...')
      console.log('Canister ID:', CANISTER_ID)
      console.log('Host:', HOST)

      // Get authenticated identity from auth store
      const authState = (window as any).authStore?.getState()
      const identity = authState?.identity || new AnonymousIdentity()

      const agent = new HttpAgent({ 
        host: HOST,
        identity
      })
      
      // Only fetch root key in development
      if (process.env.DFX_NETWORK === 'local' || HOST.includes('localhost')) {
        await agent.fetchRootKey()
      }

      const actor = Actor.createActor(idlFactory, {
        agent,
        canisterId: CANISTER_ID,
      }) as GaslessBridgeCanister

      // Test connection by getting reserve status
      const reserveStatus = await actor.get_detailed_reserve_status()

      set({ 
        agent, 
        actor, 
        isConnected: true, 
        isLoading: false,
        error: null,
        reserveStatus
      })

      // Load additional data
      const { getCacheStats } = get()
      await getCacheStats()

    } catch (error) {
      console.error('Failed to initialize canister:', error)
      
      // Better error handling with specific error messages
      let errorMessage = 'Failed to connect to HyperBridge'
      if (error instanceof Error) {
        if (error.message.includes('not found')) {
          errorMessage = 'HyperBridge canister not found. Please check if it\'s deployed correctly.'
        } else if (error.message.includes('DestinationInvalid')) {
          errorMessage = 'Invalid canister destination. Please check the canister ID.'
        } else if (error.message.includes('network')) {
          errorMessage = 'Network connection failed. Please check your internet connection.'
        } else {
          errorMessage = `Connection failed: ${error.message}`
        }
      }
      
      set({ 
        isLoading: false, 
        error: errorMessage,
        isConnected: false,
        retryCount: get().retryCount + 1
      })
    }
  },

  // Retry connection
  retryConnection: async () => {
    const { retryCount } = get()
    if (retryCount < 3) {
      console.log(`Retrying connection (attempt ${retryCount + 1}/3)...`)
      await get().initializeCanister()
    } else {
      set({ 
        error: 'Maximum retry attempts reached. Please check your connection and try again.',
        isLoading: false 
      })
    }
  },

  // Request a new quote
  requestQuote: async (amount, destinationAddress, destinationChain) => {
    const { actor } = get()
    if (!actor) {
      set({ error: 'Not connected to canister' })
      return null
    }

    try {
      set({ isLoading: true, error: null })
      const result = await actor.request_quote(amount, destinationAddress, destinationChain)
      
      if ('Ok' in result) {
        const quote = result.Ok
        set(state => ({ 
          quotes: [...state.quotes, quote],
          isLoading: false 
        }))
        return quote
      } else {
        set({ error: result.Err, isLoading: false })
        return null
      }
    } catch (error) {
      set({ error: `Failed to request quote: ${error}`, isLoading: false })
      return null
    }
  },

  // Bridge assets (automatic settlement - OISY pattern)
  bridgeAssets: async (amount, destinationAddress, destinationChain) => {
    const { actor } = get()
    if (!actor) {
      set({ error: 'Not connected to canister' })
      return null
    }

    try {
      set({ isLoading: true, error: null })
      const result = await actor.bridge_assets(amount, destinationAddress, destinationChain)
      
      if ('Ok' in result) {
        const settlement = result.Ok
        set(state => ({ 
          settlements: [...state.settlements, settlement],
          isLoading: false 
        }))
        return settlement
      } else {
        set({ error: result.Err, isLoading: false })
        return null
      }
    } catch (error) {
      set({ error: `Failed to bridge assets: ${error}`, isLoading: false })
      return null
    }
  },

  // Settle a quote (deprecated - use bridgeAssets instead)
  settleQuote: async (quoteId, paymentProof) => {
    const { actor } = get()
    if (!actor) {
      set({ error: 'Not connected to canister' })
      return null
    }

    try {
      set({ isLoading: true, error: null })
      const result = await actor.settle_quote(quoteId, paymentProof)
      
      if ('Ok' in result) {
        const settlement = result.Ok
        set(state => ({ 
          settlements: [...state.settlements, settlement],
          isLoading: false 
        }))
        return settlement
      } else {
        set({ error: result.Err, isLoading: false })
        return null
      }
    } catch (error) {
      set({ error: `Failed to settle quote: ${error}`, isLoading: false })
      return null
    }
  },

  // Get quote by ID
  getQuote: async (quoteId) => {
    const { actor } = get()
    if (!actor) return null

    try {
      const result = await actor.get_quote_by_id(quoteId)
      return 'Ok' in result ? result.Ok : null
    } catch (error) {
      console.error('Failed to get quote:', error)
      return null
    }
  },

  // Get settlement by ID
  getSettlement: async (settlementId) => {
    const { actor } = get()
    if (!actor) return null

    try {
      const result = await actor.get_settlement_by_id(settlementId)
      return 'Ok' in result ? result.Ok : null
    } catch (error) {
      console.error('Failed to get settlement:', error)
      return null
    }
  },

  // Refresh reserve status
  refreshReserveStatus: async () => {
    const { actor } = get()
    if (!actor) return

    try {
      const reserveStatus = await actor.get_detailed_reserve_status()
      set({ reserveStatus })
    } catch (error) {
      console.error('Failed to get reserve status:', error)
    }
  },

  // Get cache stats
  getCacheStats: async () => {
    const { actor } = get()
    if (!actor) return

    try {
      const result = await actor.get_rpc_cache_stats()
      if ('Ok' in result) {
        set({ cacheStats: result.Ok })
      }
    } catch (error) {
      console.error('Failed to get cache stats:', error)
    }
  },

  // Clear cache
  clearCache: async () => {
    const { actor } = get()
    if (!actor) return

    try {
      await actor.clear_rpc_cache()
      // Refresh cache stats
      const { getCacheStats } = get()
      await getCacheStats()
    } catch (error) {
      console.error('Failed to clear cache:', error)
      set({ error: `Failed to clear cache: ${error}` })
    }
  },

  // Invalidate gas cache
  invalidateGasCache: async () => {
    const { actor } = get()
    if (!actor) return

    try {
      await actor.invalidate_gas_cache()
    } catch (error) {
      console.error('Failed to invalidate gas cache:', error)
      set({ error: `Failed to invalidate gas cache: ${error}` })
    }
  },

  // Run comprehensive tests
  runTests: async () => {
    const { actor } = get()
    if (!actor) return null

    try {
      set({ isLoading: true })
      const result = await actor.run_comprehensive_test_suite()
      set({ isLoading: false })
      return 'Ok' in result ? result.Ok : result.Err
    } catch (error) {
      set({ isLoading: false })
      console.error('Failed to run tests:', error)
      return null
    }
  },

  // Error management
  setError: (error) => set({ error }),
  clearError: () => set({ error: null }),

  // New ICP payment methods
  createIcpPayment: async (amount, destinationAddress, destinationChain) => {
    const { actor } = get()
    if (!actor) {
      set({ error: 'Not connected to canister' })
      return null
    }

    try {
      set({ isLoading: true, error: null })
      const result = await actor.create_icp_payment(amount, destinationAddress, destinationChain)
      
      if ('Ok' in result) {
        const transaction = result.Ok
        set(state => ({ 
          userTransactions: [...state.userTransactions, transaction],
          isLoading: false 
        }))
        return transaction
      } else {
        set({ error: result.Err, isLoading: false })
        return null
      }
    } catch (error) {
      set({ error: `Failed to create automatic ICP payment: ${error}`, isLoading: false })
      return null
    }
  },

  getSponsorshipStatus: async (amount, destinationChain) => {
    const { actor } = get()
    if (!actor) {
      set({ error: 'Not connected to canister' })
      return null
    }

    try {
      set({ isLoading: true, error: null })
      const result = await actor.get_sponsorship_status(amount, destinationChain)
      
      if ('Ok' in result) {
        const status = result.Ok
        set({ sponsorshipStatus: status, isLoading: false })
        return status
      } else {
        set({ error: result.Err, isLoading: false })
        return null
      }
    } catch (error) {
      set({ error: `Failed to get sponsorship status: ${error}`, isLoading: false })
      return null
    }
  },

  getUserTransactions: async () => {
    const { actor } = get()
    if (!actor) return []

    try {
      const transactions = await actor.get_user_transactions()
      set({ userTransactions: transactions })
      return transactions
    } catch (error) {
      console.error('Failed to get user transactions:', error)
      return []
    }
  },

  getUserTransaction: async (transactionId) => {
    const { actor } = get()
    if (!actor) return null

    try {
      const transaction = await actor.get_user_transaction(transactionId)
      return transaction
    } catch (error) {
      console.error('Failed to get user transaction:', error)
      return null
    }
  },
}))

export default useCanisterStore
