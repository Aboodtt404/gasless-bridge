
import { motion } from 'framer-motion'
import { Github, Twitter, Globe, Heart, Zap } from 'lucide-react'

const Footer = () => {
  const currentYear = new Date().getFullYear()

  const socialLinks = [
    { icon: Github, href: '#', label: 'GitHub' },
    { icon: Twitter, href: '#', label: 'Twitter' },
    { icon: Globe, href: '#', label: 'Website' },
  ]

  const footerLinks = [
    {
      title: 'Product',
      links: [
        { label: 'Bridge', href: '/bridge' },
        { label: 'Stats', href: '/stats' },
        { label: 'Documentation', href: '/docs' },
      ]
    },
    {
      title: 'Technology',
      links: [
        { label: 'ICP Chain Fusion', href: '#' },
        { label: 'Threshold ECDSA', href: '#' },
        { label: 'Gasless Technology', href: '#' },
      ]
    },
    {
      title: 'Community',
      links: [
        { label: 'Discord', href: '#' },
        { label: 'Telegram', href: '#' },
        { label: 'Forum', href: '#' },
      ]
    }
  ]

  return (
    <footer className="relative mt-20 bg-dark-900 border-t border-dark-700">
      {/* Background gradient */}
      <div className="absolute inset-0 bg-gradient-to-t from-dark-900 via-dark-800/50 to-transparent" />
      
      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        {/* Main footer content */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-8">
          {/* Brand section */}
          <div className="lg:col-span-2">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="space-y-4"
            >
              <div className="flex items-center space-x-3">
                <div className="w-8 h-8 bg-gradient-to-r from-primary-500 to-secondary-500 rounded-lg flex items-center justify-center">
                  <Zap className="w-5 h-5 text-white" />
                </div>
                <div>
                  <h2 className="text-xl font-bold gradient-text">HyperBridge</h2>
                  <p className="text-xs text-dark-400">Revolutionary Gasless Bridge</p>
                </div>
              </div>
              
              <p className="text-dark-300 max-w-md">
                The first truly gasless cross-chain bridge powered by ICP Chain Fusion technology. 
                Bridge assets between ICP and Ethereum with zero gas fees.
              </p>
              
              <div className="flex space-x-4">
                {socialLinks.map(({ icon: Icon, href, label }) => (
                  <motion.a
                    key={label}
                    href={href}
                    whileHover={{ scale: 1.1 }}
                    whileTap={{ scale: 0.95 }}
                    className="w-10 h-10 bg-dark-800 hover:bg-dark-700 rounded-lg flex items-center justify-center text-dark-400 hover:text-white transition-colors"
                    aria-label={label}
                  >
                    <Icon className="w-5 h-5" />
                  </motion.a>
                ))}
              </div>
            </motion.div>
          </div>

          {/* Links sections */}
          {footerLinks.map(({ title, links }, index) => (
            <motion.div
              key={title}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ delay: index * 0.1 }}
              className="space-y-4"
            >
              <h3 className="text-white font-semibold">{title}</h3>
              <ul className="space-y-2">
                {links.map(({ label, href }) => (
                  <li key={label}>
                    <a
                      href={href}
                      className="text-dark-400 hover:text-white transition-colors text-sm"
                    >
                      {label}
                    </a>
                  </li>
                ))}
              </ul>
            </motion.div>
          ))}
        </div>

        {/* Bottom section */}
        <motion.div
          initial={{ opacity: 0 }}
          whileInView={{ opacity: 1 }}
          viewport={{ once: true }}
          transition={{ delay: 0.4 }}
          className="mt-12 pt-8 border-t border-dark-700 flex flex-col sm:flex-row items-center justify-between space-y-4 sm:space-y-0"
        >
          <div className="flex items-center space-x-2 text-sm text-dark-400">
            <span>© {currentYear} HyperBridge. Made with</span>
            <Heart className="w-4 h-4 text-red-400" />
            <span>on the Internet Computer</span>
          </div>
          
          <div className="flex items-center space-x-6 text-sm text-dark-400">
            <a href="#" className="hover:text-white transition-colors">Privacy Policy</a>
            <a href="#" className="hover:text-white transition-colors">Terms of Service</a>
            <a href="#" className="hover:text-white transition-colors">Security</a>
          </div>
        </motion.div>
      </div>
    </footer>
  )
}

export default Footer
