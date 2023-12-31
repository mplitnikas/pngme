use crate::{Error, Result};
use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::str::FromStr;

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct ChunkType {
    bytes: [u8; 4],
}
impl ChunkType {
    pub fn try_new(bytes: [u8; 4]) -> Result<Self> {
        let valid_ascii = bytes
            .iter()
            .map(|&x| x as char)
            .all(|x| (x >= 'a' && x <= 'z') || (x >= 'A' && x <= 'Z'));
        match valid_ascii {
            true => Ok(ChunkType { bytes }),
            false => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("invalid ascii: {}", String::from_utf8_lossy(&bytes)),
            ))),
        }
    }
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
    pub fn is_critical(&self) -> bool {
        ((self.bytes[0] >> 5) & 1) == 0
    }
    pub fn is_public(&self) -> bool {
        ((self.bytes[1] >> 5) & 1) == 0
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        ((self.bytes[2] >> 5) & 1) == 0
    }
    pub fn is_safe_to_copy(&self) -> bool {
        ((self.bytes[3] >> 5) & 1) == 1
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}
impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_rep = std::str::from_utf8(&self.bytes).unwrap();
        write!(f, "{}", string_rep)
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self> {
        ChunkType::try_new(value)
    }
}
impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self> {
        let bytes: [u8; 4] = value.as_bytes().try_into().map_err(|_| "invalid string")?;
        ChunkType::try_new(bytes)
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
