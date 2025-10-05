# Embedded Examples for EpilogLite

This directory contains examples demonstrating EpilogLite usage in embedded systems without the standard library.

## Examples

### `no_std_basic.rs`

A minimal example showing basic database operations in a no-std environment.

**Features:**
- In-memory database creation
- Table creation and data insertion
- Simple queries
- Custom allocator setup

**Target:** Any no-std compatible platform

## Building

### Simulation/Hosted

To build examples for your host system (for testing):

```bash
cargo build --example no_std_basic --no-default-features --features no-std
```

## Requirements

### Minimum

- Rust 1.70+ with `no_std` support
- Global allocator implementation
- ~100KB flash for code
- ~50KB RAM for heap

### Recommended

- 256KB+ flash
- 128KB+ RAM
- Hardware with atomic operations

## Memory Layout

Typical memory usage:

```
┌─────────────────┐
│   Stack (~8KB)  │
├─────────────────┤
│   Heap (50KB+)  │
├─────────────────┤
│   Static (1KB)  │
├─────────────────┤
│   Code (100KB+) │
└─────────────────┘
```

## Support

For embedded-specific issues:

- Check `docs/NO_STD.md` for detailed documentation
- Open an issue with platform details
- Include memory layout and constraints
