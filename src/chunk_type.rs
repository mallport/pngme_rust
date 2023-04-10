use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::fmt;
use std::str::from_utf8;
use std::str::FromStr;

use crate::chunk::Chunk;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    pub chunk_type: u32,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type.to_be_bytes()
    }
    pub fn get_bit_at(input: u32, n: u8) -> bool {
        if n < 32 {
            input & (1 << n) != 0
        } else {
            false
        }
    }
    pub fn is_valid(&self) -> bool {
        let all_chars_valid = self
            .chunk_type
            .to_be_bytes()
            .iter()
            .all(|&b| (b as char).is_alphabetic());
        all_chars_valid && self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool {
        !ChunkType::get_bit_at(self.chunk_type, 29)
    }
    pub fn is_public(&self) -> bool {
        !ChunkType::get_bit_at(self.chunk_type, 21)
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        !ChunkType::get_bit_at(self.chunk_type, 13)
    }
    pub fn is_safe_to_copy(&self) -> bool {
        ChunkType::get_bit_at(self.chunk_type, 5)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = Self {
            chunk_type: u32::from_be_bytes(value),
        };
        match chunk_type.is_valid() {
            true => Ok(chunk_type),
            false => Err(anyhow!("Chunk type bytes were not valid")),
        }
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_bytes = s.as_bytes();
        if str_bytes.len() != 4 {
            return Err(anyhow!("String was not 4 bytes"));
        }
        if str_bytes.iter().any(|&b| !(b as char).is_alphabetic()) {
            return Err(anyhow!("At least one character was not alphabetic"));
        }
        let sized_bytes: [u8; 4] = str_bytes.try_into()?;
        Ok(ChunkType {
            chunk_type: u32::from_be_bytes(sized_bytes),
        })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", from_utf8(&self.chunk_type.to_be_bytes()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
