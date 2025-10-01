mod eplite;

pub use eplite::command::processor::ExecutionResult;
pub use eplite::database::Database;
pub use eplite::error::{Error, Result};
pub use eplite::{SchemaFormat, TextEncoding};

pub const EPILOGLITE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SQLITE_SHM_NLOCK: u32 = 0;

pub enum SerializeFlags {
    SqliteSerializeNocopy = 0x01,
}
