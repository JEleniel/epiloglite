# Async I/O Implementation

## Overview

EpilogLite supports asynchronous I/O operations for non-blocking database access in modern, concurrent applications. The async I/O implementation is built on top of Tokio and provides parallel async traits alongside the existing synchronous traits.

## Architecture

### Design Principles

1. **Non-Breaking**: Async functionality is added alongside sync APIs, not replacing them
2. **Feature-Gated**: All async code is behind the `async` feature flag
3. **Safe**: 100% safe Rust with no unsafe code
4. **Compatible**: Works seamlessly with the Tokio async runtime

### Core Components

#### 1. AsyncFile Trait

Located in `src/eplite/traits/async_file.rs`, this trait provides async variants of all file operations:

```rust
#[async_trait]
pub trait AsyncFile: fmt::Debug + Send + Sync {
    async fn close(&mut self) -> Result<(), IoError>;
    async fn read(&mut self, offset: u64) -> Result<Vec<u8>, IoError>;
    async fn write(&mut self, data: &[u8], offset: u64) -> Result<(), IoError>;
    async fn truncate(&mut self, size: u64) -> Result<(), IoError>;
    async fn sync(&mut self, flags: FlagSet<SynchronizationType>) -> Result<(), IoError>;
    async fn file_size(&mut self) -> Result<u64, IoError>;
    async fn lock(&mut self, lock_type: LockType) -> Result<(), IoError>;
    async fn unlock(&mut self, unlock_type: UnlockType) -> Result<(), IoError>;
    async fn check_reserved_lock(&mut self) -> Result<u64, IoError>;
}
```

#### 2. AsyncVirtualFileSystem Trait

Located in `src/eplite/os/async_vfs.rs`, this trait provides async VFS operations:

```rust
#[async_trait]
pub trait AsyncVirtualFileSystem: Send + Sync {
    async fn open(&self, path: &Path, flags: OpenFlags) -> Result<Box<dyn AsyncFile>>;
    async fn delete(&self, path: &Path, sync_dir: bool) -> Result<()>;
    async fn access(&self, path: &Path, flags: AccessFlags) -> Result<bool>;
    async fn full_pathname(&self, path: &Path) -> Result<String>;
    async fn sleep(&self, microseconds: u64) -> Result<u64>;
    async fn current_time(&self) -> Result<i64>;
    async fn randomness(&self, buffer: &mut [u8]) -> Result<usize>;
}
```

#### 3. AsyncDefaultFile Implementation

The default implementation uses `tokio::fs::File` for async file operations:

- Non-blocking reads and writes
- Async seek operations
- Async sync to disk
- Metadata queries without blocking

#### 4. AsyncDefaultVfs Implementation

The default VFS implementation provides:

- Async file opening and deletion
- Non-blocking file system checks
- Async sleep operations
- Thread-safe random number generation using `tokio::task::spawn_blocking`

#### 5. Backpressure Control

Located in `src/eplite/os/backpressure.rs`, this utility prevents system overload:

```rust
pub struct BackpressureController {
    semaphore: Arc<Semaphore>,
    max_concurrent: usize,
}
```

Features:
- Limits concurrent I/O operations
- Automatic permit management via RAII
- Configurable concurrency limits
- Non-blocking try_acquire option

#### 6. Performance Benchmarking

Located in `src/eplite/os/performance.rs`, utilities for comparing sync vs async:

```rust
pub async fn compare_write_performance(
    iterations: usize,
    data_size: usize,
) -> std::io::Result<BenchmarkResult>

pub async fn compare_read_performance(
    iterations: usize,
    data_size: usize,
) -> std::io::Result<BenchmarkResult>
```

## Usage Examples

### Basic Async File Operations

```rust
use epiloglite::{AsyncFile, async_file::AsyncDefaultFile};
use epiloglite::SynchronizationType;
use flagset::FlagSet;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Open file asynchronously
    let mut file = AsyncDefaultFile::open("/tmp/test.db", true, true, true).await?;
    
    // Write data asynchronously
    let data = b"Hello, async world!";
    file.write(data, 0).await?;
    
    // Sync to disk
    file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull)).await?;
    
    // Read data back
    let read_data = file.read(0).await?;
    
    Ok(())
}
```

### Using Async VFS

```rust
use epiloglite::{async_vfs::{AsyncDefaultVfs, AsyncVirtualFileSystem}};
use epiloglite::{OpenFlags, AccessFlags};
use std::path::Path;

#[tokio::main]
async fn main() -> epiloglite::Result<()> {
    let vfs = AsyncDefaultVfs::new();
    let path = Path::new("/tmp/test.db");
    
    // Check if file exists
    let exists = vfs.access(path, AccessFlags::exists()).await?;
    
    // Open file via VFS
    let mut file = vfs.open(path, OpenFlags::create()).await?;
    
    // Delete file
    vfs.delete(path, false).await?;
    
    Ok(())
}
```

### Backpressure Control

```rust
use epiloglite::backpressure::BackpressureController;
use epiloglite::async_file::AsyncDefaultFile;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Limit to 10 concurrent file operations
    let controller = BackpressureController::new(10);
    
    let mut tasks = Vec::new();
    
    for i in 0..100 {
        let controller_clone = controller.clone();
        let task = tokio::spawn(async move {
            // Acquire permit - blocks if 10 operations are already running
            let _permit = controller_clone.acquire().await;
            
            // Perform I/O operation
            let path = format!("/tmp/file_{}.db", i);
            let mut file = AsyncDefaultFile::open(&path, true, true, true).await.unwrap();
            let data = b"data";
            file.write(data, 0).await.unwrap();
        });
        tasks.push(task);
    }
    
    // Wait for all to complete
    futures::future::join_all(tasks).await;
    
    Ok(())
}
```

### Performance Benchmarking

```rust
use epiloglite::performance::{compare_write_performance, compare_read_performance};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Compare write performance
    let write_result = compare_write_performance(100, 4096).await?;
    println!("{}", write_result.summary());
    
    // Compare read performance
    let read_result = compare_read_performance(100, 4096).await?;
    println!("{}", read_result.summary());
    
    Ok(())
}
```

## Configuration

### Enabling Async I/O

Add to your `Cargo.toml`:

```toml
[dependencies]
epiloglite = { version = "0.1", features = ["async"] }
tokio = { version = "1", features = ["full"] }
```

The `async` feature includes:
- `tokio` - Async runtime
- `rand` - Cryptographically secure random number generation
- `futures` - Future utilities
- `async-trait` - Async trait support

### Runtime Requirements

Async operations require a Tokio runtime. Use `#[tokio::main]` or create a runtime manually:

```rust
let runtime = tokio::runtime::Runtime::new()?;
runtime.block_on(async {
    // Async code here
});
```

## Performance Characteristics

### When to Use Async I/O

**Use async I/O when:**
- Building web servers or API services
- Handling many concurrent connections
- Need to avoid thread-per-connection models
- Working with cloud storage or networked filesystems
- Building reactive systems

**Use sync I/O when:**
- Building CLI tools or batch processors
- Single-threaded applications
- Simplicity is preferred over concurrency
- Working with local, fast filesystems

### Performance Considerations

1. **Overhead**: Async I/O has some overhead compared to sync for single operations
2. **Concurrency**: Async shines with many concurrent operations
3. **CPU vs I/O Bound**: Async is best for I/O-bound workloads
4. **Backpressure**: Use BackpressureController to prevent resource exhaustion

### Benchmark Results

From the example output:
- Async operations have overhead for simple sequential operations
- Performance varies based on filesystem and workload
- Concurrent workloads see better async performance
- Use benchmarking utilities to measure your specific use case

## Implementation Details

### Thread Safety

All async types are `Send + Sync`:
- `AsyncFile` requires `Send + Sync` for cross-task usage
- `AsyncVirtualFileSystem` requires `Send + Sync` for shared access
- `BackpressureController` is `Clone` and can be shared across tasks

### Cancellation Safety

All async operations are cancellation-safe:
- Dropping futures cancels in-flight operations
- File handles are properly closed on drop
- No resource leaks on cancellation

### Error Handling

Async functions use the same error types as sync:
- `std::io::Error` for I/O operations
- `epiloglite::Result<T>` for high-level operations
- Errors propagate naturally with `?` operator

## Testing

### Unit Tests

All async modules include comprehensive unit tests:

```bash
# Run all tests including async
cargo test --features async

# Run only async tests
cargo test --features async async
```

### Integration Testing

The example at `examples/async_io.rs` demonstrates:
- Basic async file operations
- Async VFS usage
- Backpressure control
- Performance comparisons

Run with:
```bash
cargo run --example async_io --features async
```

## Future Enhancements

Potential improvements for Phase 31+:

1. **Async Pager**: Make page cache operations async
2. **Async Transactions**: Full async transaction support
3. **Async Query Execution**: Non-blocking query processing
4. **Connection Pooling**: Async connection pool for multi-threaded access
5. **Streaming Results**: Stream-based result sets for large queries
6. **Async WAL**: Integrate with Write-Ahead Logging (Phase 30)

## Dependencies

Required crates for async functionality:
- `tokio` (1.x) - Async runtime
- `async-trait` (0.1) - Async trait support
- `rand` (0.8) - Random number generation
- `futures` (0.3) - Future utilities
- `flagset` (0.4) - Flag set support (shared with sync)

## See Also

- [VFS Architecture](ARCHITECTURE.md#vfs-layer)
- [File Format](FILEFORMAT.md)
- [API Documentation](../../README.md#async-support)
- [Examples](../../examples/async_io.rs)
