// OISY-style bridge services for HyperBridge
import { useCanisterStore } from '../hooks/useCanisterStore'
import { useBridgeStore } from '../stores/bridge.store'
import type { BridgeQuote, BridgeSettlement } from '../stores/bridge.store'

// OISY-style progress steps (simplified for automatic settlement)
export enum ProgressStepsBridge {
  BRIDGE = 'BRIDGE',
  EXECUTE = 'EXECUTE',
  COMPLETE = 'COMPLETE',
  UPDATE_UI = 'UPDATE_UI'
}

// OISY-style bridge error codes (simplified for automatic settlement)
export enum BridgeErrorCodes {
  BRIDGE_FAILED = 'BRIDGE_FAILED',
  EXECUTION_FAILED = 'EXECUTION_FAILED',
  INSUFFICIENT_FUNDS = 'INSUFFICIENT_FUNDS',
  INVALID_ADDRESS = 'INVALID_ADDRESS',
  RESERVE_UNAVAILABLE = 'RESERVE_UNAVAILABLE'
}

// OISY-style bridge error interface
export interface BridgeError {
  variant: 'error' | 'warning' | 'info'
  message: string
  url?: { url: string; text: string }
  errorType?: string
  bridgeSucceeded?: boolean
}

// OISY-style bridge parameters
export interface BridgeParams {
  amount: bigint
  destinationAddress: string
  destinationChain: string
  progress: (step: ProgressStepsBridge) => void
  setFailedProgressStep?: (step: ProgressStepsBridge) => void
}

// OISY-style bridge result
export interface BridgeResult {
  quote?: BridgeQuote
  settlement?: BridgeSettlement
  success: boolean
  error?: BridgeError
}

// OISY-style busy state (simplified for React)
const busyCallbacks = new Set<() => void>()

const busy = {
  show: () => {
    busyCallbacks.forEach(cb => cb())
  },
  stop: () => {
    busyCallbacks.forEach(cb => cb())
  },
  start: () => {
    busyCallbacks.forEach(cb => cb())
  }
}

// OISY-style toast management (simplified)
const toasts = {
  error: ({ msg, err }: { msg: { text: string }, err: unknown }) => {
    console.error('Bridge Error:', msg.text, err)
    // In a real implementation, this would show a toast notification
  },
  success: ({ text }: { text: string }) => {
    console.log('Bridge Success:', text)
    // In a real implementation, this would show a toast notification
  }
}

// OISY-style i18n (simplified for automatic settlement)
const i18n = {
  bridge: {
    error: {
      bridge_failed: 'Failed to bridge assets',
      execution_failed: 'Failed to execute bridge transaction',
      insufficient_funds: 'Insufficient funds for bridge operation',
      invalid_address: 'Invalid destination address',
      reserve_unavailable: 'Bridge reserve is currently unavailable'
    },
    success: {
      assets_bridged: 'Assets bridged successfully',
      transaction_executed: 'Bridge transaction executed successfully'
    }
  }
}

// OISY-style automatic bridge service implementation
export const bridgeAssets = async ({
  amount,
  destinationAddress,
  destinationChain,
  progress,
  setFailedProgressStep
}: BridgeParams): Promise<BridgeResult> => {
  progress(ProgressStepsBridge.BRIDGE)
  busy.show()

  try {
    const { bridgeAssets: bridgeAssetsCanister } = useCanisterStore.getState()
    
    if (!bridgeAssetsCanister) {
      throw new Error('Bridge service not available')
    }

    const settlement = await bridgeAssetsCanister(amount, destinationAddress, destinationChain)
    
    if (!settlement) {
      setFailedProgressStep?.(ProgressStepsBridge.BRIDGE)
      return {
        success: false,
        error: {
          variant: 'error',
          message: i18n.bridge.error.bridge_failed,
          errorType: BridgeErrorCodes.BRIDGE_FAILED
        }
      }
    }

    // Add settlement to store
    const { addSettlement } = useBridgeStore.getState()
    addSettlement(settlement)

    progress(ProgressStepsBridge.COMPLETE)
    toasts.success({ text: i18n.bridge.success.assets_bridged })

    return {
      settlement,
      success: true
    }

  } catch (err: unknown) {
    setFailedProgressStep?.(ProgressStepsBridge.BRIDGE)
    
    toasts.error({
      msg: { text: i18n.bridge.error.bridge_failed },
      err
    })

    return {
      success: false,
      error: {
        variant: 'error',
        message: i18n.bridge.error.bridge_failed,
        errorType: BridgeErrorCodes.BRIDGE_FAILED
      }
    }
  } finally {
    busy.stop()
  }
}

// Legacy functions for backward compatibility (deprecated)
export const fetchBridgeQuote = async (params: BridgeParams): Promise<BridgeResult> => {
  console.warn('fetchBridgeQuote is deprecated, use bridgeAssets instead')
  return bridgeAssets(params)
}

export const fetchBridgeSettlement = async (_params: any): Promise<BridgeResult> => {
  console.warn('fetchBridgeSettlement is deprecated, use bridgeAssets instead')
  return {
    success: false,
    error: {
      variant: 'error',
      message: 'Manual settlement is deprecated, use automatic bridge_assets instead',
      errorType: BridgeErrorCodes.BRIDGE_FAILED
    }
  }
}

// OISY-style complete bridge flow (now just calls bridgeAssets)
export const executeBridgeTransaction = async (params: BridgeParams): Promise<BridgeResult> => {
  return bridgeAssets(params)
}

// OISY-style bridge service object (similar to OISY's swapService)
export const bridgeService = {
  bridgeAssets,
  requestQuote: fetchBridgeQuote, // deprecated
  settleQuote: fetchBridgeSettlement, // deprecated
  executeTransaction: executeBridgeTransaction
}

// Export busy state for components to use
export { busy }
