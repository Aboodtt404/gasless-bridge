use candid::{CandidType, Deserialize};
use std::collections::HashMap;
use ic_cdk::api::time;

/// RPC response cache for improving performance
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CachedResponse {
    pub data: String,
    pub timestamp: u64,
    pub ttl_seconds: u64,
}

impl CachedResponse {
    pub fn new(data: String, ttl_seconds: u64) -> Self {
        Self {
            data,
            timestamp: time() / 1_000_000_000, // Convert to seconds
            ttl_seconds,
        }
    }

    pub fn is_expired(&self) -> bool {
        let current_time = time() / 1_000_000_000;
        current_time > self.timestamp + self.ttl_seconds
    }

    pub fn age_seconds(&self) -> u64 {
        let current_time = time() / 1_000_000_000;
        current_time.saturating_sub(self.timestamp)
    }
}

/// High-performance RPC cache with smart invalidation
pub struct RpcCache {
    cache: HashMap<String, CachedResponse>,
    max_entries: usize,
    hit_count: u64,
    miss_count: u64,
}

impl RpcCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_entries,
            hit_count: 0,
            miss_count: 0,
        }
    }

    /// Generate cache key for gas estimation
    pub fn gas_estimation_key(chain: &str) -> String {
        format!("gas_estimate_{}", chain)
    }

    /// Generate cache key for nonce
    pub fn nonce_key(address: &str, chain: &str) -> String {
        format!("nonce_{}_{}", address, chain)
    }

    /// Generate cache key for block number
    pub fn block_number_key(chain: &str) -> String {
        format!("block_number_{}", chain)
    }

    /// Get cached response if valid
    pub fn get(&mut self, key: &str) -> Option<String> {
        // Check if key exists and if it's expired
        let should_remove = if let Some(cached) = self.cache.get(key) {
            if !cached.is_expired() {
                self.hit_count += 1;
                ic_cdk::println!("🎯 Cache HIT for {}: age {}s", key, cached.age_seconds());
                return Some(cached.data.clone());
            } else {
                let age = cached.age_seconds();
                ic_cdk::println!("⏰ Cache EXPIRED for {}: was {}s old", key, age);
                true // Mark for removal
            }
        } else {
            false
        };
        
        // Remove expired entry if needed
        if should_remove {
            self.cache.remove(key);
        }
        
        self.miss_count += 1;
        ic_cdk::println!("❌ Cache MISS for {}", key);
        None
    }

    /// Store response in cache with TTL
    pub fn set(&mut self, key: String, data: String, ttl_seconds: u64) {
        // Implement LRU eviction if at capacity
        if self.cache.len() >= self.max_entries && !self.cache.contains_key(&key) {
            self.evict_oldest();
        }

        let cached_response = CachedResponse::new(data, ttl_seconds);
        self.cache.insert(key.clone(), cached_response);
        
        ic_cdk::println!("💾 Cache SET for {} (TTL: {}s)", key, ttl_seconds);
    }

    /// Evict oldest entry (simple LRU)
    fn evict_oldest(&mut self) {
        if let Some((oldest_key, _)) = self.cache.iter()
            .min_by_key(|(_, v)| v.timestamp)
            .map(|(k, v)| (k.clone(), v.clone())) {
            self.cache.remove(&oldest_key);
            ic_cdk::println!("🗑️ Cache EVICTED oldest entry: {}", oldest_key);
        }
    }

    /// Clear expired entries
    pub fn cleanup_expired(&mut self) {
        let expired_keys: Vec<String> = self.cache.iter()
            .filter(|(_, v)| v.is_expired())
            .map(|(k, _)| k.clone())
            .collect();

        for key in expired_keys {
            self.cache.remove(&key);
        }
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        let total_requests = self.hit_count + self.miss_count;
        let hit_rate = if total_requests > 0 {
            (self.hit_count as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        CacheStats {
            entries: self.cache.len(),
            max_entries: self.max_entries,
            hit_count: self.hit_count,
            miss_count: self.miss_count,
            hit_rate_percent: hit_rate,
        }
    }

    /// Invalidate specific cache entries (e.g., after new block)
    pub fn invalidate_gas_estimates(&mut self) {
        let gas_keys: Vec<String> = self.cache.keys()
            .filter(|k| k.starts_with("gas_estimate_"))
            .cloned()
            .collect();

        for key in gas_keys {
            self.cache.remove(&key);
        }
        
        ic_cdk::println!("🔄 Invalidated gas estimate cache entries");
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CacheStats {
    pub entries: usize,
    pub max_entries: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate_percent: f64,
}

/// TTL constants for different data types
pub mod ttl {
    /// Gas estimates change every block (~2 seconds on Base)
    pub const GAS_ESTIMATE: u64 = 10;
    
    /// Nonces change after each transaction
    pub const NONCE: u64 = 5;
    
    /// Block numbers update every block
    pub const BLOCK_NUMBER: u64 = 2;
    
    /// Fee history can be cached longer
    pub const FEE_HISTORY: u64 = 15;
}
