
import { motion } from 'framer-motion'
import { Link } from 'react-router-dom'
import { 
  ArrowRight, 
  Zap, 
  Shield, 
  Layers, 
  TrendingUp,
  CheckCircle,
  ArrowRightLeft,
  Coins,
  Clock,
  Users
} from 'lucide-react'

import { useCanisterStore } from '@hooks/useCanisterStore'

const HomePage = () => {
  const { reserveStatus, isConnected } = useCanisterStore()

  const features = [
    {
      icon: Zap,
      title: 'Zero Gas Fees',
      description: 'Bridge assets without paying any gas fees. HyperBridge covers all transaction costs.',
      gradient: 'from-yellow-400 to-orange-500'
    },
    {
      icon: Shield,
      title: 'Secure & Trustless',
      description: 'Powered by ICP Threshold ECDSA and Chain Fusion technology for maximum security.',
      gradient: 'from-green-400 to-blue-500'
    },
    {
      icon: Layers,
      title: 'Multi-Chain Support',
      description: 'Seamlessly bridge between ICP and Ethereum ecosystems with more chains coming.',
      gradient: 'from-purple-400 to-pink-500'
    },
    {
      icon: TrendingUp,
      title: 'Optimized Performance',
      description: 'Advanced RPC caching and gas estimation for lightning-fast transactions.',
      gradient: 'from-blue-400 to-indigo-500'
    }
  ]

  const stats = [
    {
      label: 'Total Volume',
      value: '$2.5M+',
      icon: Coins,
      change: '+24.5%'
    },
    {
      label: 'Transactions',
      value: '12,847',
      icon: ArrowRightLeft,
      change: '+18.2%'
    },
    {
      label: 'Active Users',
      value: '3,421',
      icon: Users,
      change: '+31.7%'
    },
    {
      label: 'Avg Response Time',
      value: '0.8s',
      icon: Clock,
      change: '-45.3%'
    }
  ]

  const benefits = [
    'No gas fees ever',
    'Instant settlement',
    'Cross-chain compatibility',
    'Enterprise-grade security',
    'Open source & audited',
    'Community governed'
  ]

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="relative pt-20 pb-32 overflow-hidden">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center">
            <motion.div
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6 }}
              className="space-y-8"
            >
              <div className="space-y-4">
                <motion.div
                  initial={{ opacity: 0, scale: 0.8 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.5, delay: 0.2 }}
                  className="inline-flex items-center px-4 py-2 bg-primary-500/10 border border-primary-500/20 rounded-full text-primary-400 text-sm font-medium"
                >
                  <Zap className="w-4 h-4 mr-2" />
                  Revolutionary Gasless Technology
                </motion.div>

                <h1 className="text-5xl md:text-6xl lg:text-7xl font-bold">
                  <span className="block">Bridge Assets</span>
                  <span className="block gradient-text">Without Gas Fees</span>
                </h1>

                <p className="text-xl text-dark-300 max-w-3xl mx-auto leading-relaxed">
                  The first truly gasless cross-chain bridge powered by ICP Chain Fusion. 
                  Move your assets between ICP and Ethereum ecosystems with zero transaction costs.
                </p>
              </div>

              <div className="flex flex-col sm:flex-row items-center justify-center space-y-4 sm:space-y-0 sm:space-x-6">
                <Link to="/bridge">
                  <motion.button
                    whileHover={{ scale: 1.05 }}
                    whileTap={{ scale: 0.95 }}
                    className="btn-primary text-lg px-8 py-4 flex items-center space-x-2"
                  >
                    <span>Start Bridging</span>
                    <ArrowRight className="w-5 h-5" />
                  </motion.button>
                </Link>

                <Link to="/docs">
                  <motion.button
                    whileHover={{ scale: 1.05 }}
                    whileTap={{ scale: 0.95 }}
                    className="btn-secondary text-lg px-8 py-4"
                  >
                    Learn More
                  </motion.button>
                </Link>
              </div>

              {/* Connection Status */}
              {isConnected && (
                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: 0.8 }}
                  className="inline-flex items-center px-4 py-2 bg-green-500/10 border border-green-500/20 rounded-full text-green-400 text-sm"
                >
                  <CheckCircle className="w-4 h-4 mr-2" />
                  Connected to HyperBridge Network
                </motion.div>
              )}
            </motion.div>
          </div>
        </div>

        {/* Floating elements */}
        <div className="absolute inset-0 pointer-events-none">
          <motion.div
            animate={{ 
              y: [0, -20, 0],
              rotate: [0, 5, 0] 
            }}
            transition={{ 
              duration: 6, 
              repeat: Infinity, 
              ease: "easeInOut" 
            }}
            className="absolute top-1/4 left-1/4 w-20 h-20 bg-primary-500/10 rounded-full blur-xl"
          />
          <motion.div
            animate={{ 
              y: [0, 30, 0],
              rotate: [0, -5, 0] 
            }}
            transition={{ 
              duration: 8, 
              repeat: Infinity, 
              ease: "easeInOut",
              delay: 2 
            }}
            className="absolute top-1/3 right-1/4 w-32 h-32 bg-secondary-500/10 rounded-full blur-xl"
          />
        </div>
      </section>

      {/* Stats Section */}
      <section className="py-20 border-y border-dark-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid grid-cols-2 lg:grid-cols-4 gap-8">
            {stats.map(({ label, value, icon: Icon, change }, index) => (
              <motion.div
                key={label}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: index * 0.1 }}
                className="text-center space-y-3"
              >
                <div className="w-12 h-12 bg-dark-800 rounded-xl flex items-center justify-center mx-auto">
                  <Icon className="w-6 h-6 text-primary-400" />
                </div>
                <div>
                  <h3 className="text-2xl font-bold text-white">{value}</h3>
                  <p className="text-dark-400 text-sm">{label}</p>
                  <span className={`text-xs ${
                    change.startsWith('+') ? 'text-green-400' : 'text-red-400'
                  }`}>
                    {change}
                  </span>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold mb-4">
              Why Choose <span className="gradient-text">HyperBridge</span>?
            </h2>
            <p className="text-xl text-dark-300 max-w-3xl mx-auto">
              Experience the future of cross-chain bridging with cutting-edge technology 
              and unparalleled user experience.
            </p>
          </motion.div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
            {features.map(({ icon: Icon, title, description, gradient }, index) => (
              <motion.div
                key={title}
                initial={{ opacity: 0, y: 30 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: index * 0.2 }}
                whileHover={{ y: -5 }}
                className="card group cursor-pointer"
              >
                <div className={`w-12 h-12 bg-gradient-to-r ${gradient} rounded-xl flex items-center justify-center mb-4 group-hover:scale-110 transition-transform`}>
                  <Icon className="w-6 h-6 text-white" />
                </div>
                <h3 className="text-xl font-semibold mb-3 text-white">{title}</h3>
                <p className="text-dark-300 leading-relaxed">{description}</p>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Benefits Section */}
      <section className="py-20 bg-dark-800/30">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid lg:grid-cols-2 gap-12 items-center">
            <motion.div
              initial={{ opacity: 0, x: -30 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              className="space-y-8"
            >
              <div>
                <h2 className="text-4xl font-bold mb-4">
                  Built for the <span className="gradient-text">Future</span>
                </h2>
                <p className="text-xl text-dark-300">
                  HyperBridge leverages the latest blockchain innovations to deliver 
                  a seamless cross-chain experience that scales with your needs.
                </p>
              </div>

              <div className="grid sm:grid-cols-2 gap-4">
                {benefits.map((benefit, index) => (
                  <motion.div
                    key={benefit}
                    initial={{ opacity: 0, x: -20 }}
                    whileInView={{ opacity: 1, x: 0 }}
                    viewport={{ once: true }}
                    transition={{ delay: index * 0.1 }}
                    className="flex items-center space-x-3"
                  >
                    <CheckCircle className="w-5 h-5 text-green-400 flex-shrink-0" />
                    <span className="text-dark-200">{benefit}</span>
                  </motion.div>
                ))}
              </div>

              {reserveStatus && (
                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  className="card bg-dark-700/50"
                >
                  <h3 className="font-semibold mb-3 text-white">Bridge Status</h3>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-dark-300">Reserve Health:</span>
                      <span className={
                        reserveStatus.health_status === 'Healthy' ? 'text-green-400' :
                        reserveStatus.health_status === 'Warning' ? 'text-yellow-400' : 'text-red-400'
                      }>
                        {reserveStatus.health_status}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-dark-300">Available Capacity:</span>
                      <span className="text-white">
                        {Number(reserveStatus.available) / 1e18} ETH
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-dark-300">Utilization:</span>
                      <span className="text-white">{reserveStatus.utilization_percent.toFixed(1)}%</span>
                    </div>
                  </div>
                </motion.div>
              )}
            </motion.div>

            <motion.div
              initial={{ opacity: 0, x: 30 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              className="relative"
            >
              {/* Placeholder for 3D visualization */}
              <div className="aspect-square bg-gradient-to-br from-primary-600/20 to-secondary-600/20 rounded-2xl flex items-center justify-center">
                <div className="text-center">
                  <Zap className="w-16 h-16 text-primary-400 mx-auto mb-4" />
                  <p className="text-dark-300">3D Bridge Visualization</p>
                  <p className="text-sm text-dark-400">Coming Soon</p>
                </div>
              </div>
            </motion.div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <motion.div
            initial={{ opacity: 0, y: 30 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="space-y-8"
          >
            <h2 className="text-4xl font-bold">
              Ready to Bridge <span className="gradient-text">Without Limits</span>?
            </h2>
            <p className="text-xl text-dark-300 max-w-2xl mx-auto">
              Join thousands of users who have already discovered the power of gasless bridging.
            </p>
            <Link to="/bridge">
              <motion.button
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                className="btn-primary text-lg px-12 py-4 flex items-center space-x-2 mx-auto"
              >
                <span>Start Your First Bridge</span>
                <ArrowRight className="w-5 h-5" />
              </motion.button>
            </Link>
          </motion.div>
        </div>
      </section>
    </div>
  )
}

export default HomePage
