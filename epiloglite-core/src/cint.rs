//! Compressed integer encoding and decoding
//! A compressed integer (CInt) is an integer encoded in 1 to 17 bytes, depending on its value.
//! There are no functions for u8 because it is always encoded in a single byte.
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    io::Read,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
use thiserror::Error;

const BYTE_0_MASK: u128 = 0x7F;
const BYTE_1_MASK: u128 = 0xF0;
const BYTE_1_INV_MASK: u128 = 0x0F;
const BYTE_N_MASK: u128 = 0xFF;

/// A compressed integer, encoded in 1 to 17 bytes.
/// Used for compact storage of integer values in EpilogLite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PartialOrd, Ord, Hash, Eq)]
pub struct CInt {
    /// The encoded bytes representing the integer.
    bytes: Vec<u8>,
}

impl CInt {
    /// Reads a `CInt` from a reader, reading the necessary number of bytes.
    /// Returns an error if the reader does not contain enough bytes or if the format is invalid.
    pub fn read_from(value: &mut dyn Read) -> Result<Self, CIntError> {
        let mut byte: [u8; 1] = [0];
        value.read_exact(&mut byte).map_err(|_| CIntError::Empty)?;
        let mut len: usize = ((byte[0] >> 7) + 1) as usize;
        let mut bytes: Vec<u8> = vec![byte[0]];
        if len == 1 {
            return Ok(CInt { bytes });
        }

        if len == 2 {
            byte = [0]; // reset to zero for safety; occasionally read_exact doesn't overwrite completely
            value
                .read_exact(&mut byte)
                .map_err(|_| CIntError::TooFew(2, 1))?;
            len += (byte[0] as usize & BYTE_1_MASK as usize) >> 4;
            bytes.push(byte[0]);
        }

        if len > 17 {
            return Err(CIntError::InvalidFormat);
        }
        for _ in 2..len {
            byte = [0];
            value
                .read_exact(&mut byte)
                .map_err(|_| CIntError::TooFew(len, bytes.len()))?;
            bytes.push(byte[0]);
        }
        Ok(CInt { bytes })
    }

    /// Create a `CInt` representing zero
    pub fn zero() -> Self {
        CInt { bytes: vec![0] }
    }
}

impl Display for CInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: Result<u128, CIntError> = self.clone().try_into();
        match v {
            Ok(v) => write!(f, "{}", v),
            Err(_) => write!(f, "NaN"),
        }
    }
}

impl Add for CInt {
    type Output = CInt;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs_val = u128::try_from(self.clone()).unwrap();
        let rhs_val = u128::try_from(rhs).unwrap();
        CInt::from(lhs_val + rhs_val)
    }
}

impl Sub for CInt {
    type Output = CInt;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs_val = u128::try_from(self.clone()).unwrap();
        let rhs_val = u128::try_from(rhs).unwrap();
        CInt::from(lhs_val - rhs_val)
    }
}

impl Mul for CInt {
    type Output = CInt;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs_val = u128::try_from(self.clone()).unwrap();
        let rhs_val = u128::try_from(rhs).unwrap();
        CInt::from(lhs_val * rhs_val)
    }
}

impl Div for CInt {
    type Output = CInt;

    fn div(self, rhs: Self) -> Self::Output {
        let lhs_val = u128::try_from(self.clone()).unwrap();
        let rhs_val = u128::try_from(rhs).unwrap();
        CInt::from(lhs_val / rhs_val)
    }
}

impl AddAssign for CInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl SubAssign for CInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl MulAssign for CInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl DivAssign for CInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl TryFrom<&mut Vec<u8>> for CInt {
    type Error = CIntError;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(CIntError::Empty);
        }
        let mut len: usize = (value[0] as usize >> 7) + 1;
        if len == 2 {
            len += (value[1] as usize & 0xF0) >> 4;
        }
        if value.len() < len {
            return Err(CIntError::TooFew(len as usize, value.len()));
        }

        let v = CInt {
            bytes: value[0..len].to_vec(),
        };
        value.drain(0..len);
        Ok(v)
    }
}

impl TryFrom<&[u8]> for CInt {
    type Error = CIntError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(CIntError::Empty);
        }
        if value.len() > 17 {
            return Err(CIntError::TooLong(17, value.len()));
        }
        Ok(CInt {
            bytes: value.to_vec(),
        })
    }
}

impl From<CInt> for Vec<u8> {
    fn from(value: CInt) -> Self {
        value.bytes.clone()
    }
}

impl From<u128> for CInt {
    fn from(value: u128) -> Self {
        let mut bytes: Vec<u8> = Vec::new();
        let mut val: u128 = value;

        bytes.push((val & BYTE_0_MASK) as u8);
        println! {"b0: bytes: {}, val: {}", bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(","), val};
        val >>= 7;
        if val == 0 {
            return CInt { bytes };
        } else {
            bytes[0] |= 0x80;
        }
        bytes.push((val & BYTE_1_INV_MASK) as u8);
        println! {"b1: bytes: {}, val: {}", bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(","), val};
        val >>= 4;
        if val == 0 {
            return CInt { bytes };
        }

        let mut count: u128 = 0;
        while val > 0 {
            bytes.push((val & BYTE_N_MASK) as u8);
            println! {"b{}: bytes: {}, val: {}", count,bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(","), val};
            val >>= 8;
            count += 1;
        }
        bytes[1] |= ((count << 4) & BYTE_1_MASK) as u8;
        CInt { bytes }
    }
}

impl From<usize> for CInt {
    fn from(value: usize) -> Self {
        (value as u128).into()
    }
}

impl From<u64> for CInt {
    fn from(value: u64) -> Self {
        (value as u128).into()
    }
}

impl From<u32> for CInt {
    fn from(value: u32) -> Self {
        (value as u128).into()
    }
}

impl From<u16> for CInt {
    fn from(value: u16) -> Self {
        (value as u128).into()
    }
}

impl From<i128> for CInt {
    fn from(value: i128) -> Self {
        let v: u128 = u128::from_be_bytes(value.to_be_bytes());
        v.into()
    }
}

impl From<i64> for CInt {
    fn from(value: i64) -> Self {
        let v: u128 = u64::from_be_bytes(value.to_be_bytes()) as u128;
        v.into()
    }
}

impl From<i32> for CInt {
    fn from(value: i32) -> Self {
        let v: u128 = u32::from_be_bytes(value.to_be_bytes()) as u128;
        v.into()
    }
}

impl From<i16> for CInt {
    fn from(value: i16) -> Self {
        let v: u128 = u16::from_be_bytes(value.to_be_bytes()) as u128;
        v.into()
    }
}

impl TryFrom<CInt> for u128 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        if value.bytes.len() == 0 {
            return Err(CIntError::Empty);
        }

        if (value.bytes[0] & 0x80) == 0 {
            return Ok(value.bytes[0] as u128);
        };

        let len: usize = (((value.bytes[1] & 0xF0) >> 4) + 2) as usize;
        if value.bytes.len() < len {
            return Err(CIntError::TooFew(len, value.bytes.len()));
        }
        if value.bytes.len() > len {
            return Err(CIntError::TooLong(len, value.bytes.len()));
        }
        if len > 17 {
            return Err(CIntError::InvalidFormat);
        }

        let mut v: u128 = (value.bytes[0] & 0x7F) as u128;
        v |= ((value.bytes[1] & 0x0F) as u128) << 7;
        for i in 2..len {
            v |= (value.bytes[i] as u128) << (i - 2) * 8 + 11;
        }
        Ok(v)
    }
}

impl TryFrom<CInt> for usize {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u128 = value.try_into()?;
        if v > usize::MAX as u128 {
            return Err(CIntError::ValueOutOfRange(usize::MAX as u128, v));
        }
        Ok(v as usize)
    }
}

impl TryFrom<CInt> for u64 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u128 = value.try_into()?;
        if v > u64::MAX as u128 {
            return Err(CIntError::ValueOutOfRange(u128::MAX as u128, v));
        }
        Ok(v as u64)
    }
}

impl TryFrom<CInt> for u32 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u128 = value.try_into()?;
        if v > u32::MAX as u128 {
            return Err(CIntError::ValueOutOfRange(u32::MAX as u128, v));
        }
        Ok(v as u32)
    }
}

impl TryFrom<CInt> for u16 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u128 = value.try_into()?;
        if v > u16::MAX as u128 {
            return Err(CIntError::ValueOutOfRange(u16::MAX as u128, v));
        }
        Ok(v as u16)
    }
}

impl TryFrom<CInt> for i128 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u128 = value.try_into()?;
        let bytes: [u8; 16] = (v as u128).to_be_bytes();
        Ok(i128::from_be_bytes(bytes))
    }
}

impl TryFrom<CInt> for i64 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u64 = value.try_into()?;
        let bytes: [u8; 8] = v.to_be_bytes();
        Ok(i64::from_be_bytes(bytes))
    }
}

impl TryFrom<CInt> for i32 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u32 = value.try_into()?;
        let bytes: [u8; 4] = v.to_be_bytes();
        Ok(i32::from_be_bytes(bytes))
    }
}

impl TryFrom<CInt> for i16 {
    type Error = CIntError;

    fn try_from(value: CInt) -> Result<Self, Self::Error> {
        let v: u16 = value.try_into()?;
        let bytes: [u8; 2] = v.to_be_bytes();
        Ok(i16::from_be_bytes(bytes))
    }
}

/// Errors that can occur during compressed integer encoding or decoding
#[derive(Debug, Clone, PartialEq, Error)]
pub enum CIntError {
    /// The compressed int format is invalid, typically because the decoded length is invalid (<1 || >17)
    #[error("Invalid compressed int format")]
    InvalidFormat,
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
    #[error("Compressed int is empty")]
    Empty,
}
