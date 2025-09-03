#!/bin/bash

source .env

echo "🧪 Testing All Base Sepolia RPC Endpoints..."

test_rpc() {
    local name=$1
    local url=$2
    
    echo "Testing $name..."
    
    response=$(curl -s -X POST "$url" \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}')
    
    if echo "$response" | grep -q "0x14a34"; then
        echo "✅ $name: Connected to Base Sepolia (Chain ID: 84532)"
    else
        echo "❌ $name: Failed or wrong network"
        echo "   Response: $response"
    fi
    echo
}

# Test all endpoints
test_rpc "Alchemy (Primary)" "$ALCHEMY_BASE_SEPOLIA_URL"
test_rpc "QuickNode (Fallback)" "$QUICKNODE_BASE_SEPOLIA_URL" 
test_rpc "Public RPC" "$PUBLIC_BASE_SEPOLIA_URL"

echo "🏁 RPC testing complete!"
