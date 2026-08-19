//! Canonical fixed-width `U256` encoding and decoding.

use crate::{SerializationError, fixed_width::take_bytes};

/// An unsigned 256-bit value with private canonical little-endian storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct U256 {
    bytes_le: [u8; 32],
}

impl U256 {
    /// Constructs a `U256` from exactly 32 little-endian bytes.
    pub const fn from_le_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes_le: bytes }
    }

    /// Returns the canonical 32-byte little-endian representation.
    pub const fn to_le_bytes(&self) -> [u8; 32] {
        self.bytes_le
    }
}

/// Encodes a `U256` as exactly 32 little-endian bytes.
pub fn encode_u256(value: &U256) -> [u8; 32] {
    value.to_le_bytes()
}

/// Decodes a `U256` from 32 bytes at the start of `input`.
pub fn decode_u256(input: &mut &[u8]) -> Result<U256, SerializationError> {
    Ok(U256::from_le_bytes(take_bytes(input)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO: [u8; 32] = [0x00; 32];
    const ONE: [u8; 32] = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];
    const MAXIMUM: [u8; 32] = [0xff; 32];
    const HIGHEST_BIT: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x80,
    ];
    const REPRESENTATIVE: [u8; 32] = [
        0x20, 0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12,
        0x11, 0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03,
        0x02, 0x01,
    ];
    const VECTORS: &[[u8; 32]] = &[ZERO, ONE, MAXIMUM, HIGHEST_BIT, REPRESENTATIVE];

    #[test]
    fn explicit_little_endian_conversions_preserve_fixed_vectors() {
        for &expected in VECTORS {
            let value = U256::from_le_bytes(expected);

            assert_eq!(value.to_le_bytes(), expected);
        }
    }

    #[test]
    fn encodes_fixed_vectors_at_exact_width() {
        for &expected in VECTORS {
            let value = U256::from_le_bytes(expected);
            let encoded = encode_u256(&value);

            assert_eq!(encoded, expected);
            assert_eq!(encoded.len(), 32);
        }
    }

    #[test]
    fn decodes_fixed_vectors_and_advances_input() {
        for &expected in VECTORS {
            let mut bytes = Vec::from(expected);
            bytes.extend_from_slice(&[0xaa, 0x55]);
            let mut input = bytes.as_slice();

            let decoded = decode_u256(&mut input).expect("fixed vector must decode");

            assert_eq!(decoded.to_le_bytes(), expected);
            assert_eq!(input, &[0xaa, 0x55]);
        }
    }

    #[test]
    fn rejects_every_truncated_input_without_advancing() {
        for length in 0..32 {
            let mut input = &REPRESENTATIVE[..length];
            let original = input;

            assert_eq!(
                decode_u256(&mut input),
                Err(SerializationError::UnexpectedEof)
            );
            assert_eq!(input, original);
        }
    }
}
