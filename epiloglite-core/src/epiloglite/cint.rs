//! Compressed unsigned integer (CInt) implementation
//! A compressed integer (CInt) is a u128 encoded in 1 to 17 bytes, depending on its value.
//! There are no functions for u8 because it is always encoded in a single byte already.
use serde::{Deserialize, Serialize};
use std::io::Read;
use thiserror::Error;

const BYTE_0_COUNT_MASK: u8 = 0x80;
const BYTE_0_VALUE_MASK: u8 = !BYTE_0_COUNT_MASK;
const BYTE_1_COUNT_MASK: u8 = 0xF0;
const BYTE_1_VALUE_MASK: u8 = !BYTE_1_COUNT_MASK;
const BYTE_N_VALUE_MASK: u8 = 0xFF;

/// A compressed integer, encoded in 1 to 17 bytes.
/// Used for compact storage of integer values in EpilogLite.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CInt {
    bytes: Vec<u8>,
}

impl CInt {
    /// Reads a `CInt` from a reader, reading the necessary number of bytes.
    /// Returns an error if the reader does not contain enough bytes or if the format is invalid.
    pub fn read_from(reader: &mut dyn Read) -> Result<Self, CIntError> {
        let mut bytes: Vec<u8> = Vec::new();

        let mut byte: [u8; 1] = [0];
        reader
            .read_exact(&mut byte)
            .map_err(|_| CIntError::NoData)?;
        bytes.push(byte[0]);

        if !(byte[0] & 0x80 != 0) {
            return Ok(CInt { bytes });
        }

        let mut len = 1 + ((byte[0] & BYTE_0_VALUE_MASK) as usize >> 7);
        if len != 2 {
            return Err(CIntError::InvalidEncodedLength(len));
        }

        byte = [0]; // Ensure byte is reset
        reader
            .read_exact(&mut byte)
            .map_err(|_| CIntError::TooFew(len, 1))?;
        bytes.push(byte[0]);
        len += ((byte[0] & BYTE_1_COUNT_MASK) >> 4) as usize;
        if len > 17 {
            return Err(CIntError::InvalidEncodedLength(len));
        }

        for i in 1..len {
            byte = [0];
            reader
                .read_exact(&mut byte)
                .map_err(|_| CIntError::TooFew(len, i))?;
            bytes.push(byte[0]);
        }
        Ok(CInt { bytes })
    }
}

impl std::fmt::Display for CInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Clone and reuse the existing From<CInt> for u128 implementation
        let v: u128 = u128::from(self.clone());
        write!(f, "{}", v)
    }
}

impl From<u128> for CInt {
    fn from(value: u128) -> Self {
        if value < BYTE_0_COUNT_MASK as u128 {
            return CInt {
                bytes: vec![value as u8],
            };
        }

        let mut bytes: Vec<u8> = Vec::new();
        let mut tv: u128 = value.clone();
        bytes.push(((tv & BYTE_1_VALUE_MASK as u128) as u8) | BYTE_0_COUNT_MASK);
        tv >>= 7;

        let mut byte_count: u8 = 0;
        bytes.push((tv & BYTE_1_VALUE_MASK as u128) as u8);
        tv >>= 4;

        while tv != 0 {
            bytes.push((tv & BYTE_N_VALUE_MASK as u128) as u8);
            tv >>= 8;
            byte_count += 1;
        }
        bytes[0] |= byte_count << 4;

        CInt { bytes }
    }
}

// Note: signed integer conversions intentionally omitted. CInt encodes unsigned values only.

impl From<CInt> for u128 {
    fn from(value: CInt) -> Self {
        let bytes = value.bytes.clone();
        if bytes.is_empty() {
            return 0;
        }

        if bytes[0] & BYTE_0_COUNT_MASK == 0 {
            return bytes[0] as u128;
        }

        let mut value: u128 = (bytes[0] & BYTE_1_VALUE_MASK) as u128;
        value |= ((bytes[1] & BYTE_1_VALUE_MASK) as u128) << 7;

        let byte_count = bytes[1] >> 4;
        for i in 0..byte_count {
            value |= (bytes[(2 + i) as usize] as u128) << (11 + (i as u32 * 8));
        }

        value
    }
}

impl From<u64> for CInt {
    fn from(value: u64) -> Self {
        CInt::from(value as u128)
    }
}

impl From<u32> for CInt {
    fn from(value: u32) -> Self {
        CInt::from(value as u128)
    }
}

impl From<u16> for CInt {
    fn from(value: u16) -> Self {
        CInt::from(value as u128)
    }
}

impl From<usize> for CInt {
    fn from(value: usize) -> Self {
        CInt::from(value as u128)
    }
}

// Try conversions FROM CInt into smaller integer types. These validate range and return a
// `CIntError::ValueOutOfRange` when the encoded value doesn't fit the target type.
impl std::convert::TryFrom<CInt> for u16 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u128 = u128::from(value);
        if v > u16::MAX as u128 {
            return Err(CIntError::ValueOutOfRange(u16::MAX as u128, v));
        }
        Ok(v as u16)
    }
}

impl std::convert::TryFrom<CInt> for u32 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u128 = u128::from(value);
        if v > u32::MAX as u128 {
            return Err(CIntError::ValueOutOfRange(u32::MAX as u128, v));
        }
        Ok(v as u32)
    }
}

impl std::convert::TryFrom<CInt> for u64 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u128 = u128::from(value);
        if v > u64::MAX as u128 {
            return Err(CIntError::ValueOutOfRange(u64::MAX as u128, v));
        }
        Ok(v as u64)
    }
}

impl std::convert::TryFrom<CInt> for usize {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u128 = u128::from(value);
        if v > usize::MAX as u128 {
            return Err(CIntError::ValueOutOfRange(usize::MAX as u128, v));
        }
        Ok(v as usize)
    }
}

/// Errors that can occur during compressed integer encoding or decoding
#[derive(Clone, Debug, Error, PartialEq)]
pub enum CIntError {
    /// The encoded number of bytes is invalid
    #[error("Invalid byte count in compressed int, expected 1-17 bytes, found {0} encoded")]
    InvalidEncodedLength(usize),
    /// Not enough bytes in the input to decode the expected length
    #[error("Too few bytes, expected {0}, got {1}")]
    TooFew(usize, usize),
    /// Too many bytes in the input to decode the expected length
    #[error("Too many bytes, expected {0}, got {1}")]
    TooLong(usize, usize),
    /// The decoded value is out of range for the target type
    #[error("Value out of range, expected max {0}, got {1}")]
    ValueOutOfRange(u128, u128),
    /// The compressed int is empty (no bytes)
    #[error("No bytes to decode compressed int")]
    NoData,
    /// The value would overflow the target type during conversion
    #[error("Overflow during conversion")]
    Overflow,
    /// The math operation would underflow the target type during conversion
    #[error("Underflow during conversion")]
    Underflow,
}
