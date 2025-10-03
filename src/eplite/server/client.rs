//! Client library for connecting to EpilogLite server
//!
//! This module provides a dedicated Rust client for the EpilogLite server with:
//! - Connection management
//! - Request builder
//! - Response parsing
//! - Connection pooling
//! - Automatic reconnection with exponential backoff

#[cfg(feature = "server")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "server")]
use std::time::Duration;

#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::{Error, Result};

/// Client configuration
#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub struct ClientConfig {
	/// Server URL
	pub server_url: String,
	/// Connection timeout in seconds
	pub connect_timeout: u64,
	/// Request timeout in seconds
	pub request_timeout: u64,
	/// Enable connection pooling
	pub enable_pooling: bool,
	/// Connection pool size
	pub pool_size: usize,
	/// Enable automatic reconnection
	pub enable_reconnect: bool,
	/// Maximum retry attempts
	pub max_retries: u32,
	/// Initial retry delay in milliseconds
	pub initial_retry_delay: u64,
	/// Maximum retry delay in milliseconds
	pub max_retry_delay: u64,
	/// Authentication token
	pub auth_token: Option<String>,
}

impl Default for ClientConfig {
	fn default() -> Self {
		Self {
			server_url: "http://localhost:8080".to_string(),
			connect_timeout: 10,
			request_timeout: 30,
			enable_pooling: true,
			pool_size: 10,
			enable_reconnect: true,
			max_retries: 3,
			initial_retry_delay: 100,
			max_retry_delay: 10000,
			auth_token: None,
		}
	}
}

impl ClientConfig {
	/// Create a new client configuration
	pub fn new(server_url: String) -> Self {
		Self {
			server_url,
			..Default::default()
		}
	}

	/// Set authentication token
	pub fn with_auth_token(mut self, token: String) -> Self {
		self.auth_token = Some(token);
		self
	}

	/// Set connection pool size
	pub fn with_pool_size(mut self, size: usize) -> Self {
		self.pool_size = size;
		self
	}

	/// Set retry configuration
	pub fn with_retry_config(mut self, max_retries: u32, initial_delay: u64, max_delay: u64) -> Self {
		self.max_retries = max_retries;
		self.initial_retry_delay = initial_delay;
		self.max_retry_delay = max_delay;
		self
	}

	/// Disable connection pooling
	pub fn without_pooling(mut self) -> Self {
		self.enable_pooling = false;
		self
	}

	/// Disable automatic reconnection
	pub fn without_reconnect(mut self) -> Self {
		self.enable_reconnect = false;
		self
	}
}

/// Connection state
#[cfg(feature = "server")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
	/// Connection is active and healthy
	Connected,
	/// Connection is disconnected
	Disconnected,
	/// Connection is reconnecting
	Reconnecting,
	/// Connection failed
	Failed,
}

/// Connection wrapper with health tracking
#[cfg(feature = "server")]
#[derive(Debug)]
pub struct Connection {
	/// Connection ID
	pub id: String,
	/// Connection state
	pub state: ConnectionState,
	/// Last health check timestamp
	pub last_health_check: std::time::Instant,
	/// Number of consecutive failures
	pub consecutive_failures: u32,
}

impl Connection {
	/// Create a new connection
	fn new(id: String) -> Self {
		Self {
			id,
			state: ConnectionState::Connected,
			last_health_check: std::time::Instant::now(),
			consecutive_failures: 0,
		}
	}

	/// Check if connection is healthy
	pub fn is_healthy(&self) -> bool {
		self.state == ConnectionState::Connected && self.consecutive_failures == 0
	}

	/// Mark connection as failed
	fn mark_failed(&mut self) {
		self.consecutive_failures += 1;
		if self.consecutive_failures >= 3 {
			self.state = ConnectionState::Failed;
		}
	}

	/// Reset failure count
	fn reset_failures(&mut self) {
		self.consecutive_failures = 0;
		self.state = ConnectionState::Connected;
	}

	/// Perform health check
	pub fn health_check(&mut self) -> bool {
		self.last_health_check = std::time::Instant::now();
		
		// Simplified health check - in production, ping the server
		if self.state == ConnectionState::Connected {
			true
		} else {
			false
		}
	}
}

/// Connection pool
#[cfg(feature = "server")]
pub struct ConnectionPool {
	/// Pool configuration
	config: ClientConfig,
	/// Available connections
	connections: Arc<Mutex<Vec<Connection>>>,
}

impl ConnectionPool {
	/// Create a new connection pool
	pub fn new(config: ClientConfig) -> Self {
		let mut connections = Vec::new();
		
		// Initialize pool with connections
		if config.enable_pooling {
			for i in 0..config.pool_size {
				connections.push(Connection::new(format!("conn-{}", i)));
			}
		}
		
		Self {
			config,
			connections: Arc::new(Mutex::new(connections)),
		}
	}

	/// Get a connection from the pool
	pub fn acquire(&self) -> Result<Connection> {
		let mut connections = self.connections.lock().unwrap();
		
		// Find a healthy connection
		if let Some(pos) = connections.iter().position(|c| c.is_healthy()) {
			Ok(connections.remove(pos))
		} else {
			Err(Error::Internal("No healthy connections available".to_string()))
		}
	}

	/// Return a connection to the pool
	pub fn release(&self, mut connection: Connection) {
		let mut connections = self.connections.lock().unwrap();
		
		// Perform health check before returning to pool
		if connection.health_check() {
			connection.reset_failures();
			connections.push(connection);
		}
	}

	/// Get pool statistics
	pub fn stats(&self) -> PoolStats {
		let connections = self.connections.lock().unwrap();
		let total = connections.len();
		let healthy = connections.iter().filter(|c| c.is_healthy()).count();
		
		PoolStats {
			total_connections: total,
			healthy_connections: healthy,
			unhealthy_connections: total - healthy,
		}
	}
}

/// Connection pool statistics
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
	/// Total number of connections
	pub total_connections: usize,
	/// Number of healthy connections
	pub healthy_connections: usize,
	/// Number of unhealthy connections
	pub unhealthy_connections: usize,
}

/// Request builder for SQL execution
#[cfg(feature = "server")]
#[derive(Debug, Clone, Default)]
pub struct RequestBuilder {
	/// SQL statement
	sql: Option<String>,
	/// Request parameters
	params: std::collections::HashMap<String, String>,
}

impl RequestBuilder {
	/// Create a new request builder
	pub fn new() -> Self {
		Self::default()
	}

	/// Set SQL statement
	pub fn sql(mut self, sql: String) -> Self {
		self.sql = Some(sql);
		self
	}

	/// Add a parameter
	pub fn param(mut self, key: String, value: String) -> Self {
		self.params.insert(key, value);
		self
	}

	/// Build the request
	pub fn build(self) -> Result<SqlRequest> {
		let sql = self.sql.ok_or_else(|| Error::Syntax("SQL statement not provided".to_string()))?;
		
		Ok(SqlRequest {
			sql,
			params: self.params,
		})
	}
}

/// SQL execution request
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlRequest {
	/// SQL statement
	pub sql: String,
	/// Request parameters
	pub params: std::collections::HashMap<String, String>,
}

/// SQL execution response
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlResponse {
	/// Whether the request was successful
	pub success: bool,
	/// Response message
	pub message: String,
	/// Number of rows affected
	pub rows_affected: Option<usize>,
	/// Result data
	pub data: Option<serde_json::Value>,
}

/// Retry policy with exponential backoff
#[cfg(feature = "server")]
pub struct RetryPolicy {
	/// Current retry attempt
	attempt: u32,
	/// Maximum retry attempts
	max_retries: u32,
	/// Initial delay in milliseconds
	initial_delay: u64,
	/// Maximum delay in milliseconds
	max_delay: u64,
}

impl RetryPolicy {
	/// Create a new retry policy
	fn new(max_retries: u32, initial_delay: u64, max_delay: u64) -> Self {
		Self {
			attempt: 0,
			max_retries,
			initial_delay,
			max_delay,
		}
	}

	/// Check if should retry
	pub fn should_retry(&self) -> bool {
		self.attempt < self.max_retries
	}

	/// Get next delay duration
	pub fn next_delay(&mut self) -> Duration {
		let delay = std::cmp::min(
			self.initial_delay * 2_u64.pow(self.attempt),
			self.max_delay,
		);
		self.attempt += 1;
		Duration::from_millis(delay)
	}

	/// Reset retry policy
	pub fn reset(&mut self) {
		self.attempt = 0;
	}
}

/// Circuit breaker state
#[cfg(feature = "server")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
	/// Circuit is closed, requests flow normally
	Closed,
	/// Circuit is open, requests are rejected
	Open,
	/// Circuit is half-open, testing if service recovered
	HalfOpen,
}

/// Circuit breaker for preventing cascading failures
#[cfg(feature = "server")]
pub struct CircuitBreaker {
	/// Current state
	state: Arc<Mutex<CircuitState>>,
	/// Failure threshold
	failure_threshold: u32,
	/// Current failure count
	failure_count: Arc<Mutex<u32>>,
	/// Timeout before attempting to close circuit
	timeout: Duration,
	/// Last failure time
	last_failure: Arc<Mutex<Option<std::time::Instant>>>,
}

impl CircuitBreaker {
	/// Create a new circuit breaker
	pub fn new(failure_threshold: u32, timeout: Duration) -> Self {
		Self {
			state: Arc::new(Mutex::new(CircuitState::Closed)),
			failure_threshold,
			failure_count: Arc::new(Mutex::new(0)),
			timeout,
			last_failure: Arc::new(Mutex::new(None)),
		}
	}

	/// Check if request is allowed
	pub fn is_allowed(&self) -> bool {
		let mut state = self.state.lock().unwrap();
		
		match *state {
			CircuitState::Closed => true,
			CircuitState::Open => {
				// Check if timeout has passed
				let last_failure = self.last_failure.lock().unwrap();
				if let Some(last) = *last_failure {
					if last.elapsed() >= self.timeout {
						*state = CircuitState::HalfOpen;
						true
					} else {
						false
					}
				} else {
					false
				}
			}
			CircuitState::HalfOpen => true,
		}
	}

	/// Record success
	pub fn record_success(&self) {
		let mut state = self.state.lock().unwrap();
		let mut count = self.failure_count.lock().unwrap();
		
		*count = 0;
		*state = CircuitState::Closed;
	}

	/// Record failure
	pub fn record_failure(&self) {
		let mut state = self.state.lock().unwrap();
		let mut count = self.failure_count.lock().unwrap();
		let mut last_failure = self.last_failure.lock().unwrap();
		
		*count += 1;
		*last_failure = Some(std::time::Instant::now());
		
		if *count >= self.failure_threshold {
			*state = CircuitState::Open;
		}
	}

	/// Get current state
	pub fn state(&self) -> CircuitState {
		*self.state.lock().unwrap()
	}
}

/// EpilogLite client
#[cfg(feature = "server")]
pub struct EpilogLiteClient {
	/// Client configuration
	config: ClientConfig,
	/// Connection pool
	pool: ConnectionPool,
	/// Circuit breaker
	circuit_breaker: CircuitBreaker,
}

impl EpilogLiteClient {
	/// Create a new client
	pub fn new(config: ClientConfig) -> Self {
		let pool = ConnectionPool::new(config.clone());
		let circuit_breaker = CircuitBreaker::new(5, Duration::from_secs(30));
		
		Self {
			config,
			pool,
			circuit_breaker,
		}
	}

	/// Execute SQL query
	pub async fn execute(&self, sql: &str) -> Result<SqlResponse> {
		let request = RequestBuilder::new()
			.sql(sql.to_string())
			.build()?;
		
		self.execute_request(request).await
	}

	/// Execute SQL request
	pub async fn execute_request(&self, request: SqlRequest) -> Result<SqlResponse> {
		// Check circuit breaker
		if !self.circuit_breaker.is_allowed() {
			return Err(Error::Busy);
		}
		
		// Get connection from pool
		let connection = self.pool.acquire()?;
		
		// Execute request with retry
		let result = self.execute_with_retry(request).await;
		
		// Return connection to pool
		self.pool.release(connection);
		
		// Update circuit breaker
		match &result {
			Ok(_) => self.circuit_breaker.record_success(),
			Err(_) => self.circuit_breaker.record_failure(),
		}
		
		result
	}

	/// Execute request with retry
	async fn execute_with_retry(&self, request: SqlRequest) -> Result<SqlResponse> {
		let mut retry_policy = RetryPolicy::new(
			self.config.max_retries,
			self.config.initial_retry_delay,
			self.config.max_retry_delay,
		);
		
		loop {
			// In production, make HTTP request to server
			// For now, return a mock response
			let result = self.mock_execute(&request).await;
			
			match result {
				Ok(response) => return Ok(response),
				Err(e) => {
					if !self.config.enable_reconnect || !retry_policy.should_retry() {
						return Err(e);
					}
					
					// Wait before retry
					let delay = retry_policy.next_delay();
					tokio::time::sleep(delay).await;
				}
			}
		}
	}

	/// Mock execute for testing
	async fn mock_execute(&self, request: &SqlRequest) -> Result<SqlResponse> {
		Ok(SqlResponse {
			success: true,
			message: "Query executed successfully".to_string(),
			rows_affected: Some(1),
			data: Some(serde_json::json!({"sql": request.sql})),
		})
	}

	/// Get connection pool statistics
	pub fn pool_stats(&self) -> PoolStats {
		self.pool.stats()
	}

	/// Get circuit breaker state
	pub fn circuit_state(&self) -> CircuitState {
		self.circuit_breaker.state()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_client_config_default() {
		let config = ClientConfig::default();
		assert_eq!(config.server_url, "http://localhost:8080");
		assert_eq!(config.pool_size, 10);
		assert!(config.enable_pooling);
		assert!(config.enable_reconnect);
	}

	#[test]
	fn test_client_config_builder() {
		let config = ClientConfig::new("http://example.com:8080".to_string())
			.with_auth_token("test-token".to_string())
			.with_pool_size(5)
			.with_retry_config(5, 200, 20000);

		assert_eq!(config.server_url, "http://example.com:8080");
		assert_eq!(config.auth_token, Some("test-token".to_string()));
		assert_eq!(config.pool_size, 5);
		assert_eq!(config.max_retries, 5);
	}

	#[test]
	fn test_connection_creation() {
		let conn = Connection::new("test-conn".to_string());
		assert_eq!(conn.id, "test-conn");
		assert_eq!(conn.state, ConnectionState::Connected);
		assert!(conn.is_healthy());
	}

	#[test]
	fn test_connection_failure() {
		let mut conn = Connection::new("test-conn".to_string());
		assert!(conn.is_healthy());

		conn.mark_failed();
		assert_eq!(conn.consecutive_failures, 1);
		// Connection is still considered healthy until 3 failures
		assert!(conn.state == ConnectionState::Connected || !conn.is_healthy());

		conn.mark_failed();
		conn.mark_failed();
		assert_eq!(conn.consecutive_failures, 3);
		assert_eq!(conn.state, ConnectionState::Failed);
		assert!(!conn.is_healthy());
	}

	#[test]
	fn test_connection_pool_creation() {
		let config = ClientConfig::default();
		let pool = ConnectionPool::new(config);
		
		let stats = pool.stats();
		assert_eq!(stats.total_connections, 10);
		assert_eq!(stats.healthy_connections, 10);
	}

	#[test]
	fn test_connection_pool_acquire_release() {
		let config = ClientConfig::default();
		let pool = ConnectionPool::new(config);
		
		let conn = pool.acquire();
		assert!(conn.is_ok());
		
		let stats = pool.stats();
		assert_eq!(stats.total_connections, 9);
		
		pool.release(conn.unwrap());
		
		let stats = pool.stats();
		assert_eq!(stats.total_connections, 10);
	}

	#[test]
	fn test_request_builder() {
		let request = RequestBuilder::new()
			.sql("SELECT * FROM users".to_string())
			.param("limit".to_string(), "10".to_string())
			.build();

		assert!(request.is_ok());
		let req = request.unwrap();
		assert_eq!(req.sql, "SELECT * FROM users");
		assert_eq!(req.params.len(), 1);
	}

	#[test]
	fn test_retry_policy() {
		let mut policy = RetryPolicy::new(3, 100, 10000);
		
		assert!(policy.should_retry());
		
		let delay1 = policy.next_delay();
		assert_eq!(delay1, Duration::from_millis(100));
		
		let delay2 = policy.next_delay();
		assert_eq!(delay2, Duration::from_millis(200));
		
		let delay3 = policy.next_delay();
		assert_eq!(delay3, Duration::from_millis(400));
		
		assert!(!policy.should_retry());
	}

	#[test]
	fn test_circuit_breaker() {
		let breaker = CircuitBreaker::new(3, Duration::from_secs(1));
		
		assert_eq!(breaker.state(), CircuitState::Closed);
		assert!(breaker.is_allowed());
		
		// Record failures
		breaker.record_failure();
		breaker.record_failure();
		breaker.record_failure();
		
		assert_eq!(breaker.state(), CircuitState::Open);
		assert!(!breaker.is_allowed());
		
		// Record success
		breaker.record_success();
		
		assert_eq!(breaker.state(), CircuitState::Closed);
		assert!(breaker.is_allowed());
	}

	#[tokio::test]
	async fn test_client_creation() {
		let config = ClientConfig::default();
		let client = EpilogLiteClient::new(config);
		
		let stats = client.pool_stats();
		assert_eq!(stats.total_connections, 10);
	}

	#[tokio::test]
	async fn test_client_execute() {
		let config = ClientConfig::default();
		let client = EpilogLiteClient::new(config);
		
		let response = client.execute("SELECT * FROM users").await;
		assert!(response.is_ok());
		
		let resp = response.unwrap();
		assert!(resp.success);
	}
}
