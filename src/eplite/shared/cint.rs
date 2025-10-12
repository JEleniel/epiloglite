//! Compressed integer encoding and decoding
use std::io::Read;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PartialOrd)]
pub struct CInt {
    bytes: Vec<u8>,
}

impl CInt {
    pub fn try_from_reader(value: &mut dyn Read) -> Result<Self, CIntError> {
        let mut byte: [u8; 1] = [0];
        value.read_exact(&mut byte).map_err(|_| CIntError::Empty)?;
        let mut len: usize = (byte[0] >> 7 + 1) as usize;
        let mut bytes: Vec<u8> = vec![byte[0]];
        if len == 1 {
            return Ok(CInt { bytes });
        }

        if len == 2 {
            byte = [0];
            value
                .read_exact(&mut byte)
                .map_err(|_| CIntError::TooFew(2, 1))?;
            len += (byte[0] as usize & 0xF0) >> 4;
            bytes.push(byte[0] & 0x0F);
        }

        if len > 17 {
            return Err(CIntError::InvalidFormat);
        }
        for _ in 2..len {
            value
                .read_exact(&mut byte)
                .map_err(|_| CIntError::TooFew(len, bytes.len()))?;
            bytes.push(byte[0]);
        }
        Ok(CInt { bytes })
    }
}

impl TryFrom<&mut Vec<u8>> for CInt {
    type Error = CIntError;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(CIntError::Empty);
        }
        let mut len: usize = value[0] as usize >> 7 + 1;
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

        bytes.push((val & 0x7f) as u8);
        val >>= 7;
        if val == 0 {
            return CInt { bytes };
        } else {
            bytes[0] |= 0x80;
        }
        bytes.push((val & 0x0F) as u8);
        val >>= 4;
        if val == 0 {
            return CInt { bytes };
        }

        let mut count: u8 = 0;
        while val > 0 {
            bytes.push((val & 0xFF) as u8);
            val >>= 8;
            count += 1;
        }
        bytes[1] |= count << 4;
        CInt { bytes }
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
        type Error = CIntError;

        if value.bytes.len() == 0 {
            return Err(CIntError::Empty);
        }

        if (value.bytes[0] & 0x80) == 0 {
            return Ok(value.bytes[0] as u128);
        };

        let len: usize = ((value.bytes[1] & 0xF0 >> 4) + 2) as usize;
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
            v |= (value.bytes[i] as u128) << (i - 1) * 8 + 11;
        }
        Ok(v)
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

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CIntError {
    #[error("Invalid compressed int format")]
    InvalidFormat,
    #[error("Too few bytes, expected {0}, got {1}")]
    TooFew(usize, usize),
    #[error("Too many bytes, expected {0}, got {1}")]
    TooLong(usize, usize),
    #[error("Value out of range, expected max {0}, got {1}")]
    ValueOutOfRange(u128, u128),
    #[error("Compressed int is empty")]
    Empty,
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::CInt;

    macro_rules! test_xxx {
        ($i:ident, $t:ty) => {
            #[test]
            fn $i() {
                let b = <$t>::BITS;
                for i in 0..b {
                    let v: $t = ((2 ^ i) - 1).try_into().unwrap();
                    let c: super::CInt = v.try_into().unwrap();
                    let v2: $t = c.try_into().unwrap();
                    assert_eq!(v, v2);
                }
            }
        };
    }

    test_xxx!(test_cint_u16, u16);
    test_xxx!(test_cint_u32, u32);
    test_xxx!(test_cint_u64, u64);
    test_xxx!(test_cint_u128, u128);

    test_xxx!(test_cint_i16, i16);
    test_xxx!(test_cint_i32, i32);
    test_xxx!(test_cint_i64, i64);
    test_xxx!(test_cint_i128, i128);

    #[test]
    fn test_cint_vec() {
        for i in 0..128 {
            let v: u128 = ((2 ^ i) - 1).try_into().unwrap();
            let c: super::CInt = v.try_into().unwrap();
            let v2: &mut Vec<u8> = &mut c.try_into().unwrap();
            let v3: u128 = CInt::try_from(v2).unwrap().try_into().unwrap();
            assert_eq!(v, v3);
        }
    }

    #[test]
    fn test_cint_arr() {
        for i in 0..128 {
            let v: u128 = ((2 ^ i) - 1).try_into().unwrap();
            let c: super::CInt = v.try_into().unwrap();
            let v2: Vec<u8> = c.try_into().unwrap();
            let v3: u128 = CInt::try_from(v2.as_slice()).unwrap().try_into().unwrap();
            assert_eq!(v, v3);
        }
    }

    #[test]
    fn test_cint_read() {
        for i in 0..128 {
            let v: u128 = ((2 ^ i) - 1).try_into().unwrap();
            let c: super::CInt = v.try_into().unwrap();
            let v2: Vec<u8> = c.try_into().unwrap();
            let v3: u128 = CInt::try_from_reader(&mut BufReader::new(v2.as_slice()))
                .unwrap()
                .try_into()
                .unwrap();
            assert_eq!(v, v3);
        }
    }
}
