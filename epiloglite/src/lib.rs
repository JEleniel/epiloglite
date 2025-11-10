///! # EpilogLite Main Library
///!
///! This crate provides the main API surface for EpilogLite, a pure Rust database engine inspired by SQLite.
///! It re-exports core types, derive macros, and main modules for unified and ergonomic usage.
/// # EpilogLite Main Library
///
/// This crate provides the main API surface for EpilogLite, a pure Rust database engine inspired by SQLite.
/// It re-exports core types, derive macros, and main modules for unified and ergonomic usage.

/// Main EpilogLite engine and database logic
pub mod eplite;

/// Re-export core crate for unified API
pub use epiloglite_core::*;
