// OISY-style bridge store for HyperBridge
import { create } from 'zustand'
import type { Principal } from '@dfinity/principal'

// OISY-style types
export interface BridgeQuote {
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

export interface BridgeSettlement {
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

// OISY-style bridge data interface
export interface BridgeData {
  sourceToken?: string
  destinationToken?: string
  amount?: bigint
  destinationAddress?: string
  destinationChain?: string
}

// OISY-style bridge store interface
export interface BridgeStore extends BridgeData {
  // Bridge state
  quotes: BridgeQuote[]
  settlements: BridgeSettlement[]
  reserveStatus: ReserveStatus | null
  isLoading: boolean
  error: string | null
  
  // Actions
  setSourceToken: (token: string) => void
  setDestinationToken: (token: string) => void
  setAmount: (amount: bigint) => void
  setDestinationAddress: (address: string) => void
  setDestinationChain: (chain: string) => void
  addQuote: (quote: BridgeQuote) => void
  addSettlement: (settlement: BridgeSettlement) => void
  setReserveStatus: (status: ReserveStatus) => void
  setLoading: (loading: boolean) => void
  setError: (error: string | null) => void
  clearError: () => void
  resetBridgeData: () => void
}

// OISY-style bridge store implementation
export const useBridgeStore = create<BridgeStore>((set) => ({
  // Initial state
  sourceToken: undefined,
  destinationToken: undefined,
  amount: undefined,
  destinationAddress: undefined,
  destinationChain: undefined,
  quotes: [],
  settlements: [],
  reserveStatus: null,
  isLoading: false,
  error: null,

  // Actions
  setSourceToken: (token: string) => set({ sourceToken: token }),
  setDestinationToken: (token: string) => set({ destinationToken: token }),
  setAmount: (amount: bigint) => set({ amount }),
  setDestinationAddress: (address: string) => set({ destinationAddress: address }),
  setDestinationChain: (chain: string) => set({ destinationChain: chain }),
  
  addQuote: (quote: BridgeQuote) => 
    set(state => ({ quotes: [...state.quotes, quote] })),
  
  addSettlement: (settlement: BridgeSettlement) => 
    set(state => ({ settlements: [...state.settlements, settlement] })),
  
  setReserveStatus: (status: ReserveStatus) => set({ reserveStatus: status }),
  setLoading: (loading: boolean) => set({ isLoading: loading }),
  setError: (error: string | null) => set({ error }),
  clearError: () => set({ error: null }),
  
  resetBridgeData: () => set({
    sourceToken: undefined,
    destinationToken: undefined,
    amount: undefined,
    destinationAddress: undefined,
    destinationChain: undefined,
  }),
}))

// OISY-style derived stores
export const useBridgeDerived = () => {
  const store = useBridgeStore()
  
  // Derived state calculations
  const isBridgeDataComplete = !!(
    store.sourceToken &&
    store.destinationToken &&
    store.amount &&
    store.destinationAddress &&
    store.destinationChain
  )
  
  const canRequestQuote = isBridgeDataComplete && 
    store.reserveStatus?.can_accept_quotes && 
    !store.isLoading
  
  const activeQuotes = store.quotes.filter(quote => 
    'Active' in quote.status
  )
  
  const pendingSettlements = store.settlements.filter(settlement => 
    'Pending' in settlement.status || 'Executing' in settlement.status
  )
  
  const completedSettlements = store.settlements.filter(settlement => 
    'Completed' in settlement.status
  )
  
  const failedSettlements = store.settlements.filter(settlement => 
    'Failed' in settlement.status
  )
  
  return {
    isBridgeDataComplete,
    canRequestQuote,
    activeQuotes,
    pendingSettlements,
    completedSettlements,
    failedSettlements,
  }
}

// OISY-style bridge context (similar to OISY's swap context)
export const createBridgeContext = (bridgeData: BridgeData = {}) => {
  const bridgeStore = useBridgeStore()
  
  // Set initial data
  if (bridgeData.sourceToken) bridgeStore.setSourceToken(bridgeData.sourceToken)
  if (bridgeData.destinationToken) bridgeStore.setDestinationToken(bridgeData.destinationToken)
  if (bridgeData.amount) bridgeStore.setAmount(bridgeData.amount)
  if (bridgeData.destinationAddress) bridgeStore.setDestinationAddress(bridgeData.destinationAddress)
  if (bridgeData.destinationChain) bridgeStore.setDestinationChain(bridgeData.destinationChain)
  
  return {
    bridgeStore,
    ...useBridgeDerived(),
  }
}

export default useBridgeStore
