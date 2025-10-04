# No-std Support in EpilogLite

EpilogLite can be compiled without the Rust standard library (`no_std`), making it suitable for embedded systems and resource-constrained environments.

## Features

When built with `no-std`:

- ✅ Core database functionality (in-memory only)
- ✅ SQL parsing and tokenization
- ✅ Query builder API
- ✅ Type system and error handling
- ✅ B-tree data structures
- ✅ Index support
- ❌ File I/O (requires `std`)
- ❌ Configuration system (requires `std`)
- ❌ Logging (requires `std`)
- ❌ Server mode (requires `std`)

## Building

### No-std Build

To build EpilogLite without the standard library:

```bash
cargo build --no-default-features --features no-std
```

### With std (Default)

To build with standard library support:

```bash
cargo build
# or explicitly
cargo build --features std
```

## Usage

### Basic Example (No-std)

```rust
#![no_std]

extern crate alloc;

use alloc::string::String;
use epiloglite::Database;

fn main() -> Result<(), epiloglite::Error> {
    // Create an in-memory database
    let mut db = Database::new()?;
    
    // Execute SQL statements
    db.execute("CREATE TABLE users (id INTEGER, name TEXT)")?;
    db.execute("INSERT INTO users VALUES (1, 'Alice')")?;
    
    // Query data
    let result = db.execute("SELECT * FROM users")?;
    
    Ok(())
}
```

## Memory Requirements

Minimum memory requirements for no-std mode:

- **Flash/ROM**: ~100KB for code
- **RAM**: ~10KB minimum (for basic operations)
- **Heap**: Configurable, minimum ~50KB recommended

## Custom Allocator

EpilogLite requires a global allocator in no-std mode. You need to provide one:

```rust
#![no_std]

extern crate alloc;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn main() {
    // Initialize heap
    unsafe {
        HEAP.init(HEAP_START, HEAP_SIZE);
    }
    
    // Use EpilogLite...
}
```

## Limitations in No-std Mode

### No File I/O

Database persistence to disk is not available in no-std mode. Only in-memory databases are supported:

```rust
// This works in no-std
let mut db = Database::new()?;

// This requires std feature
// let mut db = Database::open("database.db")?;
```

### No Configuration System

The configuration module requires file I/O and is not available in no-std mode.

### No Logging

The logging system requires I/O capabilities and is not available in no-std mode. Consider using platform-specific logging mechanisms.

## Platform-Specific Considerations

### ARM Cortex-M

For ARM Cortex-M microcontrollers:

```toml
[dependencies]
epiloglite = { version = "*", default-features = false, features = ["no-std"] }
embedded-alloc = "0.5"
```

### RISC-V

For RISC-V platforms:

```toml
[dependencies]
epiloglite = { version = "*", default-features = false, features = ["no-std"] }
```

## Testing

To run tests in no-std mode:

```bash
cargo test --no-default-features --features no-std
```

Note: Some tests may be disabled in no-std mode due to file I/O requirements.

## Type Mappings

### Collections

In no-std mode, some standard library types are replaced:

- `HashMap` → `BTreeMap` (from `alloc`)
- Uses `alloc::vec::Vec`
- Uses `alloc::string::String`

### I/O Types

- `std::io::Error` → `i32` error code
- No `std::fs` operations
- No `std::path` operations

## Contributing

When contributing to EpilogLite's no-std support:

1. Always test both `std` and `no-std` builds
2. Use `#[cfg(feature = "std")]` for std-only code
3. Use `#[cfg(not(feature = "std"))]` for no-std alternatives
4. Import types conditionally:

```rust
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as HashMap;
```

## Future Work

Planned enhancements for no-std support:

- [ ] Custom VFS implementation for embedded flash
- [ ] Optimized memory allocator integration
- [ ] Platform-specific optimizations
- [ ] Extended embedded-hal integration
- [ ] WASM target support

## Resources

- [Rust Embedded Book](https://docs.rust-embedded.org/)
- [The Embedded Rust Book](https://rust-embedded.github.io/book/)
- [alloc documentation](https://doc.rust-lang.org/alloc/)
- [core documentation](https://doc.rust-lang.org/core/)
