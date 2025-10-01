pub mod command;
mod constants;
pub mod database;
pub mod error;
pub mod os;
pub mod persistence;
mod traits;
mod types;
pub mod utility;

pub use constants::*;
pub use error::{Error, Result};

#[derive(Debug)]
pub struct EPLite {}
