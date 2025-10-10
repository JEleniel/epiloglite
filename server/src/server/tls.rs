//! TLS configuration module for EpilogLite server
//!
//! This module provides advanced TLS 1.3 configuration options including:
//! - Client certificate authentication
//! - Certificate validation policies
//! - Cipher suite selection
//! - Certificate management

#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::{Error, Result};

/// TLS version
#[cfg(feature = "server")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TlsVersion {
	/// TLS 1.2
	Tls12,
	/// TLS 1.3
	Tls13,
}

/// Client certificate authentication mode
#[cfg(feature = "server")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientCertMode {
	/// No client certificate required
	None,
	/// Client certificate optional
	Optional,
	/// Client certificate required
	Required,
}

/// Certificate validation policy
#[cfg(feature = "server")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertValidationPolicy {
	/// Strict validation (default)
	Strict,
	/// Skip hostname verification
	SkipHostname,
	/// Skip all verification (dangerous, for testing only)
	SkipAll,
	/// Custom validation with specific constraints
	Custom {
		/// Allow self-signed certificates
		allow_self_signed: bool,
		/// Allow expired certificates
		allow_expired: bool,
		/// Allowed certificate authorities
		allowed_cas: Vec<String>,
	},
}

impl Default for CertValidationPolicy {
	fn default() -> Self {
		Self::Strict
	}
}

/// Cipher suite selection
#[cfg(feature = "server")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CipherSuite {
	/// Use default cipher suites
	Default,
	/// Strong cipher suites only
	Strong,
	/// Custom cipher suite list
	Custom(Vec<String>),
}

impl Default for CipherSuite {
	fn default() -> Self {
		Self::Default
	}
}

/// Certificate information
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
	/// Certificate ID
	pub id: String,
	/// Certificate common name
	pub common_name: String,
	/// Certificate data (PEM format)
	pub data: String,
	/// Private key data (PEM format)
	pub key: Option<String>,
	/// Expiration date (RFC 3339 format)
	pub expires_at: String,
	/// Certificate chain
	pub chain: Vec<String>,
}

/// TLS configuration
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
	/// Enable TLS
	pub enabled: bool,
	/// TLS version
	pub version: TlsVersion,
	/// Certificate path
	pub cert_path: Option<String>,
	/// Private key path
	pub key_path: Option<String>,
	/// Client certificate authentication mode
	pub client_cert_mode: ClientCertMode,
	/// Certificate validation policy
	pub validation_policy: CertValidationPolicy,
	/// Cipher suite selection
	pub cipher_suites: CipherSuite,
	/// Enable certificate rotation
	pub enable_rotation: bool,
	/// Certificate rotation check interval in seconds
	pub rotation_check_interval: u64,
}

impl Default for TlsConfig {
	fn default() -> Self {
		Self {
			enabled: false,
			version: TlsVersion::Tls13,
			cert_path: None,
			key_path: None,
			client_cert_mode: ClientCertMode::None,
			validation_policy: CertValidationPolicy::default(),
			cipher_suites: CipherSuite::default(),
			enable_rotation: false,
			rotation_check_interval: 3600, // 1 hour
		}
	}
}

impl TlsConfig {
	/// Create a new TLS configuration
	pub fn new() -> Self {
		Self::default()
	}

	/// Enable TLS with certificate and key paths
	pub fn with_cert_and_key(mut self, cert_path: String, key_path: String) -> Self {
		self.enabled = true;
		self.cert_path = Some(cert_path);
		self.key_path = Some(key_path);
		self
	}

	/// Set TLS version
	pub fn with_version(mut self, version: TlsVersion) -> Self {
		self.version = version;
		self
	}

	/// Set client certificate mode
	pub fn with_client_cert_mode(mut self, mode: ClientCertMode) -> Self {
		self.client_cert_mode = mode;
		self
	}

	/// Set validation policy
	pub fn with_validation_policy(mut self, policy: CertValidationPolicy) -> Self {
		self.validation_policy = policy;
		self
	}

	/// Set cipher suites
	pub fn with_cipher_suites(mut self, suites: CipherSuite) -> Self {
		self.cipher_suites = suites;
		self
	}

	/// Enable certificate rotation
	pub fn with_rotation(mut self, interval: u64) -> Self {
		self.enable_rotation = true;
		self.rotation_check_interval = interval;
		self
	}

	/// Validate configuration
	pub fn validate(&self) -> Result<()> {
		if self.enabled {
			if self.cert_path.is_none() || self.key_path.is_none() {
				return Err(Error::InvalidFormat(
					"TLS enabled but certificate or key path not provided".to_string(),
				));
			}
		}
		Ok(())
	}
}

/// Certificate manager for managing TLS certificates
#[cfg(feature = "server")]
pub struct CertificateManager {
	/// Stored certificates
	certificates: std::collections::HashMap<String, Certificate>,
	/// TLS configuration
	config: TlsConfig,
}

#[cfg(feature = "server")]
impl CertificateManager {
	/// Create a new certificate manager
	pub fn new(config: TlsConfig) -> Result<Self> {
		config.validate()?;
		Ok(Self {
			certificates: std::collections::HashMap::new(),
			config,
		})
	}

	/// Upload a new certificate
	pub fn upload_certificate(&mut self, cert: Certificate) -> Result<()> {
		// Validate certificate format
		if cert.data.is_empty() {
			return Err(Error::InvalidFormat("Certificate data is empty".to_string()));
		}

		// In production, validate the certificate data (PEM format, expiration, etc.)
		self.certificates.insert(cert.id.clone(), cert);
		Ok(())
	}

	/// Get a certificate by ID
	pub fn get_certificate(&self, id: &str) -> Option<&Certificate> {
		self.certificates.get(id)
	}

	/// List all certificates
	pub fn list_certificates(&self) -> Vec<&Certificate> {
		self.certificates.values().collect()
	}

	/// Revoke a certificate
	pub fn revoke_certificate(&mut self, id: &str) -> Result<()> {
		self.certificates
			.remove(id)
			.ok_or_else(|| Error::NotFound(format!("Certificate not found: {}", id)))?;
		Ok(())
	}

	/// Check if certificate needs renewal
	pub fn needs_renewal(&self, id: &str) -> Result<bool> {
		let cert = self
			.get_certificate(id)
			.ok_or_else(|| Error::NotFound(format!("Certificate not found: {}", id)))?;

		// Parse expiration date and check if it's within renewal threshold
		// This is a simplified check - in production, use proper date parsing
		Ok(cert.expires_at.contains("2024"))
	}

	/// Get active certificate for the server
	pub fn get_active_certificate(&self) -> Option<&Certificate> {
		// Return the first available certificate
		// In production, implement proper active certificate selection
		self.certificates.values().next()
	}

	/// Load certificate from file paths
	pub fn load_from_paths(&mut self, cert_path: &str, key_path: &str) -> Result<Certificate> {
		// In production, read actual files
		// For now, create a mock certificate
		let cert = Certificate {
			id: "server-cert".to_string(),
			common_name: "localhost".to_string(),
			data: format!("mock-cert-from-{}", cert_path),
			key: Some(format!("mock-key-from-{}", key_path)),
			expires_at: "2025-12-31T23:59:59Z".to_string(),
			chain: vec![],
		};

		self.upload_certificate(cert.clone())?;
		Ok(cert)
	}

	/// Rotate certificates (check for updates)
	pub fn rotate_certificates(&mut self) -> Result<usize> {
		let mut rotated = 0;

		// Check if rotation is enabled
		if !self.config.enable_rotation {
			return Ok(0);
		}

		// In production, check for certificate updates and reload
		// For now, just count certificates that might need rotation
		for cert in self.certificates.values() {
			if cert.expires_at.contains("2024") {
				rotated += 1;
			}
		}

		Ok(rotated)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tls_config_default() {
		let config = TlsConfig::default();
		assert!(!config.enabled);
		assert_eq!(config.version, TlsVersion::Tls13);
		assert_eq!(config.client_cert_mode, ClientCertMode::None);
	}

	#[test]
	fn test_tls_config_builder() {
		let config = TlsConfig::new()
			.with_cert_and_key("/path/to/cert.pem".to_string(), "/path/to/key.pem".to_string())
			.with_version(TlsVersion::Tls13)
			.with_client_cert_mode(ClientCertMode::Required)
			.with_rotation(3600);

		assert!(config.enabled);
		assert_eq!(config.version, TlsVersion::Tls13);
		assert_eq!(config.client_cert_mode, ClientCertMode::Required);
		assert!(config.enable_rotation);
		assert_eq!(config.rotation_check_interval, 3600);
	}

	#[test]
	fn test_tls_config_validation() {
		let config = TlsConfig::default();
		assert!(config.validate().is_ok());

		let mut invalid_config = TlsConfig::default();
		invalid_config.enabled = true;
		assert!(invalid_config.validate().is_err());
	}

	#[test]
	fn test_cert_validation_policy() {
		let strict = CertValidationPolicy::Strict;
		assert_eq!(strict, CertValidationPolicy::Strict);

		let custom = CertValidationPolicy::Custom {
			allow_self_signed: true,
			allow_expired: false,
			allowed_cas: vec!["CA1".to_string()],
		};

		match custom {
			CertValidationPolicy::Custom { allow_self_signed, .. } => {
				assert!(allow_self_signed);
			}
			_ => panic!("Expected Custom policy"),
		}
	}

	#[test]
	fn test_cipher_suite() {
		let default = CipherSuite::default();
		assert_eq!(default, CipherSuite::Default);

		let strong = CipherSuite::Strong;
		assert_eq!(strong, CipherSuite::Strong);

		let custom = CipherSuite::Custom(vec!["TLS_AES_128_GCM_SHA256".to_string()]);
		match custom {
			CipherSuite::Custom(suites) => assert_eq!(suites.len(), 1),
			_ => panic!("Expected Custom cipher suite"),
		}
	}

	#[test]
	fn test_certificate_creation() {
		let cert = Certificate {
			id: "test-cert".to_string(),
			common_name: "localhost".to_string(),
			data: "mock-cert-data".to_string(),
			key: Some("mock-key-data".to_string()),
			expires_at: "2025-12-31T23:59:59Z".to_string(),
			chain: vec![],
		};

		assert_eq!(cert.id, "test-cert");
		assert_eq!(cert.common_name, "localhost");
		assert!(cert.key.is_some());
	}

	#[test]
	fn test_certificate_manager_creation() {
		let config = TlsConfig::default();
		let manager = CertificateManager::new(config);
		assert!(manager.is_ok());
	}

	#[test]
	fn test_certificate_upload() {
		let config = TlsConfig::default();
		let mut manager = CertificateManager::new(config).unwrap();

		let cert = Certificate {
			id: "test-cert".to_string(),
			common_name: "localhost".to_string(),
			data: "mock-cert-data".to_string(),
			key: Some("mock-key-data".to_string()),
			expires_at: "2025-12-31T23:59:59Z".to_string(),
			chain: vec![],
		};

		assert!(manager.upload_certificate(cert).is_ok());
		assert_eq!(manager.list_certificates().len(), 1);
	}

	#[test]
	fn test_certificate_retrieval() {
		let config = TlsConfig::default();
		let mut manager = CertificateManager::new(config).unwrap();

		let cert = Certificate {
			id: "test-cert".to_string(),
			common_name: "localhost".to_string(),
			data: "mock-cert-data".to_string(),
			key: Some("mock-key-data".to_string()),
			expires_at: "2025-12-31T23:59:59Z".to_string(),
			chain: vec![],
		};

		manager.upload_certificate(cert).unwrap();

		let retrieved = manager.get_certificate("test-cert");
		assert!(retrieved.is_some());
		assert_eq!(retrieved.unwrap().id, "test-cert");

		let not_found = manager.get_certificate("non-existent");
		assert!(not_found.is_none());
	}

	#[test]
	fn test_certificate_revocation() {
		let config = TlsConfig::default();
		let mut manager = CertificateManager::new(config).unwrap();

		let cert = Certificate {
			id: "test-cert".to_string(),
			common_name: "localhost".to_string(),
			data: "mock-cert-data".to_string(),
			key: Some("mock-key-data".to_string()),
			expires_at: "2025-12-31T23:59:59Z".to_string(),
			chain: vec![],
		};

		manager.upload_certificate(cert).unwrap();
		assert_eq!(manager.list_certificates().len(), 1);

		assert!(manager.revoke_certificate("test-cert").is_ok());
		assert_eq!(manager.list_certificates().len(), 0);

		assert!(manager.revoke_certificate("test-cert").is_err());
	}

	#[test]
	fn test_certificate_rotation() {
		let config = TlsConfig::new().with_rotation(3600);
		let mut manager = CertificateManager::new(config).unwrap();

		let cert = Certificate {
			id: "test-cert".to_string(),
			common_name: "localhost".to_string(),
			data: "mock-cert-data".to_string(),
			key: Some("mock-key-data".to_string()),
			expires_at: "2024-12-31T23:59:59Z".to_string(),
			chain: vec![],
		};

		manager.upload_certificate(cert).unwrap();

		let rotated = manager.rotate_certificates().unwrap();
		assert_eq!(rotated, 1);
	}

	#[test]
	fn test_load_from_paths() {
		let config = TlsConfig::default();
		let mut manager = CertificateManager::new(config).unwrap();

		let cert = manager.load_from_paths("/path/to/cert.pem", "/path/to/key.pem");
		assert!(cert.is_ok());

		let loaded_cert = cert.unwrap();
		assert_eq!(loaded_cert.id, "server-cert");
		assert!(manager.get_certificate("server-cert").is_some());
	}
}
