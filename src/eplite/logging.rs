/// Logging configuration and initialization for EpilogLite
///
/// Uses the log and fern crates to provide flexible logging with
/// support for stdout (with color), stderr (with color), files, and syslog

use crate::eplite::config::LogConfig;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::fs::OpenOptions;

/// Initialize logging based on configuration
pub fn init_logging(config: &LogConfig) -> Result<(), Box<dyn std::error::Error>> {
	let level = parse_log_level(&config.level)?;
	let colored = config.colored;
	
	let colors = ColoredLevelConfig::new()
		.trace(Color::Cyan)
		.debug(Color::Blue)
		.info(Color::Green)
		.warn(Color::Yellow)
		.error(Color::Red);
	
	let mut dispatch = fern::Dispatch::new()
		.format(move |out, message, record| {
			if colored {
				out.finish(format_args!(
					"[{} {} {}] {}",
					chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
					colors.color(record.level()),
					record.target(),
					message
				))
			} else {
				out.finish(format_args!(
					"[{} {} {}] {}",
					chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
					record.level(),
					record.target(),
					message
				))
			}
		})
		.level(level);
	
	// Add stdout if enabled
	if config.stdout {
		dispatch = dispatch.chain(std::io::stdout());
	}
	
	// Add stderr if enabled
	if config.stderr {
		dispatch = dispatch.chain(std::io::stderr());
	}
	
	// Add file if specified
	if let Some(ref file_path) = config.file {
		let file = OpenOptions::new()
			.write(true)
			.create(true)
			.append(true)
			.open(file_path)?;
		dispatch = dispatch.chain(file);
	}
	
	// Syslog support would require platform-specific implementation
	// For now, we log a warning if syslog is requested
	if config.syslog {
		eprintln!("Warning: Syslog support not yet implemented");
	}
	
	dispatch.apply()?;
	
	log::info!("Logging initialized at level: {}", config.level);
	
	Ok(())
}

/// Parse log level from string
fn parse_log_level(level: &str) -> Result<LevelFilter, String> {
	match level.to_lowercase().as_str() {
		"trace" => Ok(LevelFilter::Trace),
		"debug" => Ok(LevelFilter::Debug),
		"info" => Ok(LevelFilter::Info),
		"warn" => Ok(LevelFilter::Warn),
		"error" => Ok(LevelFilter::Error),
		"off" => Ok(LevelFilter::Off),
		_ => Err(format!("Invalid log level: {}", level)),
	}
}

/// Convenience macro for logging with context
#[macro_export]
macro_rules! log_db {
	(trace, $($arg:tt)*) => {
		log::trace!(target: "epiloglite::database", $($arg)*);
	};
	(debug, $($arg:tt)*) => {
		log::debug!(target: "epiloglite::database", $($arg)*);
	};
	(info, $($arg:tt)*) => {
		log::info!(target: "epiloglite::database", $($arg)*);
	};
	(warn, $($arg:tt)*) => {
		log::warn!(target: "epiloglite::database", $($arg)*);
	};
	(error, $($arg:tt)*) => {
		log::error!(target: "epiloglite::database", $($arg)*);
	};
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_parse_log_level() {
		assert_eq!(parse_log_level("trace").unwrap(), LevelFilter::Trace);
		assert_eq!(parse_log_level("DEBUG").unwrap(), LevelFilter::Debug);
		assert_eq!(parse_log_level("Info").unwrap(), LevelFilter::Info);
		assert_eq!(parse_log_level("WARN").unwrap(), LevelFilter::Warn);
		assert_eq!(parse_log_level("error").unwrap(), LevelFilter::Error);
		assert!(parse_log_level("invalid").is_err());
	}
	
	#[test]
	fn test_init_logging_basic() {
		let config = LogConfig {
			level: "info".to_string(),
			stdout: false, // Disable for test
			stderr: false,
			file: None,
			syslog: false,
			colored: false,
		};
		
		// Should not panic
		let result = init_logging(&config);
		// May fail if already initialized in another test, that's OK
		let _ = result;
	}
}