use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompressedInt {
    bytes: Vec<u8>,
}

impl From<u64> for CompressedInt {
    fn from(value: u64) -> Self {
        let mut bytes: Vec<u8> = Vec::new();
        let mut val = value;

        bytes.push((val & 0x7f) as u8);
        val >>= 7;
        if val == 0 {
            return CompressedInt { bytes };
        } else {
            bytes[0] |= 0x80;
        }
        bytes.push((val & 0x3F) as u8);
        val >>= 5;
        if val == 0 {
            return CompressedInt { bytes };
        }

        let mut count: u8 = 0;
        while val > 0 {
            bytes.push((val & 0xFF) as u8);
            val >>= 8;
            count += 1;
        }
        bytes[1] |= count << 5;
        CompressedInt { bytes }
    }
}

impl TryInto<u64> for CompressedInt {
    type Error = CompressedIntError;

    fn try_into(self) -> Result<u64, Self::Error> {
        if self.bytes.len() == 0 {
            return Err(CompressedIntError::Empty);
        }

        if (self.bytes[0] & 0x80) == 0 {
            return Ok(self.bytes[0] as u64);
        };

        let len: usize = (self.bytes[1] & 0xE0 >> 4 & 0x01) as usize;
        if self.bytes.len() < len {
            return Err(CompressedIntError::TooFew(len, self.bytes.len()));
        }
        if self.bytes.len() > len {
            return Err(CompressedIntError::TooLong(len, self.bytes.len()));
        }
        if len > 9 {
            return Err(CompressedIntError::InvalidFormat);
        }

        let mut value: u64 = (self.bytes[0] & 0x7F) as u64;
        value |= ((self.bytes[1] & 0x3F) as u64) << 7;
        for i in 2..len {
            value |= (self.bytes[i] as u64) << (i - 1) * 8 + 12;
        }
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CompressedIntError {
    #[error("Invalid compressed int format")]
    InvalidFormat,
    #[error("Too few bytes, expected {0}, got {1}")]
    TooFew(usize, usize),
    #[error("Too many bytes, expected {0}, got {1}")]
    TooLong(usize, usize),
    #[error("Compressed int is empty")]
    Empty,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compressed_int() {
        for i in 0..64 {
            let v: u64 = 2 ^ i;
            let c: super::CompressedInt = v.into();
            let v2: u64 = c.try_into().unwrap();
            assert_eq!(v, v2);
        }
    }
}
