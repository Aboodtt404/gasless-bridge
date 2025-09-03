import React from 'react'
import { motion } from 'framer-motion'
import { Loader2, Zap } from 'lucide-react'

interface LoadingSpinnerProps {
  size?: 'sm' | 'md' | 'lg' | 'xl'
  text?: string
  fullscreen?: boolean
  variant?: 'default' | 'bridge'
}

const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({
  size = 'md',
  text,
  fullscreen = false,
  variant = 'default'
}) => {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-6 h-6',
    lg: 'w-8 h-8',
    xl: 'w-12 h-12'
  }

  const textSizeClasses = {
    sm: 'text-sm',
    md: 'text-base',
    lg: 'text-lg',
    xl: 'text-xl'
  }

  const containerClasses = fullscreen
    ? 'fixed inset-0 bg-dark-900/50 backdrop-blur-sm flex items-center justify-center z-50'
    : 'flex items-center justify-center p-8'

  const Spinner = () => {
    if (variant === 'bridge') {
      return (
        <motion.div className="relative">
          {/* Outer ring */}
          <motion.div
            className={`${sizeClasses[size]} border-2 border-primary-600/20 rounded-full`}
            animate={{ rotate: 360 }}
            transition={{ duration: 2, repeat: Infinity, ease: "linear" }}
          />
          
          {/* Inner spinning element */}
          <motion.div
            className={`absolute inset-0 ${sizeClasses[size]} border-2 border-transparent border-t-primary-500 rounded-full`}
            animate={{ rotate: 360 }}
            transition={{ duration: 1, repeat: Infinity, ease: "linear" }}
          />
          
          {/* Center icon */}
          <div className="absolute inset-0 flex items-center justify-center">
            <Zap className={`${
              size === 'sm' ? 'w-2 h-2' :
              size === 'md' ? 'w-3 h-3' :
              size === 'lg' ? 'w-4 h-4' : 'w-6 h-6'
            } text-primary-400`} />
          </div>
        </motion.div>
      )
    }

    return (
      <Loader2 className={`${sizeClasses[size]} text-primary-500 animate-spin`} />
    )
  }

  return (
    <div className={containerClasses}>
      <motion.div
        initial={{ opacity: 0, scale: 0.8 }}
        animate={{ opacity: 1, scale: 1 }}
        className="flex flex-col items-center space-y-4"
      >
        <Spinner />
        
        {text && (
          <motion.p
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.2 }}
            className={`${textSizeClasses[size]} text-dark-300 text-center max-w-xs`}
          >
            {text}
          </motion.p>
        )}
      </motion.div>
    </div>
  )
}

// Preset loading states for common use cases
export const BridgeLoadingSpinner = (props: Omit<LoadingSpinnerProps, 'variant'>) => (
  <LoadingSpinner {...props} variant="bridge" />
)

export const FullscreenLoader = ({ text }: { text?: string }) => (
  <LoadingSpinner
    size="xl"
    text={text || "Connecting to HyperBridge..."}
    fullscreen
    variant="bridge"
  />
)

export const InlineLoader = ({ text }: { text?: string }) => (
  <LoadingSpinner
    size="sm"
    text={text}
    variant="default"
  />
)

export default LoadingSpinner
