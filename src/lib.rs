///! # EpilogLite Main Library
///!
///! This crate provides the main API surface for EpilogLite, a pure Rust database engine inspired by SQLite.
///! It re-exports core types, derive macros, and main modules for unified and ergonomic usage.
pub use epiloglite_core::semver::{SemVer, SemVerError};
/// # EpilogLite Main Library
///
/// This crate provides the main API surface for EpilogLite, a pure Rust database engine inspired by SQLite.
/// It re-exports core types, derive macros, and main modules for unified and ergonomic usage.

/// Constants used throughout the library
pub mod constants;
/// Main EpilogLite engine and database logic
pub mod eplite;
/// OS abstraction and virtual file system modules
pub mod os;

/// Re-export core crate for unified API
pub use epiloglite_core::*;
/// Re-export derive macros for model and record definitions
pub use epiloglite_derive::*;
