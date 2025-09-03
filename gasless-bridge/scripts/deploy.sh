#!/bin/bash

echo "🚀 Deploying Gasless Bridge..."

# Build the project
echo "Building..."
dfx build

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

# Deploy to local network
echo "Deploying to local network..."
dfx deploy

if [ $? -ne 0 ]; then
    echo "❌ Deployment failed"
    exit 1
fi

echo "✅ Deployment successful!"

# Test basic functionality
echo "🧪 Testing basic functionality..."
dfx canister call gasless-bridge health_check
dfx canister call gasless-bridge test_base_rpc

echo "🎉 Deployment and testing complete!"
