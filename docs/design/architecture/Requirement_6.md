# Platform Support and Portability

## Overview

EpilogLite must support multiple platforms including desktop, server, mobile, and embedded systems with no-std capability for resource-constrained environments.

## User Story

As a cross-platform developer, I need a database that works consistently across all major operating systems and resource profiles so that I can deploy the same codebase everywhere from embedded devices to cloud servers.

## Features

### 1. Operating System Support
- Linux (primary)
- Windows
- macOS
- BSD variants
- Mobile (Android, iOS via FFI)
- Virtual File System (VFS) abstraction

**Acceptance Criteria:**
- Core functionality works on all platforms
- Platform-specific code isolated in VFS layer
- CI/CD tests on Linux, Windows, macOS
- No platform-specific bugs
- File paths handled correctly per platform

### 2. No-std Support
- Core database without std library
- Custom allocator support
- Embedded-friendly memory footprint
- Feature-gated (no-std feature)
- Integration with embedded-hal

**Acceptance Criteria:**
- Compiles successfully with no_std
- Memory usage <100KB for core operations
- No heap allocations in critical paths (configurable)
- Works with custom allocators
- Documented embedded use cases

### 3. Cross-compilation
- Build for multiple targets
- ARM, x86, x86_64 support
- Big-endian and little-endian
- 32-bit and 64-bit architectures
- WASM target (planned)

**Acceptance Criteria:**
- Cross-compilation documented
- Tests run on multiple architectures
- Binary compatibility maintained
- No architecture-specific bugs
- File format architecture-independent

### 4. Unicode Support
- UTF-8 (complete)
- UTF-16 support for Windows APIs
- Unicode normalization
- Collation sequences
- Case-insensitive comparisons

**Acceptance Criteria:**
- All text operations handle UTF-8 correctly
- UTF-16 conversion for Windows APIs
- Unicode characters sort correctly
- Case-insensitive LIKE works with Unicode
- No Unicode-related panics

### 5. Resource Constraints
- Configurable memory limits
- Minimum memory footprint
- Graceful degradation
- Memory pressure handling
- Small binary size

**Acceptance Criteria:**
- Database works with <1MB RAM
- Binary size <500KB (stripped)
- Memory limits enforced
- Out-of-memory handled gracefully
- Performance acceptable with minimal resources
