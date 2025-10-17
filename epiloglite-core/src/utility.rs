use bincode::{
    config::{BigEndian, Configuration, Fixint},
    de::read::Reader,
    enc::write::Writer,
    error::{DecodeError, EncodeError},
};
use crc_adler::crc32;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt::Debug;

/// Calculate the CRC32 checksum of a serializable value, excluding the last 4 bytes (CRC field itself).
/// Returns 0 if the encoded value is too short.
pub fn calculate_crc<T>(value: &T) -> u32
where
    T: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug,
{
    let bytes = bincode::serde::encode_to_vec(value, BINCODE_CONFIG).unwrap_or_default();
    if bytes.len() < 4 {
        return 0;
    }

    let truncated = &bytes[..bytes.len() - 4];
    crc32(truncated)
}

/// Standard bincode configuration for EpilogLite (big-endian, fixed int, no size limit)
pub static BINCODE_CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .with_no_limit();

/// Deserialize an object from a reader using the standard bincode config.
pub fn try_from_reader<T, U>(reader: &mut T) -> Result<U, DecodeError>
where
    T: Reader,
    U: Debug + Serialize + DeserializeOwned,
{
    bincode::serde::decode_from_reader(reader, BINCODE_CONFIG)
}

/// Serialize an object to a writer using the standard bincode config.
pub fn try_to_writer<U, T>(value: &U, writer: &mut T) -> Result<(), EncodeError>
where
    U: Debug + Serialize + DeserializeOwned,
    T: Writer,
{
    bincode::serde::encode_into_writer(value, writer, BINCODE_CONFIG)
}

/// Serialize an object to a byte vector using the standard bincode config.
pub fn try_into_vec<T>(value: &T) -> Result<Vec<u8>, EncodeError>
where
    T: Debug + Serialize + DeserializeOwned,
{
    bincode::serde::encode_to_vec(value, BINCODE_CONFIG)
}

/// Deserialize an object from a byte slice
pub fn try_from_slice<U>(value: &[u8]) -> Result<U, DecodeError>
where
    U: Debug + Serialize + DeserializeOwned,
{
    let (entry, _): (U, _) = bincode::serde::decode_from_slice(value, BINCODE_CONFIG)?;
    Ok(entry)
}

#[cfg(test)]
#[path = "tests/utility_tests.rs"]
mod utility_tests;
