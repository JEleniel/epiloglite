//! Authentication module for EpilogLite server
//!
//! This module provides various authentication mechanisms including:
//! - OAuth 2.0 integration
//! - Custom authentication handlers
//! - Multi-factor authentication (MFA)
//! - JWT token management

#[cfg(feature = "server")]
use std::collections::HashMap;

#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::{Error, Result};

/// OAuth provider configuration
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
	/// Provider name (e.g., "google", "github")
	pub name: String,
	/// OAuth client ID
	pub client_id: String,
	/// OAuth client secret
	pub client_secret: String,
	/// Authorization endpoint URL
	pub auth_url: String,
	/// Token exchange endpoint URL
	pub token_url: String,
	/// User profile endpoint URL
	pub profile_url: String,
	/// OAuth scopes to request
	pub scopes: Vec<String>,
}

/// OAuth configuration with multiple providers
#[cfg(feature = "server")]
#[derive(Debug, Clone, Default)]
pub struct OAuthConfig {
	/// Registered OAuth providers
	providers: HashMap<String, OAuthProvider>,
	/// Redirect URI for OAuth callbacks
	pub redirect_uri: String,
}

#[cfg(feature = "server")]
impl OAuthConfig {
	/// Create a new OAuth configuration
	pub fn new(redirect_uri: String) -> Self {
		Self {
			providers: HashMap::new(),
			redirect_uri,
		}
	}

	/// Register a new OAuth provider
	pub fn register_provider(&mut self, provider: OAuthProvider) {
		self.providers.insert(provider.name.clone(), provider);
	}

	/// Get a provider by name
	pub fn get_provider(&self, name: &str) -> Option<&OAuthProvider> {
		self.providers.get(name)
	}

	/// Get all registered provider names
	pub fn provider_names(&self) -> Vec<String> {
		self.providers.keys().cloned().collect()
	}
}

/// OAuth authorization request
#[cfg(feature = "server")]
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthAuthRequest {
	/// Provider name
	pub provider: String,
	/// State parameter for CSRF protection
	pub state: String,
}

/// OAuth callback parameters
#[cfg(feature = "server")]
#[derive(Debug, Deserialize)]
pub struct OAuthCallback {
	/// Authorization code
	pub code: String,
	/// State parameter (must match request)
	pub state: String,
}

/// OAuth token response
#[cfg(feature = "server")]
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
	/// Access token
	pub access_token: String,
	/// Token type (usually "Bearer")
	pub token_type: String,
	/// Expiration time in seconds
	pub expires_in: Option<u64>,
	/// Refresh token (if provided)
	pub refresh_token: Option<String>,
}

/// User profile from OAuth provider
#[cfg(feature = "server")]
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
	/// User ID from provider
	pub id: String,
	/// Email address
	pub email: Option<String>,
	/// Display name
	pub name: Option<String>,
	/// Avatar URL
	pub avatar_url: Option<String>,
	/// Provider name
	pub provider: String,
}

/// Authentication handler trait for custom authentication
#[cfg(feature = "server")]
pub trait AuthHandler: Send + Sync {
	/// Authenticate a user with credentials
	fn authenticate(&self, credentials: &HashMap<String, String>) -> Result<UserProfile>;

	/// Validate a token
	fn validate_token(&self, token: &str) -> Result<UserProfile>;

	/// Handler name
	fn name(&self) -> &str;
}

/// Multi-factor authentication (MFA) configuration
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
	/// Enable TOTP-based MFA
	pub enable_totp: bool,
	/// Enable backup codes
	pub enable_backup_codes: bool,
	/// Enforce MFA for all users
	pub enforce_mfa: bool,
	/// Number of backup codes to generate
	pub backup_code_count: usize,
}

#[cfg(feature = "server")]
impl Default for MfaConfig {
	fn default() -> Self {
		Self {
			enable_totp: true,
			enable_backup_codes: true,
			enforce_mfa: false,
			backup_code_count: 10,
		}
	}
}

/// TOTP (Time-based One-Time Password) secret
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpSecret {
	/// Base32-encoded secret
	pub secret: String,
	/// Account name for QR code
	pub account: String,
	/// Issuer name for QR code
	pub issuer: String,
}

#[cfg(feature = "server")]
impl TotpSecret {
	/// Create a new TOTP secret
	pub fn new(account: String, issuer: String) -> Self {
		// Generate a random 32-character base32 secret
		let secret = Self::generate_secret();
		Self {
			secret,
			account,
			issuer,
		}
	}

	/// Generate a random base32 secret
	fn generate_secret() -> String {
		// Simple implementation - in production use a proper random generator
		use std::time::{SystemTime, UNIX_EPOCH};
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_secs();
		format!("SECRET{:016X}", timestamp)
	}

	/// Get the provisioning URI for QR code generation
	pub fn provisioning_uri(&self) -> String {
		format!(
			"otpauth://totp/{}:{}?secret={}&issuer={}",
			self.issuer, self.account, self.secret, self.issuer
		)
	}

	/// Verify a TOTP code
	pub fn verify_code(&self, code: &str) -> bool {
		// Simplified verification - in production use a proper TOTP library
		// This is a placeholder that accepts codes 000000-999999
		code.len() == 6 && code.chars().all(|c| c.is_ascii_digit())
	}
}

/// Backup code for MFA recovery
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCode {
	/// The backup code
	pub code: String,
	/// Whether the code has been used
	pub used: bool,
}

#[cfg(feature = "server")]
impl BackupCode {
	/// Generate a new backup code
	pub fn generate() -> Self {
		use std::time::{SystemTime, UNIX_EPOCH};
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos();
		
		Self {
			code: format!("{:012X}", timestamp % 0x1000000000000),
			used: false,
		}
	}

	/// Mark the code as used
	pub fn mark_used(&mut self) {
		self.used = true;
	}
}

/// Authentication manager
#[cfg(feature = "server")]
pub struct AuthManager {
	/// OAuth configuration
	pub oauth_config: OAuthConfig,
	/// Custom authentication handlers
	handlers: HashMap<String, Box<dyn AuthHandler>>,
	/// MFA configuration
	pub mfa_config: MfaConfig,
}

#[cfg(feature = "server")]
impl AuthManager {
	/// Create a new authentication manager
	pub fn new(oauth_config: OAuthConfig, mfa_config: MfaConfig) -> Self {
		Self {
			oauth_config,
			handlers: HashMap::new(),
			mfa_config,
		}
	}

	/// Register a custom authentication handler
	pub fn register_handler(&mut self, handler: Box<dyn AuthHandler>) {
		self.handlers.insert(handler.name().to_string(), handler);
	}

	/// Get an authentication handler by name
	pub fn get_handler(&self, name: &str) -> Option<&dyn AuthHandler> {
		self.handlers.get(name).map(|h| h.as_ref())
	}

	/// Generate OAuth authorization URL
	pub fn generate_oauth_url(&self, provider: &str, state: &str) -> Result<String> {
		let provider = self
			.oauth_config
			.get_provider(provider)
			.ok_or_else(|| Error::NotFound(format!("Unknown OAuth provider: {}", provider)))?;

		let scopes = provider.scopes.join(" ");
		Ok(format!(
			"{}?client_id={}&redirect_uri={}&scope={}&state={}&response_type=code",
			provider.auth_url,
			provider.client_id,
			self.oauth_config.redirect_uri,
			scopes,
			state
		))
	}

	/// Exchange OAuth code for tokens
	pub async fn exchange_oauth_code(
		&self,
		provider_name: &str,
		code: &str,
	) -> Result<OAuthTokenResponse> {
		let provider = self
			.oauth_config
			.get_provider(provider_name)
			.ok_or_else(|| Error::NotFound(format!("Unknown OAuth provider: {}", provider_name)))?;

		// In a real implementation, this would make an HTTP request to the token endpoint
		// For now, return a mock response
		Ok(OAuthTokenResponse {
			access_token: format!("mock_access_token_for_{}", code),
			token_type: "Bearer".to_string(),
			expires_in: Some(3600),
			refresh_token: Some(format!("mock_refresh_token_for_{}", code)),
		})
	}

	/// Fetch user profile from OAuth provider
	pub async fn fetch_user_profile(
		&self,
		provider_name: &str,
		access_token: &str,
	) -> Result<UserProfile> {
		let _provider = self
			.oauth_config
			.get_provider(provider_name)
			.ok_or_else(|| Error::NotFound(format!("Unknown OAuth provider: {}", provider_name)))?;

		// In a real implementation, this would make an HTTP request to the profile endpoint
		// For now, return a mock profile
		Ok(UserProfile {
			id: format!("user_{}", access_token.chars().take(8).collect::<String>()),
			email: Some("user@example.com".to_string()),
			name: Some("Mock User".to_string()),
			avatar_url: None,
			provider: provider_name.to_string(),
		})
	}

	/// Generate TOTP secret for a user
	pub fn generate_totp_secret(&self, account: String) -> TotpSecret {
		TotpSecret::new(account, "EpilogLite".to_string())
	}

	/// Verify TOTP code
	pub fn verify_totp(&self, secret: &TotpSecret, code: &str) -> bool {
		secret.verify_code(code)
	}

	/// Generate backup codes
	pub fn generate_backup_codes(&self) -> Vec<BackupCode> {
		(0..self.mfa_config.backup_code_count)
			.map(|_| BackupCode::generate())
			.collect()
	}

	/// Verify backup code
	pub fn verify_backup_code(&self, codes: &mut [BackupCode], code: &str) -> bool {
		if let Some(backup_code) = codes.iter_mut().find(|c| c.code == code && !c.used) {
			backup_code.mark_used();
			true
		} else {
			false
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_oauth_config_creation() {
		let config = OAuthConfig::new("http://localhost:8080/callback".to_string());
		assert_eq!(config.redirect_uri, "http://localhost:8080/callback");
		assert_eq!(config.provider_names().len(), 0);
	}

	#[test]
	fn test_oauth_provider_registration() {
		let mut config = OAuthConfig::new("http://localhost:8080/callback".to_string());
		
		let provider = OAuthProvider {
			name: "github".to_string(),
			client_id: "test_client_id".to_string(),
			client_secret: "test_secret".to_string(),
			auth_url: "https://github.com/login/oauth/authorize".to_string(),
			token_url: "https://github.com/login/oauth/access_token".to_string(),
			profile_url: "https://api.github.com/user".to_string(),
			scopes: vec!["user:email".to_string()],
		};

		config.register_provider(provider);
		assert_eq!(config.provider_names().len(), 1);
		assert!(config.get_provider("github").is_some());
	}

	#[test]
	fn test_mfa_config_default() {
		let config = MfaConfig::default();
		assert!(config.enable_totp);
		assert!(config.enable_backup_codes);
		assert!(!config.enforce_mfa);
		assert_eq!(config.backup_code_count, 10);
	}

	#[test]
	fn test_totp_secret_creation() {
		let secret = TotpSecret::new("user@example.com".to_string(), "EpilogLite".to_string());
		assert_eq!(secret.account, "user@example.com");
		assert_eq!(secret.issuer, "EpilogLite");
		assert!(!secret.secret.is_empty());
	}

	#[test]
	fn test_totp_provisioning_uri() {
		let secret = TotpSecret::new("user@example.com".to_string(), "EpilogLite".to_string());
		let uri = secret.provisioning_uri();
		assert!(uri.starts_with("otpauth://totp/"));
		assert!(uri.contains("user@example.com"));
		assert!(uri.contains("EpilogLite"));
	}

	#[test]
	fn test_totp_verify_code() {
		let secret = TotpSecret::new("user@example.com".to_string(), "EpilogLite".to_string());
		assert!(secret.verify_code("123456"));
		assert!(secret.verify_code("000000"));
		assert!(!secret.verify_code("12345")); // Too short
		assert!(!secret.verify_code("1234567")); // Too long
		assert!(!secret.verify_code("abcdef")); // Non-numeric
	}

	#[test]
	fn test_backup_code_generation() {
		let code = BackupCode::generate();
		assert_eq!(code.code.len(), 12);
		assert!(!code.used);
	}

	#[test]
	fn test_backup_code_usage() {
		let mut code = BackupCode::generate();
		assert!(!code.used);
		code.mark_used();
		assert!(code.used);
	}

	#[test]
	fn test_auth_manager_creation() {
		let oauth_config = OAuthConfig::new("http://localhost:8080/callback".to_string());
		let mfa_config = MfaConfig::default();
		let manager = AuthManager::new(oauth_config, mfa_config);
		
		assert_eq!(manager.oauth_config.redirect_uri, "http://localhost:8080/callback");
		assert!(manager.mfa_config.enable_totp);
	}

	#[test]
	fn test_generate_oauth_url() {
		let mut oauth_config = OAuthConfig::new("http://localhost:8080/callback".to_string());
		
		let provider = OAuthProvider {
			name: "github".to_string(),
			client_id: "test_client_id".to_string(),
			client_secret: "test_secret".to_string(),
			auth_url: "https://github.com/login/oauth/authorize".to_string(),
			token_url: "https://github.com/login/oauth/access_token".to_string(),
			profile_url: "https://api.github.com/user".to_string(),
			scopes: vec!["user:email".to_string()],
		};

		oauth_config.register_provider(provider);
		
		let mfa_config = MfaConfig::default();
		let manager = AuthManager::new(oauth_config, mfa_config);
		
		let url = manager.generate_oauth_url("github", "random_state").unwrap();
		assert!(url.contains("https://github.com/login/oauth/authorize"));
		assert!(url.contains("client_id=test_client_id"));
		assert!(url.contains("state=random_state"));
	}

	#[test]
	fn test_generate_backup_codes() {
		let oauth_config = OAuthConfig::new("http://localhost:8080/callback".to_string());
		let mfa_config = MfaConfig::default();
		let manager = AuthManager::new(oauth_config, mfa_config);
		
		let codes = manager.generate_backup_codes();
		assert_eq!(codes.len(), 10);
		assert!(codes.iter().all(|c| !c.used));
	}

	#[test]
	fn test_verify_backup_code() {
		let oauth_config = OAuthConfig::new("http://localhost:8080/callback".to_string());
		let mfa_config = MfaConfig::default();
		let manager = AuthManager::new(oauth_config, mfa_config);
		
		let mut codes = manager.generate_backup_codes();
		let test_code = codes[0].code.clone();
		
		// Should succeed on first use
		assert!(manager.verify_backup_code(&mut codes, &test_code));
		
		// Should fail on second use (code already used)
		assert!(!manager.verify_backup_code(&mut codes, &test_code));
	}
}
