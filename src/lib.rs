#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod constants;
pub mod eplite;
mod semver;

#[cfg(feature = "cabi")]
pub mod cabi;

pub use constants::*;
pub use eplite::command::processor::{ExecutionResult, Processor};
pub use eplite::database::Database;
pub use eplite::error::{Error, Result};
pub use eplite::graph::{Edge, EdgeId, Graph, GraphManager, Node, NodeId};
pub use eplite::query_builder::{
    CreateTableBuilder, DeleteBuilder, InsertBuilder, SelectBuilder, UpdateBuilder,
};
