# Changelog

All notable changes to EpilogLite will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure and foundation
- Comprehensive error handling with Error and Result types
- Support for all Rust native types (Bool, I8-I128, U8-U128, F32/F64, etc.)
- Database file format constants and enums
- Database header parsing for both EPLite and SQLite 3 formats
- OS abstraction layer with file I/O operations
- Virtual File System (VFS) trait and default implementation
- Page cache management system
- B-tree structure definitions
- SQL tokenizer with basic keyword recognition
- SQL parser structure definitions
- Bytecode instruction definitions
- Virtual machine structure for bytecode execution
- Database connection API (open/close)
- Utility functions for string and type operations
- Comprehensive test suite (52 tests)
- Basic usage example
- Documentation (README, CONTRIBUTING, STATUS)

### Security
- `unsafe_code = "forbid"` enforced - no unsafe code allowed
- All code is safe Rust

## [0.1.0] - 2024

### Added
- Initial alpha release
- Core foundation complete
- Basic module structure
- Type system
- File I/O abstraction

### Notes
- SQL execution not yet implemented
- Database operations return NotSupported errors
- This is an alpha release for testing the foundation
