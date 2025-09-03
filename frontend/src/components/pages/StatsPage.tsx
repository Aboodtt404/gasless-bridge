
import { motion } from 'framer-motion'

const StatsPage = () => {
  return (
    <div className="min-h-screen pt-20">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center"
        >
          <h1 className="text-4xl font-bold mb-4">
            Bridge <span className="gradient-text">Statistics</span>
          </h1>
          <p className="text-xl text-dark-300 mb-12">
            Real-time analytics and performance metrics
          </p>
          
          <div className="card max-w-md mx-auto">
            <h2 className="text-2xl font-semibold mb-4">Coming Soon</h2>
            <p className="text-dark-300">
              Comprehensive statistics and analytics dashboard is under development.
            </p>
          </div>
        </motion.div>
      </div>
    </div>
  )
}

export default StatsPage
