use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
    SignWithEcdsaArgument,
};
use sha3::{Digest, Keccak256};
use libsecp256k1::{PublicKey, Message, Signature, RecoveryId, recover};
use candid::{CandidType, Deserialize};

/// Threshold ECDSA key identifier for Ethereum signatures
const ECDSA_KEY_NAME: &str = "key_1";

/// Simple Ethereum address representation (20 bytes)
#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct EthereumAddress(pub [u8; 20]);

impl std::fmt::Display for EthereumAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// Simple transaction hash representation (32 bytes)
#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TransactionHash(pub [u8; 32]);

impl std::fmt::Display for TransactionHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// Threshold ECDSA service for generating Ethereum addresses and signing transactions
pub struct ThresholdECDSA {
    key_id: EcdsaKeyId,
}

impl ThresholdECDSA {
    /// Create new threshold ECDSA service
    pub fn new() -> Self {
        Self {
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: ECDSA_KEY_NAME.to_string(),
            },
        }
    }

    /// Generate canister-controlled Ethereum address
    /// This is the core breakthrough that enables gasless bridges on ICP!
    pub async fn get_ethereum_address(&self) -> Result<EthereumAddress, String> {
        ic_cdk::println!("🔐 Generating Ethereum address from ICP threshold ECDSA...");
        // Get the canister's principal for derivation path
        let canister_id = ic_cdk::id();
        let derivation_path = vec![canister_id.as_slice().to_vec()];

        // Request public key from threshold ECDSA
        let public_key_request = EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path,
            key_id: self.key_id.clone(),
        };

        let response = ecdsa_public_key(public_key_request)
            .await
            .map_err(|e| format!("Failed to get ECDSA public key: {:?}", e))?;

        // Convert public key to Ethereum address
        let ethereum_address = self.public_key_to_address(&response.0.public_key)?;

        ic_cdk::println!("✅ Generated Ethereum address: {}", ethereum_address);
        Ok(ethereum_address)
    }

    /// Convert threshold ECDSA public key to Ethereum address
    /// Uses Keccak256 hash of uncompressed public key (last 20 bytes)
    fn public_key_to_address(&self, public_key_bytes: &[u8]) -> Result<EthereumAddress, String> {
        if public_key_bytes.len() != 33 {
            return Err("Invalid public key length. Expected 33 bytes (compressed)".to_string());
        }

        // Parse compressed public key
        let public_key = PublicKey::parse_compressed(
            &public_key_bytes.try_into()
                .map_err(|_| "Failed to convert public key bytes")?
        ).map_err(|e| format!("Failed to parse public key: {}", e))?;

        // Get uncompressed point (65 bytes: 0x04 + 32-byte x + 32-byte y)
        let uncompressed = public_key.serialize();

        // Hash the public key (skip the 0x04 prefix, use x + y coordinates)
        let mut hasher = Keccak256::new();
        hasher.update(&uncompressed[1..]);
        let hash = hasher.finalize();

        // Take last 20 bytes as Ethereum address
        let address_bytes: [u8; 20] = hash[12..32].try_into()
            .map_err(|_| "Failed to extract address from hash")?;

        Ok(EthereumAddress(address_bytes))
    }

    /// Sign Ethereum transaction hash using threshold ECDSA
    /// This is where the magic happens - ICP signs Ethereum transactions!
    pub async fn sign_transaction_hash(&self, message_hash: TransactionHash) -> Result<(Signature, RecoveryId), String> {
        ic_cdk::println!("✍️ Signing transaction hash: {}", hex::encode(message_hash.0));
        let canister_id = ic_cdk::id();
        let derivation_path = vec![canister_id.as_slice().to_vec()];

        let sign_request = SignWithEcdsaArgument {
            message_hash: message_hash.0.to_vec(),
            derivation_path,
            key_id: self.key_id.clone(),
        };

        let response = sign_with_ecdsa(sign_request)
            .await
            .map_err(|e| format!("Failed to sign with ECDSA: {:?}", e))?;

        // Parse signature from response
        let signature_bytes = &response.0.signature;
        if signature_bytes.len() != 64 {
            return Err("Invalid signature length. Expected 64 bytes".to_string());
        }

        // Extract r and s components and create signature
        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&signature_bytes[0..64]);
        let signature = Signature::parse_standard(&sig_array)
            .map_err(|e| format!("Failed to parse signature: {:?}", e))?;

        // Calculate recovery ID
        let recovery_id = self.calculate_recovery_id(&signature, &message_hash).await?;

        ic_cdk::println!("✅ Transaction signed successfully");
        Ok((signature, recovery_id))
    }

    /// Calculate recovery ID using proper libsecp256k1 recovery
    async fn calculate_recovery_id(&self, signature: &Signature, message_hash: &TransactionHash) -> Result<RecoveryId, String> {
        // Get expected public key for comparison
        let public_key_bytes = ecdsa_public_key(EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path: vec![ic_cdk::id().as_slice().to_vec()],
            key_id: self.key_id.clone(),
        }).await.map_err(|e| format!("Failed to get public key: {:?}", e))?.0.public_key;

        let expected_key = PublicKey::parse_compressed(
            &public_key_bytes.try_into().map_err(|_| "Invalid public key length")?
        ).map_err(|e| format!("Invalid expected public key: {:?}", e))?;

        // Create message from hash - Message::parse returns Message directly
        let message = Message::parse(&message_hash.0);

        // Test recovery IDs (0, 1, 2, 3)
        for recid_val in 0..4 {
            if let Ok(recovery_id) = RecoveryId::parse(recid_val) {
                // Try to recover public key using libsecp256k1::recover
                // FIXED: Pass &signature directly, not serialized bytes
                match recover(&message, signature, &recovery_id) {
                    Ok(recovered_key) => {
                        if recovered_key == expected_key {
                            ic_cdk::println!("✅ Found correct recovery ID: {}", recid_val);
                            return Ok(recovery_id);
                        } else {
                            ic_cdk::println!("❌ Recovery ID {} produces different key", recid_val);
                        }
                    }
                    Err(e) => {
                        ic_cdk::println!("❌ Recovery ID {} failed: {:?}", recid_val, e);
                        continue;
                    }
                }
            }
        }

        // If no recovery ID works, return error
        Err("Could not determine correct recovery ID".to_string())
    }

    /// Convert public key to Ethereum address (for recovery ID calculation)
    fn public_key_to_address_direct(&self, public_key: &PublicKey) -> Result<EthereumAddress, String> {
        // Get uncompressed point
        let uncompressed = public_key.serialize();

        // Hash the public key (skip the 0x04 prefix)
        let mut hasher = Keccak256::new();
        hasher.update(&uncompressed[1..]);
        let hash = hasher.finalize();

        // Take last 20 bytes as Ethereum address
        let address_bytes: [u8; 20] = hash[12..32].try_into()
            .map_err(|_| "Failed to extract address from hash")?;

        Ok(EthereumAddress(address_bytes))
    }

    /// Test function to verify threshold ECDSA setup
    pub async fn test_ecdsa_integration(&self) -> Result<String, String> {
        ic_cdk::println!("🧪 Testing ICP Threshold ECDSA integration...");

        // 1. Generate Ethereum address
        let address = self.get_ethereum_address().await?;

        // 2. Create a test message hash
        let test_message = "Hello from ICP Threshold ECDSA!";
        let mut hasher = Keccak256::new();
        hasher.update(test_message.as_bytes());
        let hash = hasher.finalize();
        let message_hash = TransactionHash(hash.into());

        // 3. Sign the test message
        let (signature, recovery_id) = self.sign_transaction_hash(message_hash.clone()).await?;

        // 4. Create result with comprehensive information
        let result = format!(
            "✅ ICP Threshold ECDSA integration successful!\n\
             🏠 Canister Ethereum Address: {}\n\
             📝 Test Message: {}\n\
             🔐 Message Hash: {}\n\
             ✍️ Signature: {}\n\
             🔄 Recovery ID: {}\n\
             ✅ Signature created successfully!",
            address,
            test_message,
            hex::encode(message_hash.0),
            hex::encode(signature.serialize()),
            recovery_id.serialize()
        );

        ic_cdk::println!("{}", result);
        Ok(result)
    }
}

impl Default for ThresholdECDSA {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static THRESHOLD_ECDSA: ThresholdECDSA = ThresholdECDSA::new();
}

/// Get the global threshold ECDSA service
pub fn get_threshold_ecdsa() -> &'static ThresholdECDSA {
    THRESHOLD_ECDSA.with(|ecdsa| unsafe {
        // SAFETY: This is safe because the thread_local is never modified
        std::mem::transmute::<&ThresholdECDSA, &'static ThresholdECDSA>(ecdsa)
    })
}

/// Public API functions for threshold ECDSA
/// Get the canister's Ethereum address
pub async fn get_canister_ethereum_address() -> Result<EthereumAddress, String> {
    let ecdsa = ThresholdECDSA::new();
    ecdsa.get_ethereum_address().await
}

/// Sign a transaction hash using threshold ECDSA
pub async fn sign_ethereum_transaction_hash(message_hash: TransactionHash) -> Result<(Signature, RecoveryId), String> {
    let ecdsa = ThresholdECDSA::new();
    ecdsa.sign_transaction_hash(message_hash).await
}

/// Test the threshold ECDSA integration
pub async fn test_threshold_ecdsa() -> Result<String, String> {
    let ecdsa = ThresholdECDSA::new();
    ecdsa.test_ecdsa_integration().await
}
