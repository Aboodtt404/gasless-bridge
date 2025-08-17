#!/bin/bash

echo "🚀 Setting up Gasless Bridge development environment..."

# Create directories if they don't exist
mkdir -p {config,docs,scripts,tests}

# Set up environment variables
if [ ! -f .env ]; then
    cat > .env << EOF
# Development Environment Configuration
ENVIRONMENT=development

# Base Sepolia Configuration
ALCHEMY_BASE_SEPOLIA_URL=https://base-sepolia.g.alchemy.com/v2/keg5qPpXALLYHHhXJBKuL
QUICKNODE_BASE_SEPOLIA_URL=https://fluent-empty-meadow.base-sepolia.quiknode.pro/a9ebb15ae2a849c069efd78d24022e2e60c1be1f/
PUBLIC_BASE_SEPOLIA_URL=https://sepolia.base.org

# Test Wallet
TEST_WALLET_ADDRESS=0x48E464E6390713C3Ba8082D21275aCB24e24ff96

# Network Info
BASE_SEPOLIA_CHAIN_ID=84532

# Development Settings
LOG_LEVEL=debug
QUOTE_VALIDITY_MINUTES=10
GAS_SAFETY_MARGIN=20
EOF
    echo "✅ Created .env file"
fi

# Make scripts executable
chmod +x scripts/*.sh

echo "🎉 Environment setup complete!"
echo "Next steps:"
echo "1. Run 'dfx build' to compile with new dependencies"
echo "2. Run 'dfx deploy' to update the canister"
echo "3. Run 'scripts/test_rpcs.sh' to verify connectivity"
