/// Configuration management for EpilogLite
///
/// Supports JSON configuration files and environment variables with defaults

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
	/// Default page size in bytes (must be power of 2 between 512 and 65536)
	pub page_size: usize,
	
	/// Page cache size in number of pages
	pub cache_size: usize,
	
	/// Enable Write-Ahead Logging
	pub enable_wal: bool,
	
	/// Default text encoding
	pub text_encoding: String,
	
	/// Enable auto-vacuum
	pub auto_vacuum: bool,
}

impl Default for DatabaseConfig {
	fn default() -> Self {
		DatabaseConfig {
			page_size: 4096,
			cache_size: 100,
			enable_wal: false,
			text_encoding: "UTF-8".to_string(),
			auto_vacuum: false,
		}
	}
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
	/// Log level: trace, debug, info, warn, error
	pub level: String,
	
	/// Enable logging to stdout
	pub stdout: bool,
	
	/// Enable logging to stderr
	pub stderr: bool,
	
	/// Log file path (if any)
	pub file: Option<String>,
	
	/// Enable syslog
	pub syslog: bool,
	
	/// Enable colored output
	pub colored: bool,
}

impl Default for LogConfig {
	fn default() -> Self {
		LogConfig {
			level: "info".to_string(),
			stdout: true,
			stderr: false,
			file: None,
			syslog: false,
			colored: true,
		}
	}
}

/// Complete EpilogLite configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpilogLiteConfig {
	pub database: DatabaseConfig,
	pub logging: LogConfig,
}

impl Default for EpilogLiteConfig {
	fn default() -> Self {
		EpilogLiteConfig {
			database: DatabaseConfig::default(),
			logging: LogConfig::default(),
		}
	}
}

impl EpilogLiteConfig {
	/// Load configuration from a JSON file with environment variable overrides
	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
		let config = Config::builder()
			// Start with defaults
			.set_default("database.page_size", 4096)?
			.set_default("database.cache_size", 100)?
			.set_default("database.enable_wal", false)?
			.set_default("database.text_encoding", "UTF-8")?
			.set_default("database.auto_vacuum", false)?
			.set_default("logging.level", "info")?
			.set_default("logging.stdout", true)?
			.set_default("logging.stderr", false)?
			.set_default("logging.syslog", false)?
			.set_default("logging.colored", true)?
			// Load from file
			.add_source(File::from(path.as_ref()))
			// Override with environment variables (e.g., EPILOGLITE_DATABASE_PAGE_SIZE)
			.add_source(Environment::with_prefix("EPILOGLITE").separator("_"))
			.build()?;
		
		config.try_deserialize()
	}
	
	/// Create configuration from environment variables only
	pub fn from_env() -> Result<Self, ConfigError> {
		let config = Config::builder()
			// Start with defaults
			.set_default("database.page_size", 4096)?
			.set_default("database.cache_size", 100)?
			.set_default("database.enable_wal", false)?
			.set_default("database.text_encoding", "UTF-8")?
			.set_default("database.auto_vacuum", false)?
			.set_default("logging.level", "info")?
			.set_default("logging.stdout", true)?
			.set_default("logging.stderr", false)?
			.set_default("logging.syslog", false)?
			.set_default("logging.colored", true)?
			// Override with environment variables
			.add_source(Environment::with_prefix("EPILOGLITE").separator("_"))
			.build()?;
		
		config.try_deserialize()
	}
	
	/// Validate the configuration
	pub fn validate(&self) -> Result<(), String> {
		// Validate page size is power of 2 between 512 and 65536
		let ps = self.database.page_size;
		if ps < 512 || ps > 65536 || !ps.is_power_of_two() {
			return Err(format!(
				"Invalid page_size: {}. Must be power of 2 between 512 and 65536",
				ps
			));
		}
		
		// Validate cache size is reasonable
		if self.database.cache_size == 0 {
			return Err("cache_size must be greater than 0".to_string());
		}
		
		// Validate log level
		let valid_levels = ["trace", "debug", "info", "warn", "error"];
		if !valid_levels.contains(&self.logging.level.to_lowercase().as_str()) {
			return Err(format!(
				"Invalid log level: {}. Must be one of: trace, debug, info, warn, error",
				self.logging.level
			));
		}
		
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_default_config() {
		let config = EpilogLiteConfig::default();
		assert_eq!(config.database.page_size, 4096);
		assert_eq!(config.database.cache_size, 100);
		assert!(!config.database.enable_wal);
		assert_eq!(config.logging.level, "info");
		assert!(config.logging.stdout);
	}
	
	#[test]
	fn test_validate_config() {
		let mut config = EpilogLiteConfig::default();
		assert!(config.validate().is_ok());
		
		// Invalid page size
		config.database.page_size = 1000; // Not power of 2
		assert!(config.validate().is_err());
		
		config.database.page_size = 4096;
		assert!(config.validate().is_ok());
		
		// Invalid cache size
		config.database.cache_size = 0;
		assert!(config.validate().is_err());
	}
}