/// Example demonstrating async I/O operations in EpilogLite
///
/// This example shows how to use the async file and VFS traits for
/// non-blocking database operations.

#[cfg(feature = "async")]
use epiloglite::async_file::AsyncDefaultFile;
#[cfg(feature = "async")]
use epiloglite::async_vfs::{AsyncDefaultVfs, AsyncVirtualFileSystem};
#[cfg(feature = "async")]
use epiloglite::backpressure::BackpressureController;
#[cfg(feature = "async")]
use epiloglite::performance::{compare_read_performance, compare_write_performance};
#[cfg(feature = "async")]
use epiloglite::{AccessFlags, AsyncFile, OpenFlags, SynchronizationType};
#[cfg(feature = "async")]
use flagset::FlagSet;
#[cfg(feature = "async")]
use std::path::Path;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> std::io::Result<()> {
	println!("=== EpilogLite Async I/O Example ===\n");

	// Example 1: Basic async file operations
	println!("1. Basic Async File Operations");
	basic_async_file_example().await?;

	// Example 2: Using async VFS
	println!("\n2. Async VFS Operations");
	async_vfs_example().await?;

	// Example 3: Concurrent operations with backpressure
	println!("\n3. Concurrent Operations with Backpressure");
	backpressure_example().await?;

	// Example 4: Performance comparison
	println!("\n4. Performance Comparison: Sync vs Async");
	performance_comparison_example().await?;

	println!("\n=== All examples completed successfully ===");

	Ok(())
}

#[cfg(feature = "async")]
async fn basic_async_file_example() -> std::io::Result<()> {
	let path = "/tmp/epiloglite_async_example.db";

	// Clean up any existing file
	let _ = tokio::fs::remove_file(path).await;

	// Open file asynchronously
	let mut file = AsyncDefaultFile::open(path, true, true, true).await?;
	println!("  ✓ Opened file asynchronously: {}", path);

	// Write data asynchronously
	let data = b"Hello from async EpilogLite!";
	file.write(data, 0).await?;
	println!("  ✓ Wrote {} bytes asynchronously", data.len());

	// Sync to disk asynchronously
	file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull))
		.await?;
	println!("  ✓ Synced to disk asynchronously");

	// Get file size asynchronously
	let size = file.file_size().await?;
	println!("  ✓ File size: {} bytes", size);

	// Read data asynchronously
	let read_data = file.read(0).await?;
	println!("  ✓ Read {} bytes asynchronously", read_data.len());
	println!("  ✓ Content: {}", String::from_utf8_lossy(&read_data));

	// Clean up
	drop(file);
	tokio::fs::remove_file(path).await?;
	println!("  ✓ Cleaned up test file");

	Ok(())
}

#[cfg(feature = "async")]
async fn async_vfs_example() -> std::io::Result<()> {
	let vfs = AsyncDefaultVfs::new();
	let path = Path::new("/tmp/epiloglite_vfs_example.db");

	// Clean up
	let _ = tokio::fs::remove_file(path).await;

	// Check if file exists
	let exists = vfs.access(path, AccessFlags::exists()).await.unwrap();
	println!("  ✓ File exists: {}", exists);

	// Open file via VFS
	let mut file = vfs.open(path, OpenFlags::create()).await.unwrap();
	println!("  ✓ Opened file via async VFS");

	// Write some data
	let data = b"VFS async operations work!";
	file.write(data, 0).await?;
	println!("  ✓ Wrote data via VFS");

	// Check file exists now
	let exists = vfs.access(path, AccessFlags::exists()).await.unwrap();
	println!("  ✓ File exists after write: {}", exists);

	// Get current time
	let time = vfs.current_time().await.unwrap();
	println!("  ✓ Current time: {}", time);

	// Generate random bytes
	let mut buffer = [0u8; 16];
	vfs.randomness(&mut buffer).await.unwrap();
	println!("  ✓ Generated {} random bytes", buffer.len());

	// Clean up
	drop(file);
	vfs.delete(path, false).await.unwrap();
	println!("  ✓ Deleted file via VFS");

	Ok(())
}

#[cfg(feature = "async")]
async fn backpressure_example() -> std::io::Result<()> {
	let controller = BackpressureController::new(3);
	println!(
		"  ✓ Created backpressure controller (max {} concurrent ops)",
		controller.max_concurrent()
	);

	let mut tasks = Vec::new();

	// Spawn 10 concurrent file operations, but only 3 can run at once
	for i in 0..10 {
		let controller_clone = controller.clone();
		let task = tokio::spawn(async move {
			// Acquire permit - this will block if 3 operations are already running
			let _permit = controller_clone.acquire().await;

			let path = format!("/tmp/epiloglite_bp_test_{}.db", i);
			let mut file = AsyncDefaultFile::open(&path, true, true, true).await.unwrap();
			let data = format!("File {} data", i);
			file.write(data.as_bytes(), 0).await.unwrap();
			drop(file);
			tokio::fs::remove_file(&path).await.unwrap();

			i
		});
		tasks.push(task);
	}

	// Wait for all tasks to complete
	let results = futures::future::join_all(tasks).await;
	println!(
		"  ✓ Completed {} concurrent file operations with backpressure",
		results.len()
	);

	Ok(())
}

#[cfg(feature = "async")]
async fn performance_comparison_example() -> std::io::Result<()> {
	println!("  Running performance benchmarks...");

	// Compare write performance
	let write_result = compare_write_performance(50, 4096).await?;
	println!("\n  Write Performance:");
	println!("  {}", write_result.summary());

	// Compare read performance
	let read_result = compare_read_performance(50, 4096).await?;
	println!("\n  Read Performance:");
	println!("  {}", read_result.summary());

	Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
	println!("This example requires the 'async' feature to be enabled.");
	println!("Run with: cargo run --example async_io --features async");
}
