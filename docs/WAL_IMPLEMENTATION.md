# Write-Ahead Logging (WAL) Implementation

## Overview

EpilogLite now supports Write-Ahead Logging (WAL) mode, which provides improved concurrency, crash recovery, and durability compared to the traditional rollback journal mode.

## Features

### Implemented

- ✅ **WAL File Format**: Complete implementation of the WAL file format with headers and frames
- ✅ **WAL Writer**: Supports writing transactions to WAL with proper checksumming
- ✅ **WAL Reader**: Concurrent reader support with snapshot isolation
- ✅ **Checksum Algorithm**: Fibonacci-weighted checksum for data integrity
- ✅ **Checkpoint Mechanism**: Transfers WAL changes back to the main database
- ✅ **Recovery**: Automatic recovery from WAL on database open
- ✅ **Transaction Support**: Begin, commit, and rollback transactions
- ✅ **Pager Integration**: WAL mode integrated into the page cache system
- ✅ **Database API**: Simple API for using WAL mode

### Benefits

1. **Improved Concurrency**: Multiple readers can access the database while a write is in progress
2. **Better Performance**: Writes are faster as they only need to append to the WAL file
3. **Crash Recovery**: Database can recover to a consistent state after crashes
4. **Atomic Commits**: All-or-nothing transaction semantics

## Usage

### Opening a Database in WAL Mode

```rust
use epiloglite::Database;

// Open database with WAL mode
let mut db = Database::open_with_wal("mydb.db").unwrap();
```

### Using Transactions

```rust
// Begin a transaction
db.begin_transaction().unwrap();

// Perform operations
db.execute("INSERT INTO users VALUES (1, 'Alice')").unwrap();
db.execute("INSERT INTO users VALUES (2, 'Bob')").unwrap();

// Commit the transaction
db.commit_transaction().unwrap();
```

### Performing Checkpoints

```rust
use epiloglite::eplite::persistence::wal::CheckpointMode;

// Perform a full checkpoint
db.checkpoint(CheckpointMode::Full).unwrap();
```

## Architecture

### Components

1. **WalHeader** (32 bytes):
   - Magic number (4 bytes): 0x377f0682 or 0x377f0683
   - File format version (4 bytes): Currently 3007000
   - Database page size (4 bytes)
   - Checkpoint sequence number (4 bytes)
   - Salt-1 and Salt-2 (8 bytes): Random values for frame validation
   - Checksums (8 bytes): Header integrity verification

2. **WalFrame** (24-byte header + page data):
   - Page number (4 bytes)
   - Database size after commit (4 bytes): 0 for non-commit frames
   - Salt values (8 bytes): Copied from WAL header
   - Checksums (8 bytes): Cumulative checksum including all previous frames

3. **WalWriter**:
   - Manages WAL file writing
   - Maintains cumulative checksums
   - Supports frame addition and transaction commits
   - Can reset for new transactions

4. **WalReader**:
   - Provides concurrent read access
   - Implements snapshot isolation
   - Returns latest committed version of pages
   - Validates frame integrity

5. **WalCheckpoint**:
   - Transfers committed frames to main database
   - Supports multiple checkpoint modes (Passive, Full, Restart, Truncate)
   - Handles page consolidation (multiple updates to same page)

6. **WalRecovery**:
   - Recovers database state from WAL after crashes
   - Validates all frames before applying
   - Returns consolidated page updates

### File Layout

```
database.db      - Main database file
database.db-wal  - Write-ahead log file
```

The WAL file structure:
```
[WAL Header: 32 bytes]
[Frame 1 Header: 24 bytes][Frame 1 Data: page_size bytes]
[Frame 2 Header: 24 bytes][Frame 2 Data: page_size bytes]
...
[Frame N Header: 24 bytes][Frame N Data: page_size bytes]
```

## Checksum Algorithm

The WAL uses a Fibonacci-weighted checksum algorithm:

```rust
fn compute_checksum(data: &[u8], s0: u32, s1: u32, big_endian: bool) -> (u32, u32) {
    let mut sum0 = s0;
    let mut sum1 = s1;
    
    // Process data in 8-byte chunks
    for chunk in data.chunks_exact(8) {
        let x0 = read_u32(chunk, big_endian);
        let x1 = read_u32(&chunk[4..], big_endian);
        
        sum0 = sum0.wrapping_add(x0).wrapping_add(sum1);
        sum1 = sum1.wrapping_add(x1).wrapping_add(sum0);
    }
    
    (sum0, sum1)
}
```

This provides strong integrity guarantees while being efficient to compute.

## Checkpoint Modes

1. **Passive**: Checkpoint what can be done without blocking readers
2. **Full**: Complete checkpoint even if readers are active
3. **Restart**: Checkpoint and restart the WAL with new salts
4. **Truncate**: Checkpoint and truncate the WAL file

## Concurrent Access

### Readers

Multiple readers can access the database concurrently:
- Each reader sees a consistent snapshot from when it started
- Readers are not blocked by writers
- Readers see the database state up to the last committed transaction

### Writers

Currently supports single writer:
- Only one write transaction at a time
- Writers append frames to the WAL
- Commit marks the last frame as a commit point

## Performance Characteristics

### Advantages of WAL Mode

1. **Write Performance**: 
   - Append-only writes to WAL (faster than random writes)
   - No need to update main database on every transaction

2. **Read Performance**:
   - Readers not blocked during writes
   - Sequential reads from WAL for recently modified pages

3. **Durability**:
   - Changes are fsynced to WAL immediately
   - Main database updated during checkpoint

### Considerations

1. **WAL File Growth**: WAL grows until checkpoint
2. **Checkpoint Overhead**: Periodic checkpoints needed to transfer to main DB
3. **Memory Usage**: Page cache holds both main DB and WAL pages

## Testing

The implementation includes comprehensive tests:

- **Unit Tests** (21 tests in `src/eplite/persistence/wal.rs`):
  - Header and frame serialization
  - Checksum algorithm
  - Writer and reader operations
  - Checkpoint and recovery

- **Integration Tests** (9 tests in `tests/wal_integration_test.rs`):
  - Write-read cycles
  - Multiple transactions
  - Page versioning
  - Concurrent readers
  - Pager integration
  - Checkpoint scenarios
  - Recovery scenarios
  - Large transactions

- **Database Tests** (6 tests in `src/eplite/database.rs`):
  - WAL mode initialization
  - Transaction lifecycle
  - Checkpoint operations

## Limitations and Future Work

### Current Limitations

1. **Single Writer**: Only one write transaction at a time
2. **No Shared Memory**: Each process has its own WAL reader
3. **Manual Checkpoints**: Application must call checkpoint explicitly
4. **In-Memory Rollback**: Full rollback support requires additional state management

### Future Enhancements

1. **Automatic Checkpointing**: Background checkpoint thread
2. **WAL-Index**: Shared memory structure for faster page lookups
3. **Multiple Writers**: Support for concurrent write transactions
4. **Better Rollback**: Enhanced transaction state tracking
5. **Metrics**: Instrumentation for WAL size, checkpoint frequency, etc.

## Examples

### Basic WAL Usage

```rust
use epiloglite::Database;

fn main() -> epiloglite::Result<()> {
    // Open database with WAL
    let mut db = Database::open_with_wal("example.db")?;
    
    // Create table
    db.execute("CREATE TABLE items (id INTEGER, name TEXT)")?;
    
    // Insert data in transaction
    db.begin_transaction()?;
    db.execute("INSERT INTO items VALUES (1, 'Item 1')")?;
    db.execute("INSERT INTO items VALUES (2, 'Item 2')")?;
    db.commit_transaction()?;
    
    // Query data
    let result = db.execute("SELECT * FROM items")?;
    println!("{:?}", result);
    
    // Checkpoint to transfer WAL to main DB
    use epiloglite::eplite::persistence::wal::CheckpointMode;
    db.checkpoint(CheckpointMode::Full)?;
    
    Ok(())
}
```

### Concurrent Readers

```rust
use epiloglite::eplite::persistence::wal::{WalWriter, WalReader, WalFrame};

// Writer creates WAL
let mut writer = WalWriter::new(4096);

// Add frames
let frame = WalFrame::new(1, vec![0u8; 4096], 0, 0);
writer.add_frame(frame)?;
writer.commit(1)?;

let wal_bytes = writer.to_bytes();

// Multiple readers can access
let reader1 = WalReader::from_bytes(&wal_bytes)?;
let reader2 = WalReader::from_bytes(&wal_bytes)?;

// Both see same snapshot
let page1 = reader1.get_page(1);
let page2 = reader2.get_page(1);
assert_eq!(page1, page2);
```

## References

- [SQLite WAL Documentation](https://www.sqlite.org/wal.html)
- [EpilogLite File Format](FILEFORMAT.md)
- [Transaction Handling](TRANSACTIONS.md)

## Version History

- **v0.1.0**: Initial WAL implementation
  - Basic WAL file format
  - Single writer support
  - Concurrent reader support
  - Checkpoint mechanism
  - Recovery support
