use candid::{CandidType, Deserialize};
use sha3::{Digest, Keccak256};
use rlp::RlpStream;
use crate::services::threshold_ecdsa::{EthereumAddress, TransactionHash};
use crate::services::gas_estimator::GasEstimate;
use libsecp256k1::{Signature, RecoveryId};

/// EIP-1559 Ethereum transaction structure for Base Sepolia
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EthereumTransaction {
    /// Transaction nonce (number of transactions sent from this address)
    pub nonce: u64,
    /// Maximum fee per gas (EIP-1559)
    pub max_fee_per_gas: u64,
    /// Maximum priority fee per gas (EIP-1559 tip)
    pub max_priority_fee_per_gas: u64,
    /// Gas limit for the transaction
    pub gas_limit: u64,
    /// Recipient address
    pub to: EthereumAddress,
    /// Value to transfer (in wei)
    pub value: u64,
    /// Transaction data (empty for simple transfers)
    pub data: Vec<u8>,
    /// Chain ID (Base Sepolia = 84532)
    pub chain_id: u64,
}

/// Signed Ethereum transaction ready for broadcasting
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct SignedTransaction {
    pub raw_transaction: Vec<u8>,
    pub transaction_hash: TransactionHash,
    pub from_address: EthereumAddress,
    pub to_address: EthereumAddress,
    pub value: u64,
    pub gas_limit: u64,
    pub max_fee_per_gas: u64,
}

impl EthereumTransaction {
    /// Create a new ETH transfer transaction for Base Sepolia
    pub fn new_transfer(
        to: EthereumAddress,
        value: u64,
        nonce: u64,
        gas_estimate: &GasEstimate,
    ) -> Self {
        Self {
            nonce,
            max_fee_per_gas: gas_estimate.max_fee_per_gas,
            max_priority_fee_per_gas: gas_estimate.priority_fee,
            gas_limit: gas_estimate.gas_limit,
            to,
            value,
            data: vec![], // Empty for simple transfers
            chain_id: 84532, // Base Sepolia chain ID
        }
    }

    /// Create transaction for gasless bridge delivery
    /// This is the core function that creates the actual ETH delivery transaction!
    pub fn new_bridge_delivery(
        recipient: EthereumAddress,
        amount: u64,
        nonce: u64,
        gas_estimate: &GasEstimate,
    ) -> Self {
        ic_cdk::println!(
            "🚀 Creating bridge delivery transaction: {} wei to {}",
            amount,
            recipient
        );
        
        Self::new_transfer(recipient, amount, nonce, gas_estimate)
    }

    /// Get the transaction hash for signing (EIP-1559 format)
    /// This hash is what gets signed by threshold ECDSA
    pub fn get_signing_hash(&self) -> TransactionHash {
        // EIP-1559 transaction type (0x02)
        let mut rlp_stream = RlpStream::new();
        rlp_stream.begin_list(9);
        
        // Add transaction fields in EIP-1559 order
        rlp_stream.append(&self.chain_id);
        rlp_stream.append(&self.nonce);
        rlp_stream.append(&self.max_priority_fee_per_gas);
        rlp_stream.append(&self.max_fee_per_gas);
        rlp_stream.append(&self.gas_limit);
        rlp_stream.append(&self.to.0.as_slice());
        rlp_stream.append(&self.value);
        rlp_stream.append(&self.data);
        rlp_stream.append_empty_data(); // access_list (empty)

        let encoded = rlp_stream.out();
        
        let mut tx_bytes = vec![0x02];
        tx_bytes.extend_from_slice(&encoded);
        
        let mut hasher = Keccak256::new();
        hasher.update(&tx_bytes);
        let hash = hasher.finalize();
        
        TransactionHash(hash.into())
    }

    pub fn to_signed_transaction(
        &self,
        signature: &Signature,
        recovery_id: &RecoveryId,
        from_address: EthereumAddress,
    ) -> Result<SignedTransaction, String> {
        ic_cdk::println!("✍️ Creating signed transaction with threshold ECDSA signature");
        
        // Calculate v value for EIP-1559 (recovery_id + chain_id * 2 + 35)
        let v = recovery_id.serialize() as u64;
        
        // Extract r and s from signature
        let sig_bytes = signature.serialize();
        let r_bytes = &sig_bytes[0..32];
        let s_bytes = &sig_bytes[32..64];
        
        // Convert to big-endian u64 arrays for RLP encoding
        let r = u64::from_be_bytes([
            r_bytes[24], r_bytes[25], r_bytes[26], r_bytes[27],
            r_bytes[28], r_bytes[29], r_bytes[30], r_bytes[31]
        ]);
        let s = u64::from_be_bytes([
            s_bytes[24], s_bytes[25], s_bytes[26], s_bytes[27],
            s_bytes[28], s_bytes[29], s_bytes[30], s_bytes[31]
        ]);

        // Create signed transaction RLP
        let mut rlp_stream = RlpStream::new();
        rlp_stream.begin_list(12); // 9 transaction fields + 3 signature fields
        
        rlp_stream.append(&self.chain_id);
        rlp_stream.append(&self.nonce);
        rlp_stream.append(&self.max_priority_fee_per_gas);
        rlp_stream.append(&self.max_fee_per_gas);
        rlp_stream.append(&self.gas_limit);
        rlp_stream.append(&self.to.0.as_slice());
        rlp_stream.append(&self.value);
        rlp_stream.append(&self.data);
        rlp_stream.append_empty_data(); // access_list (empty)
        rlp_stream.append(&v);
        rlp_stream.append(&r);
        rlp_stream.append(&s);

        let encoded = rlp_stream.out();
        
        // Final transaction: 0x02 || rlp_encoded_signed_tx
        let mut raw_transaction = vec![0x02];
        raw_transaction.extend_from_slice(&encoded);
        
        // Calculate transaction hash for tracking
        let mut hasher = Keccak256::new();
        hasher.update(&raw_transaction);
        let transaction_hash = TransactionHash(hasher.finalize().into());
        
        ic_cdk::println!(
            "✅ Signed transaction created! Hash: {}, Size: {} bytes",
            transaction_hash,
            raw_transaction.len()
        );

        Ok(SignedTransaction {
            raw_transaction,
            transaction_hash,
            from_address,
            to_address: self.to.clone(),
            value: self.value,
            gas_limit: self.gas_limit,
            max_fee_per_gas: self.max_fee_per_gas,
        })
    }

    /// Calculate total transaction cost (value + gas fees)
    pub fn calculate_total_cost(&self) -> u64 {
        let max_gas_cost = self.gas_limit * self.max_fee_per_gas;
        self.value + max_gas_cost
    }

    /// Validate transaction parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.to.0 == [0u8; 20] {
            return Err("Invalid recipient address (zero address)".to_string());
        }
        
        if self.value == 0 {
            return Err("Transaction value cannot be zero".to_string());
        }
        
        if self.gas_limit < 21000 {
            return Err("Gas limit too low (minimum 21000 for transfers)".to_string());
        }
        
        if self.max_fee_per_gas < self.max_priority_fee_per_gas {
            return Err("Max fee per gas must be >= max priority fee per gas".to_string());
        }
        
        if self.chain_id != 84532 {
            return Err("Invalid chain ID (expected 84532 for Base Sepolia)".to_string());
        }
        
        Ok(())
    }

    /// Create a test transaction for verification
    pub fn create_test_transaction(nonce: u64) -> Self {
        let test_recipient = EthereumAddress([
            0x74, 0x2d, 0x35, 0xBc, 0xBb, 0x06, 0xAa, 0x0B, 0x89, 0xf1,
            0x14, 0xEf, 0xc1, 0xaA, 0xd7, 0xBe, 0x20, 0x98, 0x6a, 0x4b
        ]); // Test address
        
        let test_gas_estimate = GasEstimate {
            gas_limit: 21000,
            max_fee_per_gas: 1_000_000_000, // 1 Gwei
            priority_fee: 100_000_000, // 0.1 Gwei
            base_fee: 900_000_000, // 0.9 Gwei
            total_cost: 21_000_000_000_000, // 21000 * 1 Gwei
            safety_margin: 25, // 25% safety margin
        };
        
        Self::new_bridge_delivery(
            test_recipient,
            1_000_000_000_000_000_000, // 1 ETH
            nonce,
            &test_gas_estimate,
        )
    }
}

impl SignedTransaction {
    /// Get the raw transaction as hex string for broadcasting
    pub fn to_hex_string(&self) -> String {
        format!("0x{}", hex::encode(&self.raw_transaction))
    }
    
    /// Get transaction info for logging/debugging
    pub fn get_info(&self) -> String {
        format!(
            "📋 Signed Transaction Info:\n\
            🔗 Hash: {}\n\
            📤 From: {}\n\
            📥 To: {}\n\
            💰 Value: {:.6} ETH\n\
            ⛽ Gas Limit: {}\n\
            💸 Max Fee: {} Gwei\n\
            📦 Size: {} bytes",
            self.transaction_hash,
            self.from_address,
            self.to_address,
            self.value as f64 / 1e18,
            self.gas_limit,
            self.max_fee_per_gas / 1_000_000_000,
            self.raw_transaction.len()
        )
    }
}

/// Ethereum transaction builder service
pub struct EthTransactionBuilder;

impl EthTransactionBuilder {
    /// Build a complete bridge delivery transaction
    /// This integrates with threshold ECDSA to create signed transactions
    pub async fn build_bridge_delivery_transaction(
        recipient: EthereumAddress,
        amount: u64,
        nonce: u64,
        gas_estimate: GasEstimate,
        from_address: EthereumAddress,
    ) -> Result<SignedTransaction, String> {
        ic_cdk::println!(
            "🏗️ Building bridge delivery transaction: {} ETH to {}",
            amount as f64 / 1e18,
            recipient
        );
        
        // 1. Create the transaction
        let transaction = EthereumTransaction::new_bridge_delivery(
            recipient,
            amount,
            nonce,
            &gas_estimate,
        );
        
        // 2. Validate transaction
        transaction.validate()?;
        
        // 3. Get signing hash
        let signing_hash = transaction.get_signing_hash();
        
        // 4. Sign with threshold ECDSA
        let (signature, recovery_id) = crate::services::threshold_ecdsa::sign_ethereum_transaction_hash(signing_hash).await?;
        
        // 5. Create signed transaction
        let signed_tx = transaction.to_signed_transaction(&signature, &recovery_id, from_address.clone())?;
        
        ic_cdk::println!("✅ Bridge delivery transaction built successfully!");
        Ok(signed_tx)
    }
    
    /// Test the transaction building workflow
    pub async fn test_transaction_building() -> Result<String, String> {
        ic_cdk::println!("🧪 Testing Ethereum transaction building...");
        
        // Get our canister's Ethereum address
        let from_address = crate::services::threshold_ecdsa::get_canister_ethereum_address().await?;
        
        // Create test transaction
        let test_tx = EthereumTransaction::create_test_transaction(0);
        
        // Test validation
        test_tx.validate()?;
        
        // Test signing hash calculation
        let signing_hash = test_tx.get_signing_hash();
        
        // Test signing with threshold ECDSA
        let (signature, recovery_id) = crate::services::threshold_ecdsa::sign_ethereum_transaction_hash(signing_hash.clone()).await?;
        
        // Create signed transaction
        let signed_tx = test_tx.to_signed_transaction(&signature, &recovery_id, from_address.clone())?;
        
        let result = format!(
            "✅ Ethereum Transaction Building Test Successful!\n\
            \n\
            🏗️ Transaction Built:\n\
            📤 From: {}\n\
            📥 To: {}\n\
            💰 Value: {} ETH\n\
            ⛽ Gas Limit: {}\n\
            💸 Max Fee: {} Gwei\n\
            \n\
            ✍️ Signature Created:\n\
            🔐 Signing Hash: {}\n\
            📝 Signature: {}\n\
            🔄 Recovery ID: {}\n\
            \n\
            📦 Final Signed Transaction:\n\
            🔗 TX Hash: {}\n\
            📏 Size: {} bytes\n\
            📋 Ready for broadcast to Base Sepolia!",
            from_address,
            test_tx.to,
            test_tx.value as f64 / 1e18,
            test_tx.gas_limit,
            test_tx.max_fee_per_gas / 1_000_000_000,
            hex::encode(signing_hash.0),
            hex::encode(signature.serialize()),
            recovery_id.serialize(),
            signed_tx.transaction_hash,
            signed_tx.raw_transaction.len()
        );
        
        ic_cdk::println!("{}", result);
        Ok(result)
    }
}

/// Public API functions

/// Build a signed transaction for bridge delivery
pub async fn build_signed_bridge_transaction(
    recipient: EthereumAddress,
    amount: u64,
    nonce: u64,
    gas_estimate: GasEstimate,
) -> Result<SignedTransaction, String> {
    let from_address = crate::services::threshold_ecdsa::get_canister_ethereum_address().await?;
    EthTransactionBuilder::build_bridge_delivery_transaction(
        recipient,
        amount,
        nonce,
        gas_estimate,
        from_address,
    ).await
}

/// Test the transaction building workflow
pub async fn test_ethereum_transaction_building() -> Result<String, String> {
    EthTransactionBuilder::test_transaction_building().await
}

/// Complete bridge transaction execution
/// This is the holy grail - the complete ckETH → ETH flow!
pub async fn execute_bridge_transaction(
    recipient: EthereumAddress,
    amount: u64,
    gas_estimate: GasEstimate,
) -> Result<String, String> {
    ic_cdk::println!("🚀 Executing complete bridge transaction: {} wei to {}", amount, recipient);
    
    // 1. Get our canister's Ethereum address
    let from_address = crate::services::threshold_ecdsa::get_canister_ethereum_address().await?;
    ic_cdk::println!("📤 From address: {}", from_address);
    
    // 2. Get current nonce for our address
    let mut rpc_client = crate::services::rpc_client::RpcClient::new_base_sepolia();
    let nonce = rpc_client.get_nonce_cached(&from_address.to_string(), "base_sepolia").await
        .map_err(|e| format!("Failed to get nonce: {}", e.message))?;
    ic_cdk::println!("🔢 Current nonce: {}", nonce);
    
    // 3. Build the transaction
    let transaction = EthereumTransaction::new_bridge_delivery(recipient.clone(), amount, nonce, &gas_estimate);
    ic_cdk::println!("🏗️ Transaction built successfully");
    
    // 4. Validate transaction
    transaction.validate()?;
    ic_cdk::println!("✅ Transaction validation passed");
    
    // 5. Get signing hash
    let signing_hash = transaction.get_signing_hash();
    ic_cdk::println!("🔐 Signing hash: {}", hex::encode(signing_hash.0));
    
    // 6. Sign with threshold ECDSA
    let (signature, recovery_id) = crate::services::threshold_ecdsa::sign_ethereum_transaction_hash(signing_hash).await?;
    ic_cdk::println!("✍️ Transaction signed with recovery ID: {}", recovery_id.serialize());
    
    // 7. Create signed transaction
    let signed_tx = transaction.to_signed_transaction(&signature, &recovery_id, from_address.clone())?;
    ic_cdk::println!("📦 Signed transaction created: {}", signed_tx.transaction_hash);
    
    // 8. Convert to hex string for broadcasting
    let raw_tx_hex = format!("0x{}", hex::encode(&signed_tx.raw_transaction));
    ic_cdk::println!("📡 Raw transaction ({} bytes): {}", signed_tx.raw_transaction.len(), raw_tx_hex);
    
    // 9. Broadcast to Ethereum network
    let tx_hash = crate::services::rpc_client::broadcast_ethereum_transaction(&raw_tx_hex, "base_sepolia").await?;
    ic_cdk::println!("✅ Transaction broadcast successful! Hash: {}", tx_hash);
    
    let result = format!(
        "🎉 Bridge Transaction Executed Successfully!\n\
         \n\
         📤 From: {}\n\
         📥 To: {}\n\
         💰 Amount: {} wei\n\
         ⛽ Gas Limit: {}\n\
         💸 Max Fee: {} Gwei\n\
         \n\
         🔐 Signature Details:\n\
         ✍️ Recovery ID: {}\n\
         📝 Signature: {}\n\
         \n\
         📡 Network Details:\n\
         🔗 Transaction Hash: {}\n\
         🌐 Network: Base Sepolia\n\
         ✅ Status: Broadcast Successful!\n\
         \n\
         🎯 This completes the ckETH → ETH flow!",
        from_address,
        recipient,
        amount,
        gas_estimate.gas_limit,
        gas_estimate.max_fee_per_gas / 1_000_000_000,
        recovery_id.serialize(),
        hex::encode(signature.serialize()),
        tx_hash
    );
    
    ic_cdk::println!("{}", result);
    Ok(result)
}

/// Test the complete bridge transaction flow
pub async fn test_complete_bridge_flow() -> Result<String, String> {
    ic_cdk::println!("🧪 Testing complete bridge transaction flow...");
    
    // Create a test recipient address
    let test_recipient = EthereumAddress([0x42u8; 20]); // Test address
    
    // Create a test gas estimate
    let gas_estimate = GasEstimate {
        gas_limit: 21000,
        max_fee_per_gas: 20_000_000_000, // 20 Gwei
        priority_fee: 1_000_000_000,     // 1 Gwei
        total_cost: 420_000_000_000, // 21000 * 20 Gwei
        base_fee: 15_000_000_000,        // 15 Gwei base fee
        safety_margin: 5_000_000_000,    // 5 Gwei safety margin
    };
    
    // Test with a small amount (0.001 ETH)
    let test_amount = 1_000_000_000_000_000; // 0.001 ETH in wei
    
    execute_bridge_transaction(test_recipient, test_amount, gas_estimate).await
}
