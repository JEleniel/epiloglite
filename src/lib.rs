mod eplite;

pub const EPILOGLITE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SQLITE_SHM_NLOCK: u32 = 0;

pub enum SerializeFlags {
    SqliteSerializeNocopy = 0x01,
}
