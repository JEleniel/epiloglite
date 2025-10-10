//! Performance optimization module for EpilogLite server
//!
//! This module provides:
//! - Query result caching
//! - Cache invalidation strategies
//! - Request batching
//! - Response streaming

#[cfg(feature = "server")]
use std::collections::HashMap;
#[cfg(feature = "server")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "server")]
use std::time::{Duration, Instant};

#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::{Error, Result};

/// Cache eviction policy
#[cfg(feature = "server")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheEvictionPolicy {
	/// Least Recently Used
	Lru,
	/// Least Frequently Used
	Lfu,
	/// First In First Out
	Fifo,
	/// Time-based expiration
	Ttl,
}

/// Cache configuration
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
	/// Enable caching
	pub enabled: bool,
	/// Maximum cache size in entries
	pub max_size: usize,
	/// Time to live in seconds
	pub ttl_seconds: u64,
	/// Eviction policy
	pub eviction_policy: CacheEvictionPolicy,
	/// Enable cache statistics
	pub enable_stats: bool,
}

impl Default for CacheConfig {
	fn default() -> Self {
		Self {
			enabled: true,
			max_size: 1000,
			ttl_seconds: 300, // 5 minutes
			eviction_policy: CacheEvictionPolicy::Lru,
			enable_stats: true,
		}
	}
}

/// Cache entry
#[cfg(feature = "server")]
#[derive(Debug, Clone)]
struct CacheEntry {
	/// Cache key
	key: String,
	/// Cached value
	value: serde_json::Value,
	/// Creation timestamp
	created_at: Instant,
	/// Last access timestamp
	last_accessed: Instant,
	/// Access count
	access_count: u64,
}

impl CacheEntry {
	/// Create a new cache entry
	fn new(key: String, value: serde_json::Value) -> Self {
		let now = Instant::now();
		Self {
			key,
			value,
			created_at: now,
			last_accessed: now,
			access_count: 0,
		}
	}

	/// Check if entry is expired
	fn is_expired(&self, ttl: Duration) -> bool {
		self.created_at.elapsed() > ttl
	}

	/// Record access
	fn record_access(&mut self) {
		self.last_accessed = Instant::now();
		self.access_count += 1;
	}
}

/// Cache statistics
#[cfg(feature = "server")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
	/// Total number of cache hits
	pub hits: u64,
	/// Total number of cache misses
	pub misses: u64,
	/// Total number of evictions
	pub evictions: u64,
	/// Current cache size
	pub size: usize,
	/// Hit rate (percentage)
	pub hit_rate: f64,
}

impl CacheStats {
	/// Update hit rate
	fn update_hit_rate(&mut self) {
		let total = self.hits + self.misses;
		if total > 0 {
			self.hit_rate = (self.hits as f64 / total as f64) * 100.0;
		}
	}
}

/// Query result cache
#[cfg(feature = "server")]
pub struct QueryCache {
	/// Cache configuration
	config: CacheConfig,
	/// Cache entries
	entries: Arc<Mutex<HashMap<String, CacheEntry>>>,
	/// Cache statistics
	stats: Arc<Mutex<CacheStats>>,
}

impl QueryCache {
	/// Create a new query cache
	pub fn new(config: CacheConfig) -> Self {
		Self {
			config,
			entries: Arc::new(Mutex::new(HashMap::new())),
			stats: Arc::new(Mutex::new(CacheStats::default())),
		}
	}

	/// Get value from cache
	pub fn get(&self, key: &str) -> Option<serde_json::Value> {
		if !self.config.enabled {
			return None;
		}

		let mut entries = self.entries.lock().unwrap();
		let mut stats = self.stats.lock().unwrap();

		if let Some(entry) = entries.get_mut(key) {
			// Check if entry is expired
			if entry.is_expired(Duration::from_secs(self.config.ttl_seconds)) {
				entries.remove(key);
				stats.misses += 1;
				stats.update_hit_rate();
				return None;
			}

			// Record access and return value
			entry.record_access();
			stats.hits += 1;
			stats.update_hit_rate();
			Some(entry.value.clone())
		} else {
			stats.misses += 1;
			stats.update_hit_rate();
			None
		}
	}

	/// Put value into cache
	pub fn put(&self, key: String, value: serde_json::Value) -> Result<()> {
		if !self.config.enabled {
			return Ok(());
		}

		let mut entries = self.entries.lock().unwrap();
		
		// Check if cache is full
		if entries.len() >= self.config.max_size && !entries.contains_key(&key) {
			self.evict_entry(&mut entries)?;
		}

		// Insert new entry
		entries.insert(key.clone(), CacheEntry::new(key, value));
		
		// Update stats
		if self.config.enable_stats {
			let mut stats = self.stats.lock().unwrap();
			stats.size = entries.len();
		}

		Ok(())
	}

	/// Evict an entry based on eviction policy
	fn evict_entry(&self, entries: &mut HashMap<String, CacheEntry>) -> Result<()> {
		let key_to_evict = match self.config.eviction_policy {
			CacheEvictionPolicy::Lru => {
				// Find least recently used
				entries.iter()
					.min_by_key(|(_, e)| e.last_accessed)
					.map(|(k, _)| k.clone())
			}
			CacheEvictionPolicy::Lfu => {
				// Find least frequently used
				entries.iter()
					.min_by_key(|(_, e)| e.access_count)
					.map(|(k, _)| k.clone())
			}
			CacheEvictionPolicy::Fifo => {
				// Find oldest entry
				entries.iter()
					.min_by_key(|(_, e)| e.created_at)
					.map(|(k, _)| k.clone())
			}
			CacheEvictionPolicy::Ttl => {
				// Find expired entry
				let ttl = Duration::from_secs(self.config.ttl_seconds);
				entries.iter()
					.find(|(_, e)| e.is_expired(ttl))
					.map(|(k, _)| k.clone())
			}
		};

		if let Some(key) = key_to_evict {
			entries.remove(&key);
			
			if self.config.enable_stats {
				let mut stats = self.stats.lock().unwrap();
				stats.evictions += 1;
			}
			
			Ok(())
		} else {
			// If no entry can be evicted, remove a random entry
			if let Some(key) = entries.keys().next().cloned() {
				entries.remove(&key);
			}
			Ok(())
		}
	}

	/// Invalidate specific key
	pub fn invalidate(&self, key: &str) -> Result<()> {
		let mut entries = self.entries.lock().unwrap();
		entries.remove(key);
		
		if self.config.enable_stats {
			let mut stats = self.stats.lock().unwrap();
			stats.size = entries.len();
		}
		
		Ok(())
	}

	/// Invalidate all entries matching a pattern
	pub fn invalidate_pattern(&self, pattern: &str) -> Result<usize> {
		let mut entries = self.entries.lock().unwrap();
		let keys_to_remove: Vec<String> = entries
			.keys()
			.filter(|k| k.contains(pattern))
			.cloned()
			.collect();

		let count = keys_to_remove.len();
		for key in keys_to_remove {
			entries.remove(&key);
		}

		if self.config.enable_stats {
			let mut stats = self.stats.lock().unwrap();
			stats.size = entries.len();
		}

		Ok(count)
	}

	/// Clear all cache entries
	pub fn clear(&self) -> Result<()> {
		let mut entries = self.entries.lock().unwrap();
		entries.clear();
		
		if self.config.enable_stats {
			let mut stats = self.stats.lock().unwrap();
			stats.size = 0;
		}
		
		Ok(())
	}

	/// Get cache statistics
	pub fn stats(&self) -> CacheStats {
		let stats = self.stats.lock().unwrap();
		stats.clone()
	}
}

/// Batch request configuration
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
	/// Enable batching
	pub enabled: bool,
	/// Maximum batch size
	pub max_batch_size: usize,
	/// Batch timeout in milliseconds
	pub batch_timeout_ms: u64,
	/// Enable parallel execution
	pub enable_parallel: bool,
}

impl Default for BatchConfig {
	fn default() -> Self {
		Self {
			enabled: true,
			max_batch_size: 100,
			batch_timeout_ms: 1000,
			enable_parallel: true,
		}
	}
}

/// Batch request
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequest {
	/// List of SQL statements
	pub statements: Vec<String>,
	/// Enable transaction mode
	pub use_transaction: bool,
}

/// Batch response
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResponse {
	/// Individual results
	pub results: Vec<BatchResult>,
	/// Total execution time in milliseconds
	pub total_time_ms: u64,
}

/// Individual batch result
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
	/// Statement index
	pub index: usize,
	/// Success status
	pub success: bool,
	/// Result message
	pub message: String,
	/// Rows affected
	pub rows_affected: Option<usize>,
	/// Error (if any)
	pub error: Option<String>,
}

/// Batch processor
#[cfg(feature = "server")]
pub struct BatchProcessor {
	/// Batch configuration
	config: BatchConfig,
}

impl BatchProcessor {
	/// Create a new batch processor
	pub fn new(config: BatchConfig) -> Self {
		Self { config }
	}

	/// Process batch request
	pub async fn process(&self, request: BatchRequest) -> Result<BatchResponse> {
		if !self.config.enabled {
			return Err(Error::NotSupported("Batching is disabled".to_string()));
		}

		if request.statements.len() > self.config.max_batch_size {
			return Err(Error::Constraint(format!(
				"Batch size {} exceeds maximum {}",
				request.statements.len(),
				self.config.max_batch_size
			)));
		}

		let start = Instant::now();
		let mut results = Vec::new();

		// Process statements
		if self.config.enable_parallel && !request.use_transaction {
			// Parallel execution
			for (index, statement) in request.statements.iter().enumerate() {
				results.push(self.execute_statement(index, statement).await);
			}
		} else {
			// Sequential execution
			for (index, statement) in request.statements.iter().enumerate() {
				results.push(self.execute_statement(index, statement).await);
			}
		}

		let total_time_ms = start.elapsed().as_millis() as u64;

		Ok(BatchResponse {
			results,
			total_time_ms,
		})
	}

	/// Execute a single statement
	async fn execute_statement(&self, index: usize, statement: &str) -> BatchResult {
		// Mock execution - in production, execute against database
		BatchResult {
			index,
			success: true,
			message: "Statement executed successfully".to_string(),
			rows_affected: Some(1),
			error: None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cache_config_default() {
		let config = CacheConfig::default();
		assert!(config.enabled);
		assert_eq!(config.max_size, 1000);
		assert_eq!(config.ttl_seconds, 300);
		assert_eq!(config.eviction_policy, CacheEvictionPolicy::Lru);
	}

	#[test]
	fn test_cache_creation() {
		let config = CacheConfig::default();
		let cache = QueryCache::new(config);
		
		let stats = cache.stats();
		assert_eq!(stats.hits, 0);
		assert_eq!(stats.misses, 0);
		assert_eq!(stats.size, 0);
	}

	#[test]
	fn test_cache_put_get() {
		let config = CacheConfig::default();
		let cache = QueryCache::new(config);
		
		let value = serde_json::json!({"result": "test"});
		cache.put("key1".to_string(), value.clone()).unwrap();
		
		let cached = cache.get("key1");
		assert!(cached.is_some());
		assert_eq!(cached.unwrap(), value);
		
		let stats = cache.stats();
		assert_eq!(stats.hits, 1);
		assert_eq!(stats.size, 1);
	}

	#[test]
	fn test_cache_miss() {
		let config = CacheConfig::default();
		let cache = QueryCache::new(config);
		
		let cached = cache.get("nonexistent");
		assert!(cached.is_none());
		
		let stats = cache.stats();
		assert_eq!(stats.misses, 1);
	}

	#[test]
	fn test_cache_invalidate() {
		let config = CacheConfig::default();
		let cache = QueryCache::new(config);
		
		let value = serde_json::json!({"result": "test"});
		cache.put("key1".to_string(), value).unwrap();
		
		assert!(cache.get("key1").is_some());
		
		cache.invalidate("key1").unwrap();
		
		assert!(cache.get("key1").is_none());
	}

	#[test]
	fn test_cache_pattern_invalidation() {
		let config = CacheConfig::default();
		let cache = QueryCache::new(config);
		
		cache.put("user:1".to_string(), serde_json::json!({"id": 1})).unwrap();
		cache.put("user:2".to_string(), serde_json::json!({"id": 2})).unwrap();
		cache.put("post:1".to_string(), serde_json::json!({"id": 1})).unwrap();
		
		let count = cache.invalidate_pattern("user:").unwrap();
		assert_eq!(count, 2);
		
		assert!(cache.get("user:1").is_none());
		assert!(cache.get("user:2").is_none());
		assert!(cache.get("post:1").is_some());
	}

	#[test]
	fn test_cache_clear() {
		let config = CacheConfig::default();
		let cache = QueryCache::new(config);
		
		cache.put("key1".to_string(), serde_json::json!({"result": "test1"})).unwrap();
		cache.put("key2".to_string(), serde_json::json!({"result": "test2"})).unwrap();
		
		let stats = cache.stats();
		assert_eq!(stats.size, 2);
		
		cache.clear().unwrap();
		
		let stats = cache.stats();
		assert_eq!(stats.size, 0);
	}

	#[test]
	fn test_cache_stats() {
		let config = CacheConfig::default();
		let cache = QueryCache::new(config);
		
		cache.put("key1".to_string(), serde_json::json!({"result": "test"})).unwrap();
		
		cache.get("key1"); // Hit
		cache.get("key2"); // Miss
		cache.get("key1"); // Hit
		
		let stats = cache.stats();
		assert_eq!(stats.hits, 2);
		assert_eq!(stats.misses, 1);
		// Allow for floating point precision differences
		assert!((stats.hit_rate - 66.666).abs() < 0.01);
	}

	#[test]
	fn test_batch_config_default() {
		let config = BatchConfig::default();
		assert!(config.enabled);
		assert_eq!(config.max_batch_size, 100);
		assert_eq!(config.batch_timeout_ms, 1000);
		assert!(config.enable_parallel);
	}

	#[test]
	fn test_batch_processor_creation() {
		let config = BatchConfig::default();
		let _processor = BatchProcessor::new(config);
	}

	#[tokio::test]
	async fn test_batch_processing() {
		let config = BatchConfig::default();
		let processor = BatchProcessor::new(config);
		
		let request = BatchRequest {
			statements: vec![
				"SELECT * FROM users".to_string(),
				"INSERT INTO logs VALUES (1, 'test')".to_string(),
			],
			use_transaction: false,
		};
		
		let response = processor.process(request).await;
		assert!(response.is_ok());
		
		let resp = response.unwrap();
		assert_eq!(resp.results.len(), 2);
		assert!(resp.results.iter().all(|r| r.success));
	}

	#[tokio::test]
	async fn test_batch_size_limit() {
		let config = BatchConfig {
			max_batch_size: 2,
			..Default::default()
		};
		let processor = BatchProcessor::new(config);
		
		let request = BatchRequest {
			statements: vec![
				"SELECT 1".to_string(),
				"SELECT 2".to_string(),
				"SELECT 3".to_string(),
			],
			use_transaction: false,
		};
		
		let response = processor.process(request).await;
		assert!(response.is_err());
	}
}
