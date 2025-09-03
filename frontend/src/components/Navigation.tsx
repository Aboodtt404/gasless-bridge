import { useState } from 'react'
import { Link, useLocation } from 'react-router-dom'
import { motion, AnimatePresence } from 'framer-motion'
import { 
  Home, 
  ArrowRightLeft, 
  BarChart3, 
  BookOpen, 
  Menu, 
  X,
  Zap,
  Shield,
  Activity
} from 'lucide-react'

import { useCanisterStore } from '@hooks/useCanisterStore'
import { useAuth } from '@hooks/useAuth'

// OISY-style auth services
import { signIn, signOut } from '../services/auth.services'

const Navigation = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false)
  const location = useLocation()
  const { isConnected, reserveStatus, error } = useCanisterStore()
  const { isAuthenticated, principal, isLoading } = useAuth()

  // OISY-style auth handlers
  const handleSignIn = async () => {
    const result = await signIn({ domain: 'ic0.app' })
    if (result.success === 'error') {
      console.error('Sign in failed:', result.err)
    }
  }

  const handleSignOut = async () => {
    await signOut({ resetUrl: false, clearAllPrincipalsStorages: false })
  }

  const navItems = [
    { path: '/', label: 'Home', icon: Home },
    { path: '/bridge', label: 'Bridge', icon: ArrowRightLeft },
    { path: '/stats', label: 'Stats', icon: BarChart3 },
    { path: '/docs', label: 'Docs', icon: BookOpen },
  ]

  const isActivePath = (path: string) => location.pathname === path

  return (
    <>
      <nav className="fixed top-0 left-0 right-0 z-50 bg-dark-900/80 backdrop-blur-xl border-b border-dark-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            {/* Logo */}
            <Link to="/" className="flex items-center space-x-3 group">
              <motion.div
                whileHover={{ rotate: 180 }}
                transition={{ duration: 0.3 }}
                className="w-8 h-8 bg-gradient-to-r from-primary-500 to-secondary-500 rounded-lg flex items-center justify-center"
              >
                <Zap className="w-5 h-5 text-white" />
              </motion.div>
              <div className="hidden sm:block">
                <h1 className="text-xl font-bold gradient-text">HyperBridge</h1>
                <p className="text-xs text-dark-400 -mt-1">Gasless Bridge</p>
              </div>
            </Link>

            {/* Desktop Navigation */}
            <div className="hidden md:flex items-center space-x-1">
              {navItems.map(({ path, label, icon: Icon }) => (
                <Link
                  key={path}
                  to={path}
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 flex items-center space-x-2 ${
                    isActivePath(path)
                      ? 'bg-primary-600 text-white shadow-lg'
                      : 'text-dark-300 hover:text-white hover:bg-dark-700'
                  }`}
                >
                  <Icon className="w-4 h-4" />
                  <span>{label}</span>
                </Link>
              ))}
            </div>

            {/* Status Indicators */}
            <div className="hidden lg:flex items-center space-x-4">
              {/* Connection Status */}
              <div className="flex items-center space-x-2">
                <div className={`w-2 h-2 rounded-full ${
                  isConnected ? 'bg-green-400' : error ? 'bg-red-400' : 'bg-yellow-400 animate-pulse'
                }`} />
                <span className="text-xs text-dark-400">
                  {isConnected ? 'Connected' : error ? 'Error' : 'Connecting...'}
                </span>
              </div>

              {/* Reserve Health */}
              {reserveStatus && (
                <div className="flex items-center space-x-2">
                  <Shield className={`w-4 h-4 ${
                    reserveStatus.health_status === 'Healthy' ? 'text-green-400' :
                    reserveStatus.health_status === 'Warning' ? 'text-yellow-400' : 'text-red-400'
                  }`} />
                  <span className="text-xs text-dark-400">
                    {reserveStatus.health_status}
                  </span>
                </div>
              )}

              {/* Performance Indicator */}
              <motion.div
                className="flex items-center space-x-2 cursor-pointer group"
                whileHover={{ scale: 1.05 }}
              >
                <Activity className="w-4 h-4 text-primary-400" />
                <span className="text-xs text-dark-400 group-hover:text-primary-400 transition-colors">
                  Cached
                </span>
              </motion.div>

              {/* Login/Logout Button */}
              <div className="border-l border-dark-600 pl-4">
                {isAuthenticated ? (
                  <div className="flex items-center space-x-3">
                    <div className="text-xs text-dark-400">
                      {principal?.slice(0, 8)}...
                    </div>
                    <button
                      onClick={handleSignOut}
                      className="text-xs bg-dark-700 hover:bg-dark-600 px-3 py-1 rounded transition-colors"
                    >
                      Logout
                    </button>
                  </div>
                ) : (
                  <button
                    onClick={handleSignIn}
                    disabled={isLoading}
                    className="text-xs bg-primary-600 hover:bg-primary-700 disabled:opacity-50 text-white px-3 py-1 rounded transition-colors"
                  >
                    {isLoading ? 'Connecting...' : 'Connect'}
                  </button>
                )}
              </div>
            </div>

            {/* Mobile Menu Button */}
            <button
              onClick={() => setIsMenuOpen(!isMenuOpen)}
              className="md:hidden p-2 rounded-lg text-dark-300 hover:text-white hover:bg-dark-700 transition-colors"
            >
              {isMenuOpen ? <X className="w-5 h-5" /> : <Menu className="w-5 h-5" />}
            </button>
          </div>
        </div>

        {/* Mobile Menu */}
        <AnimatePresence>
          {isMenuOpen && (
            <motion.div
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              exit={{ opacity: 0, height: 0 }}
              className="md:hidden bg-dark-800 border-t border-dark-700"
            >
              <div className="px-4 py-2 space-y-1">
                {navItems.map(({ path, label, icon: Icon }) => (
                  <Link
                    key={path}
                    to={path}
                    onClick={() => setIsMenuOpen(false)}
                    className={`flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                      isActivePath(path)
                        ? 'bg-primary-600 text-white'
                        : 'text-dark-300 hover:text-white hover:bg-dark-700'
                    }`}
                  >
                    <Icon className="w-4 h-4" />
                    <span>{label}</span>
                  </Link>
                ))}
                
                {/* Mobile Status */}
                <div className="pt-2 mt-2 border-t border-dark-600">
                  <div className="px-3 py-2 text-xs text-dark-400 space-y-1">
                    <div className="flex items-center justify-between">
                      <span>Connection:</span>
                      <span className={
                        isConnected ? 'text-green-400' : error ? 'text-red-400' : 'text-yellow-400'
                      }>
                        {isConnected ? 'Connected' : error ? 'Error' : 'Connecting...'}
                      </span>
                    </div>
                    {reserveStatus && (
                      <div className="flex items-center justify-between">
                        <span>Reserve:</span>
                        <span className={
                          reserveStatus.health_status === 'Healthy' ? 'text-green-400' :
                          reserveStatus.health_status === 'Warning' ? 'text-yellow-400' : 'text-red-400'
                        }>
                          {reserveStatus.health_status}
                        </span>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </nav>

      {/* Spacer to prevent content from hiding under fixed nav */}
      <div className="h-16" />
    </>
  )
}

export default Navigation
