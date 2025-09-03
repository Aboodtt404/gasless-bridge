// OISY-style auth services for HyperBridge
import { useAuth, type AuthSignInParams } from '../hooks/useAuth'

// OISY-style auth service interfaces
export interface AuthServiceResult {
  success: 'ok' | 'cancelled' | 'error'
  err?: unknown
}

// OISY-style busy state management (simplified for React)
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
  clean: () => {
    // Clear previous messages
    console.log('Clearing previous toast messages')
  },
  error: ({ msg, err }: { msg: { text: string }, err: unknown }) => {
    console.error('Toast Error:', msg.text, err)
    // In a real implementation, this would show a toast notification
  },
  show: ({ text, level }: { text: string, level: 'success' | 'error' | 'warn' }) => {
    console.log(`Toast ${level}:`, text)
    // In a real implementation, this would show a toast notification
  }
}

// OISY-style i18n (simplified)
const i18n = {
  auth: {
    error: {
      error_while_signing_in: 'Error while signing in',
      no_internet_identity: 'No Internet Identity found'
    },
    warning: {
      not_signed_in: 'Not signed in',
      session_expired: 'Session expired'
    }
  }
}

// OISY-style signIn function
export const signIn = async (
  params: AuthSignInParams
): Promise<AuthServiceResult> => {
  busy.show()

  try {
    const { signIn: authSignIn } = useAuth.getState()
    await authSignIn(params)

    // We clean previous messages in case user was signed out automatically before sign-in again.
    toasts.clean()

    return { success: 'ok' }
  } catch (err: unknown) {
    if (err === 'UserInterrupt') {
      // We do not display an error if user explicitly cancelled the process of sign-in
      return { success: 'cancelled' }
    }

    toasts.error({
      msg: { text: i18n.auth.error.error_while_signing_in },
      err
    })

    return { success: 'error', err }
  } finally {
    busy.stop()
  }
}

// OISY-style signOut function
export const signOut = ({
  resetUrl = false,
  clearAllPrincipalsStorages = false
}: {
  resetUrl?: boolean
  clearAllPrincipalsStorages?: boolean
}): Promise<void> => {
  return logout({ resetUrl, clearAllPrincipalsStorages })
}

// OISY-style error sign out
export const errorSignOut = (text: string): Promise<void> => {
  return logout({
    msg: {
      text,
      level: 'error'
    }
  })
}

// OISY-style warning sign out
export const warnSignOut = (text: string): Promise<void> => {
  return logout({
    msg: {
      text,
      level: 'warn'
    }
  })
}

// OISY-style nullish sign out
export const nullishSignOut = (): Promise<void> =>
  warnSignOut(i18n.auth.warning.not_signed_in)

// OISY-style idle sign out
export const idleSignOut = (): Promise<void> => {
  return logout({
    msg: {
      text: i18n.auth.warning.session_expired,
      level: 'warn'
    },
    clearCurrentPrincipalStorages: false
  })
}

// OISY-style lock session
export const lockSession = ({ resetUrl = false }: { resetUrl?: boolean }): Promise<void> =>
  logout({
    resetUrl,
    clearCurrentPrincipalStorages: false
  })

// OISY-style logout implementation
const logout = async ({
  msg = undefined,
  clearCurrentPrincipalStorages = true,
  clearAllPrincipalsStorages = false,
  resetUrl = false
}: {
  msg?: { text: string; level: 'success' | 'error' | 'warn' }
  clearCurrentPrincipalStorages?: boolean
  clearAllPrincipalsStorages?: boolean
  resetUrl?: boolean
}) => {
  // To mask not operational UI (a side effect of sometimes slow JS loading after window.reload because of service worker and no cache).
  busy.start()

  // In a real implementation, we would clear IndexedDB stores here
  // For now, we'll just log the cleanup actions
  if (clearCurrentPrincipalStorages) {
    console.log('Clearing current principal storages')
  }
  if (clearAllPrincipalsStorages) {
    console.log('Clearing all principals storages')
  }

  // Clear session storage
  if (typeof window !== 'undefined') {
    sessionStorage.clear()
  }

  // Sign out from auth store
  const { signOut: authSignOut } = useAuth.getState()
  await authSignOut()

  if (msg) {
    appendMsgToUrl(msg)
  }

  if (resetUrl) {
    // Reset URL to root
    if (typeof window !== 'undefined') {
      window.location.href = '/'
    }
  }

  // We reload the page to make sure all the states are cleared
  if (typeof window !== 'undefined') {
    window.location.reload()
  }
}

// OISY-style URL message handling
const PARAM_MSG = 'msg'
const PARAM_LEVEL = 'level'

/**
 * If a message was provided to the logout process - e.g. a message informing the logout happened because the session timed-out - append the information to the url as query params
 */
const appendMsgToUrl = (msg: { text: string; level: 'success' | 'error' | 'warn' }) => {
  if (typeof window === 'undefined') {
    return
  }

  const { text, level } = msg

  const url: URL = new URL(window.location.href)

  url.searchParams.append(PARAM_MSG, encodeURI(text))
  url.searchParams.append(PARAM_LEVEL, level)

  // Replace history without reload
  window.history.replaceState({}, '', url.toString())
}

/**
 * If the url contains a msg that has been provided on logout, display it as a toast message. Cleanup url afterwards - we don't want the user to see the message again if reloads the browser
 */
export const displayAndCleanLogoutMsg = () => {
  if (typeof window === 'undefined') {
    return
  }

  const urlParams: URLSearchParams = new URLSearchParams(window.location.search)

  const msg: string | null = urlParams.get(PARAM_MSG)

  if (msg === null) {
    return
  }

  // For simplicity reason we assume the level pass as query params is one of the type ToastLevel
  const level: 'success' | 'error' | 'warn' = (urlParams.get(PARAM_LEVEL) as 'success' | 'error' | 'warn' | null) ?? 'success'

  toasts.show({ text: decodeURI(msg), level })

  cleanUpMsgUrl()
}

const cleanUpMsgUrl = () => {
  if (typeof window === 'undefined') {
    return
  }

  const url: URL = new URL(window.location.href)

  url.searchParams.delete(PARAM_MSG)
  url.searchParams.delete(PARAM_LEVEL)

  window.history.replaceState({}, '', url.toString())
}

// Export busy state for components to use
export { busy }
