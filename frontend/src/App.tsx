import React, { Suspense, useEffect } from 'react'
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { motion, AnimatePresence } from 'framer-motion'

// Components
import Navigation from '@components/Navigation'
import LoadingSpinner from '@components/LoadingSpinner'
import Footer from '@components/Footer'

// Pages
import HomePage from '@components/pages/HomePage'
import BridgePage from '@components/pages/BridgePage'
import StatsPage from '@components/pages/StatsPage'
import DocsPage from '@components/pages/DocsPage'

// Hooks
import { useCanisterStore } from '@hooks/useCanisterStore'
import { useAuth } from '@hooks/useAuth'

// OISY-style services
import { displayAndCleanLogoutMsg } from './services/auth.services'

// 3D Background Component (lazy loaded)
const BridgeBackground = React.lazy(() => import('@components/3d/BridgeBackground'))

function App() {
  const { initializeCanister, isConnected, error, retryConnection, isLoading } = useCanisterStore()
  const { sync } = useAuth()

  useEffect(() => {
    // OISY-style initialization
    const init = async () => {
      // Display any logout messages from URL params
      displayAndCleanLogoutMsg()
      
      // Sync auth state (OISY pattern)
      await sync()
      
      // Initialize canister connection
      await initializeCanister()
    }
    init()
  }, [sync, initializeCanister])

  return (
    <Router>
      <div className="min-h-screen bg-dark-900 text-white relative overflow-hidden">
        {/* 3D Background */}
        <div className="fixed inset-0 -z-10">
          <Suspense fallback={<div className="bg-gradient-to-br from-dark-900 via-dark-800 to-primary-900/20 h-full w-full" />}>
            <BridgeBackground />
          </Suspense>
        </div>

        {/* Main Layout */}
        <div className="relative z-10">
          <Navigation />
          
          {/* Connection Status Indicator */}
          <AnimatePresence>
            {(!isConnected || error) && (
              <motion.div
                initial={{ opacity: 0, y: -50 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -50 }}
                className="fixed top-20 right-4 z-50"
              >
                <div className={`${error ? 'bg-red-500/90' : 'bg-orange-500/90'} backdrop-blur-sm text-white px-4 py-2 rounded-lg shadow-lg max-w-sm`}>
                  <div className="flex items-center justify-between space-x-2">
                    <div className="flex items-center space-x-2">
                      <div className={`w-2 h-2 rounded-full ${error ? 'bg-red-300' : 'bg-orange-300'} ${isLoading ? 'animate-pulse' : ''}`} />
                      <span className="text-sm font-medium">
                        {error ? 'Connection Failed' : 'Connecting to ICP...'}
                      </span>
                    </div>
                    {error && (
                      <button
                        onClick={retryConnection}
                        disabled={isLoading}
                        className="text-xs bg-white/20 hover:bg-white/30 px-2 py-1 rounded transition-colors disabled:opacity-50"
                      >
                        Retry
                      </button>
                    )}
                  </div>
                  {error && (
                    <div className="mt-2 text-xs text-white/80">
                      {error}
                    </div>
                  )}
                </div>
              </motion.div>
            )}
          </AnimatePresence>

          {/* Main Content */}
          <main className="relative">
            <Suspense fallback={<LoadingSpinner />}>
              <Routes>
                <Route path="/" element={<HomePage />} />
                <Route path="/bridge" element={<BridgePage />} />
                <Route path="/stats" element={<StatsPage />} />
                <Route path="/docs" element={<DocsPage />} />
                <Route path="*" element={<NotFoundPage />} />
              </Routes>
            </Suspense>
          </main>

          <Footer />
        </div>

        {/* Global Loading Overlay */}
        <AnimatePresence>
          {/* Add global loading states here if needed */}
        </AnimatePresence>
      </div>
    </Router>
  )
}

// 404 Page Component
const NotFoundPage = () => (
  <div className="min-h-screen flex items-center justify-center">
    <motion.div
      initial={{ opacity: 0, scale: 0.8 }}
      animate={{ opacity: 1, scale: 1 }}
      className="text-center"
    >
      <h1 className="text-6xl font-bold gradient-text mb-4">404</h1>
      <h2 className="text-2xl font-semibold mb-4">Bridge Not Found</h2>
      <p className="text-dark-400 mb-8">
        The page you're looking for doesn't exist in this dimension.
      </p>
      <motion.a
        href="/"
        className="btn-primary inline-block"
        whileHover={{ scale: 1.05 }}
        whileTap={{ scale: 0.95 }}
      >
        Return to Bridge
      </motion.a>
    </motion.div>
  </div>
)

export default App
