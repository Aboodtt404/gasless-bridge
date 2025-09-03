import { useState, useEffect } from 'react'
import { motion } from 'framer-motion'
import { ArrowRightLeft, Wallet, Check, AlertCircle, Info, CreditCard, History } from 'lucide-react'

import { useCanisterStore } from '@hooks/useCanisterStore'
import { useAuth } from '@hooks/useAuth'
import { signIn } from '../../services/auth.services'
import { bridgeAssets } from '../../services/bridge.services'
import LoadingSpinner from '@components/LoadingSpinner'
import type { SponsorshipStatus, UserTransaction } from '@hooks/useCanisterStore'

const BridgePage = () => {
  const [amount, setAmount] = useState('')
  const [destinationAddress, setDestinationAddress] = useState('')
  const [destinationChain] = useState('Base Sepolia')
  const [isProcessing, setIsProcessing] = useState(false)
  const [bridgeCompleted, setBridgeCompleted] = useState(false)
  const [completedSettlement, setCompletedSettlement] = useState<any>(null)
  
  // New ICP payment state
  const [sponsorshipStatus, setSponsorshipStatus] = useState<SponsorshipStatus | null>(null)
  const [userTransactions, setUserTransactions] = useState<UserTransaction[]>([])
  const [showTransactionHistory, setShowTransactionHistory] = useState(false)
  
  const { 
    isLoading, 
    error, 
    reserveStatus, 
    createIcpPayment, 
    getSponsorshipStatus, 
    getUserTransactions 
  } = useCanisterStore()
  const { isAuthenticated, principal } = useAuth()

  // Check sponsorship status when amount changes
  useEffect(() => {
    if (amount && destinationAddress) {
      const amountWei = BigInt(Math.floor(parseFloat(amount) * 1e18))
      getSponsorshipStatus(amountWei, destinationChain).then(setSponsorshipStatus)
    }
  }, [amount, destinationAddress, destinationChain, getSponsorshipStatus])

  // Load user transactions on mount
  useEffect(() => {
    if (isAuthenticated) {
      getUserTransactions().then(setUserTransactions)
    }
  }, [isAuthenticated, getUserTransactions])

  const handleIcpPayment = async () => {
    if (!amount || !destinationAddress) return
    
    setIsProcessing(true)
    
    try {
      const amountWei = BigInt(Math.floor(parseFloat(amount) * 1e18))
      
      // AUTOMATIC ICP PAYMENT - No manual steps needed!
      const transaction = await createIcpPayment(amountWei, destinationAddress, destinationChain)
      if (transaction) {
        setCompletedSettlement(transaction)
        setBridgeCompleted(true)
        setUserTransactions(prev => [...prev, transaction])
        console.log('Automatic ICP payment completed:', transaction)
      }
    } catch (err) {
      console.error('Automatic ICP payment error:', err)
    } finally {
      setIsProcessing(false)
    }
  }

  const handleBridge = async () => {
    if (!amount || !destinationAddress) return
    
    setIsProcessing(true)
    setBridgeCompleted(false)
    setCompletedSettlement(null)
    
    try {
      const amountWei = BigInt(Math.floor(parseFloat(amount) * 1e18))
      
      // Use OISY-style automatic settlement
      const result = await bridgeAssets({
        amount: amountWei,
        destinationAddress,
        destinationChain,
        progress: (step) => {
          console.log('Bridge progress:', step)
        },
        setFailedProgressStep: (step) => {
          console.log('Bridge failed at step:', step)
        }
      })
      
      if (result.success && result.settlement) {
        setCompletedSettlement(result.settlement)
        setBridgeCompleted(true)
        console.log('Bridge completed successfully:', result.settlement)
      } else {
        console.error('Bridge failed:', result.error)
      }
    } catch (err) {
      console.error('Bridge error:', err)
    } finally {
      setIsProcessing(false)
    }
  }

  // Authentication gate
  if (!isAuthenticated) {
    return (
      <div className="min-h-screen pt-20">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-center"
          >
            <h1 className="text-4xl font-bold mb-4">
              Cross-Chain <span className="gradient-text">Bridge</span>
            </h1>
            <p className="text-xl text-dark-300 mb-8">
              Please connect your Internet Identity to use the bridge
            </p>
            <motion.button
              onClick={() => signIn({ domain: 'ic0.app' })}
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="btn-primary text-lg px-8 py-4"
            >
              Connect Internet Identity
            </motion.button>
          </motion.div>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen pt-20">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <h1 className="text-4xl font-bold mb-4">
            Cross-Chain <span className="gradient-text">Bridge</span>
          </h1>
          <p className="text-xl text-dark-300">
            Transfer assets between ICP and Ethereum ecosystems with zero gas fees
          </p>
          {principal && (
            <p className="text-sm text-dark-400 mt-2">
              Connected as: {principal.slice(0, 8)}...{principal.slice(-8)}
            </p>
          )}
        </motion.div>

        {/* Bridge Interface */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="card max-w-2xl mx-auto"
        >
          <div className="space-y-6">
            {/* From Section */}
            <div className="space-y-3">
              <label className="block text-sm font-medium text-dark-300">
                From (ICP - Your Internet Identity)
              </label>
              <div className="bg-dark-700 rounded-xl p-4 border border-dark-600">
                <div className="flex items-center justify-between mb-3">
                  <span className="text-lg font-semibold">ICP</span>
                  <div className="flex items-center space-x-2">
                    <Wallet className="w-4 h-4 text-green-400" />
                    <span className="text-sm text-green-400">
                      {principal ? `Connected: ${principal.slice(0, 8)}...${principal.slice(-8)}` : 'Not Connected'}
                    </span>
                  </div>
                </div>
                <input
                  type="number"
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                  placeholder="0.0"
                  className="w-full bg-transparent text-2xl font-bold text-white placeholder-dark-400 focus:outline-none"
                />
              </div>
            </div>

            {/* Bridge Icon */}
            <div className="flex justify-center">
              <motion.button
                whileHover={{ rotate: 180 }}
                className="w-12 h-12 bg-primary-600 rounded-full flex items-center justify-center text-white hover:bg-primary-700 transition-colors"
              >
                <ArrowRightLeft className="w-6 h-6" />
              </motion.button>
            </div>

            {/* To Section */}
            <div className="space-y-3">
              <label className="block text-sm font-medium text-dark-300">
                To (Ethereum - Your Wallet Address)
              </label>
              <div className="bg-dark-700 rounded-xl p-4 border border-dark-600">
                <div className="flex items-center justify-between mb-3">
                  <span className="text-lg font-semibold">{destinationChain}</span>
                  <span className="text-sm text-dark-400">Ethereum L2</span>
                </div>
                <input
                  type="text"
                  value={destinationAddress}
                  onChange={(e) => setDestinationAddress(e.target.value)}
                  placeholder="0x... destination address"
                  className="w-full bg-transparent text-white placeholder-dark-400 focus:outline-none"
                />
              </div>
            </div>

            {/* Sponsorship Status */}
            {sponsorshipStatus && (
              <motion.div
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                className={`rounded-xl p-4 space-y-3 ${
                  sponsorshipStatus.can_sponsor 
                    ? 'bg-green-500/10 border border-green-500/20' 
                    : 'bg-red-500/10 border border-red-500/20'
                }`}
              >
                <div className="flex items-center space-x-2">
                  <Info className="w-4 h-4 text-blue-400" />
                  <h3 className="font-semibold text-white">Sponsorship Status</h3>
                </div>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-dark-300">Can sponsor:</span>
                    <span className={`font-medium flex items-center ${
                      sponsorshipStatus.can_sponsor ? 'text-green-400' : 'text-red-400'
                    }`}>
                      <Check className="w-4 h-4 mr-1" />
                      {sponsorshipStatus.can_sponsor ? 'YES' : 'NO'}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-dark-300">Gas coverage:</span>
                    <span className={`font-medium ${
                      sponsorshipStatus.gas_coverage === 'Covered' ? 'text-green-400' : 'text-yellow-400'
                    }`}>
                      {sponsorshipStatus.gas_coverage}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-dark-300">Reserve health:</span>
                    <span className={`font-medium ${
                      sponsorshipStatus.reserve_health === 'Healthy' ? 'text-green-400' :
                      sponsorshipStatus.reserve_health === 'Warning' ? 'text-yellow-400' : 'text-red-400'
                    }`}>
                      {sponsorshipStatus.reserve_health}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-dark-300">ICP cost:</span>
                    <span className="text-white font-medium">
                      {(Number(sponsorshipStatus.estimated_cost_icp) / 1e8).toFixed(4)} ICP
                    </span>
                  </div>
                </div>
              </motion.div>
            )}

            {/* Bridge Details */}
            {amount && destinationAddress && (
              <motion.div
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                className="bg-dark-700/50 rounded-xl p-4 space-y-3"
              >
                <h3 className="font-semibold text-white">Bridge Details</h3>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-dark-300">You will receive:</span>
                    <span className="text-white font-medium">{amount} ETH</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-dark-300">Gas fees:</span>
                    <span className="text-green-400 font-medium flex items-center">
                      <Check className="w-4 h-4 mr-1" />
                      FREE
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-dark-300">Estimated time:</span>
                    <span className="text-white">~30 seconds</span>
                  </div>
                </div>
              </motion.div>
            )}

            {/* Error Display */}
            {error && (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="bg-red-500/10 border border-red-500/20 rounded-xl p-4 flex items-center space-x-3"
              >
                <AlertCircle className="w-5 h-5 text-red-400 flex-shrink-0" />
                <span className="text-red-400">{error}</span>
              </motion.div>
            )}

            {/* Bridge Buttons */}
            <div className="space-y-3">
              <button
                onClick={handleBridge}
                disabled={!amount || !destinationAddress || isLoading || isProcessing}
                className="w-full btn-primary py-4 text-lg font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading || isProcessing ? (
                  <div className="flex items-center justify-center space-x-2">
                    <LoadingSpinner size="sm" />
                    <span>Bridging Assets...</span>
                  </div>
                ) : (
                  'Bridge Assets (Free)'
                )}
              </button>
              
              <button
                onClick={handleIcpPayment}
                disabled={!amount || !destinationAddress || isLoading || isProcessing || !sponsorshipStatus?.can_sponsor}
                className="w-full bg-blue-600 hover:bg-blue-700 text-white py-4 text-lg font-semibold rounded-xl transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center space-x-2"
              >
                <CreditCard className="w-5 h-5" />
                <span>Pay with ICP</span>
              </button>
            </div>

            {/* Automatic ICP Payment Success Message */}
            {bridgeCompleted && completedSettlement && 'Completed' in completedSettlement.status && (
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                className="bg-green-500/10 border border-green-500/20 rounded-xl p-4"
              >
                <div className="flex items-center space-x-2 mb-3">
                  <CreditCard className="w-5 h-5 text-green-400" />
                  <h3 className="font-semibold text-white">Automatic ICP Payment Successful!</h3>
                </div>
                <p className="text-sm text-green-400">
                  Your ICP payment was processed automatically. No manual steps required!
                </p>
              </motion.div>
            )}

            {/* Transaction History Button */}
            <button
              onClick={() => setShowTransactionHistory(!showTransactionHistory)}
              className="w-full bg-dark-600 hover:bg-dark-500 text-white py-3 px-4 rounded-lg transition-colors flex items-center justify-center space-x-2"
            >
              <History className="w-4 h-4" />
              <span>View Transaction History ({userTransactions.length})</span>
            </button>

            {/* Transaction History */}
            {showTransactionHistory && (
              <motion.div
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                className="bg-dark-700/50 rounded-xl p-4 space-y-3"
              >
                <h3 className="font-semibold text-white flex items-center space-x-2">
                  <History className="w-4 h-4" />
                  <span>Transaction History</span>
                </h3>
                
                {userTransactions.length === 0 ? (
                  <p className="text-dark-300 text-sm">No transactions yet</p>
                ) : (
                  <div className="space-y-2">
                    {userTransactions.map((tx) => (
                      <div key={tx.id} className="bg-dark-600 rounded-lg p-3 space-y-2">
                        <div className="flex justify-between items-start">
                          <div className="space-y-1">
                            <p className="text-white font-medium text-sm">
                              {tx.destination_chain}
                            </p>
                            <p className="text-dark-300 text-xs">
                              {new Date(Number(tx.created_at) * 1000).toLocaleString()}
                            </p>
                          </div>
                          <div className="text-right space-y-1">
                            <p className="text-white font-medium text-sm">
                              {(Number(tx.amount_eth) / 1e18).toFixed(4)} ETH
                            </p>
                            <p className={`text-xs font-medium ${
                              'Completed' in tx.status ? 'text-green-400' :
                              'Failed' in tx.status ? 'text-red-400' :
                              'Processing' in tx.status ? 'text-yellow-400' : 'text-blue-400'
                            }`}>
                              {'Completed' in tx.status ? 'Completed' :
                               'Failed' in tx.status ? 'Failed' :
                               'Processing' in tx.status ? 'Processing' : 'Pending'}
                            </p>
                          </div>
                        </div>
                        {tx.transaction_hash && (
                          <p className="text-dark-300 text-xs font-mono">
                            TX: {tx.transaction_hash.slice(0, 10)}...{tx.transaction_hash.slice(-8)}
                          </p>
                        )}
                      </div>
                    ))}
                  </div>
                )}
              </motion.div>
            )}

            {/* Success Message */}
            {bridgeCompleted && completedSettlement && (
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                className="bg-green-500/10 border border-green-500/20 rounded-xl p-4"
              >
                <div className="flex items-center space-x-3 mb-3">
                  <Check className="w-5 h-5 text-green-400 flex-shrink-0" />
                  <span className="text-green-400 font-medium">Assets Bridged Successfully!</span>
                </div>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-dark-300">Settlement ID:</span>
                    <span className="text-white font-mono">{completedSettlement.id?.slice(0, 8)}...{completedSettlement.id?.slice(-8)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-dark-300">Amount:</span>
                    <span className="text-white">{amount} ETH</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-dark-300">Destination:</span>
                    <span className="text-white font-mono">{destinationAddress.slice(0, 6)}...{destinationAddress.slice(-4)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-dark-300">Status:</span>
                    <span className="text-green-400">Completed</span>
                  </div>
                  {completedSettlement.transaction_hash && (
                    <div className="flex justify-between">
                      <span className="text-dark-300">Transaction:</span>
                      <span className="text-white font-mono">{completedSettlement.transaction_hash.slice(0, 6)}...{completedSettlement.transaction_hash.slice(-4)}</span>
                    </div>
                  )}
                </div>
                <div className="mt-4 p-3 bg-dark-700 rounded-lg">
                  <p className="text-xs text-dark-300">
                    Your assets have been successfully bridged! You should receive {amount} ETH on {destinationChain} 
                    at the specified address within a few minutes.
                  </p>
                </div>
                <button
                  onClick={() => {
                    setBridgeCompleted(false)
                    setCompletedSettlement(null)
                    setAmount('')
                    setDestinationAddress('')
                  }}
                  className="mt-3 w-full bg-dark-600 hover:bg-dark-500 text-white py-2 px-4 rounded-lg transition-colors text-sm"
                >
                  Bridge More Assets
                </button>
              </motion.div>
            )}
          </div>
        </motion.div>

        {/* Reserve Status */}
        {reserveStatus && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4 }}
            className="mt-8 max-w-2xl mx-auto"
          >
            <div className="card bg-dark-800/50">
              <h3 className="font-semibold mb-4 text-white flex items-center">
                <span className={`w-2 h-2 rounded-full mr-3 ${
                  reserveStatus.health_status === 'Healthy' ? 'bg-green-400' :
                  reserveStatus.health_status === 'Warning' ? 'bg-yellow-400' : 'bg-red-400'
                }`} />
                Bridge Status: {reserveStatus.health_status}
              </h3>
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <span className="text-dark-300">Available Capacity:</span>
                  <p className="text-white font-medium">
                    {(Number(reserveStatus.available) / 1e18).toFixed(2)} ETH
                  </p>
                </div>
                <div>
                  <span className="text-dark-300">Utilization:</span>
                  <p className="text-white font-medium">
                    {reserveStatus.utilization_percent.toFixed(1)}%
                  </p>
                </div>
              </div>
            </div>
          </motion.div>
        )}
      </div>
    </div>
  )
}

export default BridgePage
