use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Varint {
    pub bytes: Vec<u8>,
}

impl From<i64> for Varint {
    fn from(value: i64) -> Self {
        let mut bytes = Vec::new();
        let mut val = value;
        loop {
            let byte = (val & 0x7F) as u8;
            val >>= 7;
            if val == 0 {
                bytes.push(byte);
                break;
            } else {
                bytes.push(byte | 0x80);
            }
        }
        Varint { bytes }
    }
}

impl TryInto<i64> for Varint {
    type Error = VarintError;

    fn try_into(self) -> Result<i64, Self::Error> {
        if self.bytes.is_empty() {
            return Err(VarintError::TooShort);
        }
        if self.bytes.len() > 9 {
            return Err(VarintError::TooLong(self.bytes.len()));
        }
        let mut result = 0i64;
        for (i, byte) in self.bytes.iter().enumerate() {
            let value = (byte & 0x7F) as i64;
            result |= value << (7 * i);
            if byte & 0x80 == 0 {
                return Ok(result);
            }
        }
        Err(VarintError::TooLong(self.bytes.len()))
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum VarintError {
    #[error("Varint has 0 bytes")]
    TooShort,
    #[error("Too many bytes in varint {0}")]
    TooLong(usize),
}

#[cfg(test)]
mod tests {
    use crate::sqlite::{Varint, VarintError};

    #[test]
    fn test_varint() {
        let values = [
            0i64,
            1,
            127,
            128,
            255,
            300,
            16384,
            2097151,
            268435455,
            34359738367,
            4398046511103,
            562949953421311,
            72057594037927935,
            i64::MAX,
        ];
        for &value in &values {
            println!("Testing value: {}", value);
            let varint: Varint = value.into();
            let decoded: i64 = varint.clone().try_into().unwrap();
            assert_eq!(value, decoded, "Failed for value: {}", value);
        }

        // Test error cases
        let empty_varint = Varint { bytes: vec![] };
        assert_eq!(
            TryInto::<i64>::try_into(empty_varint),
            Err(VarintError::TooShort),
            "Empty varint should return TooShort error"
        );

        let long_varint = Varint {
            bytes: vec![0x80; 10],
        };
        assert_eq!(
            TryInto::<i64>::try_into(long_varint),
            Err(VarintError::TooLong(10)),
            "Varint with too many bytes should return TooLong error"
        );
    }
}
