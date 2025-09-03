import { create } from 'zustand'
import { Identity } from '@dfinity/agent'
import { AuthClient } from '@dfinity/auth-client'

// OISY-style auth constants
const AUTH_MAX_TIME_TO_LIVE = 7 * 24 * 60 * 60 * 1000 * 1000 * 1000 // 7 days in nanoseconds
const AUTH_POPUP_HEIGHT = 800
const AUTH_POPUP_WIDTH = 600
const INTERNET_IDENTITY_CANISTER_ID = process.env.INTERNET_IDENTITY_CANISTER_ID

// OISY-style auth interfaces
export interface AuthSignInParams {
  domain?: 'ic0.app' | 'internetcomputer.org'
}

export interface AuthStoreData {
  identity: Identity | null
  isAuthenticated: boolean
  principal: string | null
  isLoading: boolean
  error: string | null
  authClient: AuthClient | null
}

interface AuthState extends AuthStoreData {
  // OISY-style actions
  sync: () => Promise<void>
  signIn: (params: AuthSignInParams) => Promise<void>
  signOut: () => Promise<void>
  setForTesting: (identity: Identity) => void
  setError: (error: string | null) => void
}

// OISY-style helper functions
const createAuthClient = async (): Promise<AuthClient> => {
  return await AuthClient.create({
    idleOptions: {
      disableIdle: true,
    }
  })
}

const getOptionalDerivationOrigin = () => {
  // OISY-style derivation origin handling
  return {}
}

const popupCenter = ({ width, height }: { width: number; height: number }) => {
  const left = window.screen.width / 2 - width / 2
  const top = window.screen.height / 2 - height / 2
  return `left=${left},top=${top},toolbar=0,scrollbars=1,status=1,resizable=1,location=1,menuBar=0,width=${width},height=${height}`
}

let authClient: AuthClient | null = null

export const useAuth = create<AuthState>((set, get) => {
  // Expose store globally for canister integration (OISY pattern)
  if (typeof window !== 'undefined') {
    (window as any).authStore = { getState: get }
  }
  
  return {
    identity: null,
    isAuthenticated: false,
    principal: null,
    isLoading: false,
    error: null,
    authClient: null,

    // OISY-style sync method
    sync: async () => {
      authClient = authClient ?? (await createAuthClient())
      const isAuthenticated: boolean = await authClient.isAuthenticated()

      set({
        identity: isAuthenticated ? authClient.getIdentity() : null,
        isAuthenticated,
        principal: isAuthenticated ? authClient.getIdentity().getPrincipal().toString() : null,
        authClient,
        isLoading: false,
        error: null
      })
    },

    // OISY-style signIn method
    signIn: ({ domain }: AuthSignInParams) =>
      new Promise<void>(async (resolve, reject) => {
        authClient = authClient ?? (await createAuthClient())

        const identityProvider = INTERNET_IDENTITY_CANISTER_ID
          ? /apple/i.test(navigator?.vendor)
            ? `http://localhost:4943?canisterId=${INTERNET_IDENTITY_CANISTER_ID}`
            : `http://${INTERNET_IDENTITY_CANISTER_ID}.localhost:4943`
          : `https://identity.${domain ?? 'internetcomputer.org'}`

        await authClient?.login({
          maxTimeToLive: BigInt(AUTH_MAX_TIME_TO_LIVE),
          onSuccess: () => {
            const identity = authClient?.getIdentity()
            set({
              identity: identity || null,
              isAuthenticated: !!identity,
              principal: identity?.getPrincipal().toString() || null,
              authClient,
              isLoading: false,
              error: null
            })
            resolve()
          },
          onError: reject,
          identityProvider,
          windowOpenerFeatures: popupCenter({ width: AUTH_POPUP_WIDTH, height: AUTH_POPUP_HEIGHT }),
          ...getOptionalDerivationOrigin()
        })
      }),

    // OISY-style signOut method
    signOut: async () => {
      const client: AuthClient = authClient ?? (await createAuthClient())

      await client.logout()

      // OISY pattern: Clear auth client to fix "sign in -> sign out -> sign in again" flow
      authClient = null

      set({
        identity: null,
        isAuthenticated: false,
        principal: null,
        authClient: null,
        error: null
      })
    },

    // OISY-style testing method
    setForTesting: (identity: Identity) => {
      if (process.env.NODE_ENV !== 'test') {
        throw new Error('This function should only be used in test environment')
      }
      set({ identity, isAuthenticated: true, principal: identity.getPrincipal().toString() })
    },

    setError: (error) => set({ error }),
  }
})

export default useAuth
