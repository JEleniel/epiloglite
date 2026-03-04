//! Core types and utilities for the EpilogLite database engine.
//!
//! This crate contains the small, focused core types used throughout the
//! EpilogLite project: compact integer encodings, on-disk pointers, record
//! traits/flags, containers, and light-weight serialization utilities. It is
//! intended to be minimal and dependency-light so other crates in the
//! workspace can re-use these building blocks.
//!
//! Publicly exported items live under the `epiloglite` module and are
//! re-exported from the crate root for convenient consumption (for example
//! consumers may use `epiloglite_core::CInt` or `epiloglite_core::Record`).
mod epiloglite;

pub use epiloglite::*;
