use bincode::{
    config::{BigEndian, Configuration, Fixint},
    de::read::Reader,
    enc::write::Writer,
    error::{DecodeError, EncodeError},
};
use crc_adler::crc32;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt::Debug;
use thiserror::Error;

/// Standard bincode configuration for EpilogLite (big-endian, fixed int, no size limit)
pub static BINCODE_CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .with_no_limit();

/// Calculate the CRC32 checksum of a serializable value, excluding the last 4 bytes (CRC field itself).
pub fn calculate_crc<T>(value: &T) -> Result<u32, SerializeError>
where
    T: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug,
{
    let bytes = try_into_vec(value)?;
    if bytes.len() < 4 {
        return Err(SerializeError::CrcInputEmpty);
    }

    // The CRC is always stores in the last 4 bytes, so exclude them from the calculation
    let truncated = &bytes[..bytes.len() - 4];
    Ok(crc32(truncated))
}

/// Deserialize an object from a reader using the standard bincode config.
pub fn try_from_reader<T, U>(reader: &mut T) -> Result<U, SerializeError>
where
    T: Reader,
    U: Debug + Serialize + DeserializeOwned,
{
    bincode::serde::decode_from_reader(reader, BINCODE_CONFIG).map_err(SerializeError::from)
}

/// Serialize an object to a writer using the standard bincode config.
pub fn try_to_writer<U, T>(value: &U, writer: &mut T) -> Result<(), SerializeError>
where
    U: Debug + Serialize + DeserializeOwned,
    T: Writer,
{
    bincode::serde::encode_into_writer(value, writer, BINCODE_CONFIG).map_err(SerializeError::from)
}

/// Serialize an object to a byte vector using the standard bincode config.
pub fn try_into_vec<T>(value: &T) -> Result<Vec<u8>, SerializeError>
where
    T: Debug + Serialize + DeserializeOwned,
{
    bincode::serde::encode_to_vec(value, BINCODE_CONFIG).map_err(SerializeError::from)
}

/// Get the serialized size of an object in bytes
pub fn serialized_size<T>(value: &T) -> Result<usize, SerializeError>
where
    T: Debug + Serialize + DeserializeOwned,
{
    let bytes = bincode::serde::encode_to_vec(value, BINCODE_CONFIG)?;
    Ok(bytes.len())
}

/// Deserialize an object from a byte slice
pub fn try_from_slice<U>(value: &[u8]) -> Result<U, SerializeError>
where
    U: Debug + Serialize + DeserializeOwned,
{
    bincode::serde::decode_from_slice(value, BINCODE_CONFIG)
        .map(|(v, _)| v)
        .map_err(SerializeError::from)
}

/// Errors that can occur during serialization or deserialization
/// All utility functions involve serialization or deserialization and will return this error type
#[derive(Debug, Error)]
pub enum SerializeError {
    /// An error occurred while encoding a type
    #[error("Encode error: {0}")]
    Encode(#[from] EncodeError),
    /// An error occurred while decoding a type
    #[error("Decode error: {0}")]
    Decode(#[from] DecodeError),
    /// The input to the CRC is empty
    #[error("CRC calculation error: input is empty")]
    CrcInputEmpty,
    /// An error occurred while converting between integer types
    #[error("Integer conversion error: {0}")]
    IntConversion(#[from] std::num::TryFromIntError),
}

#[cfg(test)]
#[path = "tests/utility_tests.rs"]
mod utility_tests;
