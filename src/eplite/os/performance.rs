/// Performance comparison utilities for sync vs async I/O operations
///
/// This module provides benchmarking utilities to compare the performance
/// of synchronous and asynchronous file operations.

#[cfg(all(feature = "async", feature = "std"))]
use std::time::{Duration, Instant};

#[cfg(all(feature = "async", feature = "std"))]
use super::async_file::AsyncDefaultFile;
#[cfg(all(feature = "async", feature = "std"))]
use super::file::DefaultFile;
#[cfg(feature = "async")]
use crate::eplite::traits::async_file::AsyncFile;
#[cfg(feature = "std")]
use crate::eplite::traits::file::{File, SynchronizationType};
#[cfg(all(feature = "async", feature = "std"))]
use flagset::FlagSet;

/// Performance comparison result
#[cfg(all(feature = "async", feature = "std"))]
#[derive(Debug, Clone)]
pub struct PerformanceComparison {
	pub sync_duration: Duration,
	pub async_duration: Duration,
	pub async_speedup: f64,
}

impl PerformanceComparison {
	/// Get a human-readable summary
	pub fn summary(&self) -> String {
		format!(
			"Sync: {:?}, Async: {:?}, Speedup: {:.2}x",
			self.sync_duration, self.async_duration, self.async_speedup
		)
	}
}

/// Benchmark result for a series of operations
#[cfg(all(feature = "async", feature = "std"))]
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
	pub operation: String,
	pub iterations: usize,
	pub total_bytes: u64,
	pub comparison: PerformanceComparison,
}

impl BenchmarkResult {
	/// Get throughput in MB/s for sync operations
	pub fn sync_throughput_mbps(&self) -> f64 {
		let mb = self.total_bytes as f64 / 1_048_576.0;
		mb / self.comparison.sync_duration.as_secs_f64()
	}

	/// Get throughput in MB/s for async operations
	pub fn async_throughput_mbps(&self) -> f64 {
		let mb = self.total_bytes as f64 / 1_048_576.0;
		mb / self.comparison.async_duration.as_secs_f64()
	}

	/// Get a human-readable summary
	pub fn summary(&self) -> String {
		format!(
			"{} ({} iterations, {} bytes): {} | Throughput: Sync {:.2} MB/s, Async {:.2} MB/s",
			self.operation,
			self.iterations,
			self.total_bytes,
			self.comparison.summary(),
			self.sync_throughput_mbps(),
			self.async_throughput_mbps()
		)
	}
}

/// Compare sync vs async write performance
#[cfg(all(feature = "async", feature = "std"))]
pub async fn compare_write_performance(
	iterations: usize,
	data_size: usize,
) -> std::io::Result<BenchmarkResult> {
	let data = vec![0u8; data_size];
	let sync_path = "/tmp/epiloglite_perf_sync.db";
	let async_path = "/tmp/epiloglite_perf_async.db";

	// Clean up
	let _ = std::fs::remove_file(sync_path);
	let _ = tokio::fs::remove_file(async_path).await;

	// Benchmark sync
	let sync_start = Instant::now();
	{
		let mut file = DefaultFile::open(sync_path, true, true, true)?;
		for i in 0..iterations {
			file.write(&data, (i * data_size) as u64)?;
		}
		file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull))?;
	}
	let sync_duration = sync_start.elapsed();

	// Benchmark async
	let async_start = Instant::now();
	{
		let mut file = AsyncDefaultFile::open(async_path, true, true, true).await?;
		for i in 0..iterations {
			file.write(&data, (i * data_size) as u64).await?;
		}
		file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull))
			.await?;
	}
	let async_duration = async_start.elapsed();

	// Clean up
	let _ = std::fs::remove_file(sync_path);
	let _ = tokio::fs::remove_file(async_path).await;

	Ok(BenchmarkResult {
		operation: "write".to_string(),
		iterations,
		total_bytes: (iterations * data_size) as u64,
		comparison: PerformanceComparison {
			sync_duration,
			async_duration,
			async_speedup: sync_duration.as_secs_f64() / async_duration.as_secs_f64(),
		},
	})
}

/// Compare sync vs async read performance
#[cfg(all(feature = "async", feature = "std"))]
pub async fn compare_read_performance(
	iterations: usize,
	data_size: usize,
) -> std::io::Result<BenchmarkResult> {
	let data = vec![42u8; data_size];
	let sync_path = "/tmp/epiloglite_perf_sync_read.db";
	let async_path = "/tmp/epiloglite_perf_async_read.db";

	// Clean up and prepare files
	let _ = std::fs::remove_file(sync_path);
	let _ = tokio::fs::remove_file(async_path).await;

	// Create test files with data
	{
		let mut file = DefaultFile::open(sync_path, true, true, true)?;
		for i in 0..iterations {
			file.write(&data, (i * data_size) as u64)?;
		}
	}

	{
		let mut file = AsyncDefaultFile::open(async_path, true, true, true).await?;
		for i in 0..iterations {
			file.write(&data, (i * data_size) as u64).await?;
		}
	}

	// Benchmark sync reads
	let sync_start = Instant::now();
	{
		let mut file = DefaultFile::open(sync_path, true, false, false)?;
		for i in 0..iterations {
			let _ = file.read((i * data_size) as u64)?;
		}
	}
	let sync_duration = sync_start.elapsed();

	// Benchmark async reads
	let async_start = Instant::now();
	{
		let mut file = AsyncDefaultFile::open(async_path, true, false, false).await?;
		for i in 0..iterations {
			let _ = file.read((i * data_size) as u64).await?;
		}
	}
	let async_duration = async_start.elapsed();

	// Clean up
	let _ = std::fs::remove_file(sync_path);
	let _ = tokio::fs::remove_file(async_path).await;

	Ok(BenchmarkResult {
		operation: "read".to_string(),
		iterations,
		total_bytes: (iterations * data_size) as u64,
		comparison: PerformanceComparison {
			sync_duration,
			async_duration,
			async_speedup: sync_duration.as_secs_f64() / async_duration.as_secs_f64(),
		},
	})
}

#[cfg(all(test, feature = "async"))]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_write_performance_comparison() {
		let result = compare_write_performance(10, 1024).await;
		assert!(result.is_ok());
		let result = result.unwrap();
		assert_eq!(result.iterations, 10);
		assert_eq!(result.total_bytes, 10240);
		assert!(result.sync_throughput_mbps() > 0.0);
		assert!(result.async_throughput_mbps() > 0.0);
	}

	#[tokio::test]
	async fn test_read_performance_comparison() {
		let result = compare_read_performance(10, 1024).await;
		assert!(result.is_ok());
		let result = result.unwrap();
		assert_eq!(result.iterations, 10);
		assert_eq!(result.total_bytes, 10240);
		assert!(result.sync_throughput_mbps() > 0.0);
		assert!(result.async_throughput_mbps() > 0.0);
	}

	#[test]
	fn test_performance_comparison_summary() {
		let comparison = PerformanceComparison {
			sync_duration: Duration::from_millis(100),
			async_duration: Duration::from_millis(50),
			async_speedup: 2.0,
		};
		let summary = comparison.summary();
		assert!(summary.contains("Sync"));
		assert!(summary.contains("Async"));
		assert!(summary.contains("2.00x"));
	}

	#[test]
	fn test_benchmark_result_summary() {
		let result = BenchmarkResult {
			operation: "test".to_string(),
			iterations: 10,
			total_bytes: 1024,
			comparison: PerformanceComparison {
				sync_duration: Duration::from_millis(100),
				async_duration: Duration::from_millis(50),
				async_speedup: 2.0,
			},
		};
		let summary = result.summary();
		assert!(summary.contains("test"));
		assert!(summary.contains("10 iterations"));
	}
}
