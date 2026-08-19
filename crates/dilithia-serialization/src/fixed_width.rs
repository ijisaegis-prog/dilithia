//! Canonical fixed-width unsigned integer encoding and decoding.

use crate::SerializationError;

/// Encodes a `u8` as exactly one byte.
pub fn encode_u8(value: u8) -> [u8; 1] {
    value.to_le_bytes()
}

/// Encodes a `u16` as exactly two little-endian bytes.
pub fn encode_u16(value: u16) -> [u8; 2] {
    value.to_le_bytes()
}

/// Encodes a `u32` as exactly four little-endian bytes.
pub fn encode_u32(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

/// Encodes a `u64` as exactly eight little-endian bytes.
pub fn encode_u64(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}

/// Encodes a `u128` as exactly sixteen little-endian bytes.
pub fn encode_u128(value: u128) -> [u8; 16] {
    value.to_le_bytes()
}

/// Decodes a `u8` from exactly one byte at the start of `input`.
pub fn decode_u8(input: &mut &[u8]) -> Result<u8, SerializationError> {
    Ok(u8::from_le_bytes(take_bytes(input)?))
}

/// Decodes a `u16` from two little-endian bytes at the start of `input`.
pub fn decode_u16(input: &mut &[u8]) -> Result<u16, SerializationError> {
    Ok(u16::from_le_bytes(take_bytes(input)?))
}

/// Decodes a `u32` from four little-endian bytes at the start of `input`.
pub fn decode_u32(input: &mut &[u8]) -> Result<u32, SerializationError> {
    Ok(u32::from_le_bytes(take_bytes(input)?))
}

/// Decodes a `u64` from eight little-endian bytes at the start of `input`.
pub fn decode_u64(input: &mut &[u8]) -> Result<u64, SerializationError> {
    Ok(u64::from_le_bytes(take_bytes(input)?))
}

/// Decodes a `u128` from sixteen little-endian bytes at the start of `input`.
pub fn decode_u128(input: &mut &[u8]) -> Result<u128, SerializationError> {
    Ok(u128::from_le_bytes(take_bytes(input)?))
}

pub(crate) fn take_bytes<const N: usize>(input: &mut &[u8]) -> Result<[u8; N], SerializationError> {
    let (bytes, remaining) = input
        .split_first_chunk::<N>()
        .ok_or(SerializationError::UnexpectedEof)?;
    let bytes = *bytes;
    *input = remaining;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use super::*;

    const U8_VECTORS: &[(u8, [u8; 1])] =
        &[(0, [0x00]), (1, [0x01]), (u8::MAX, [0xff]), (0xa5, [0xa5])];
    const U16_VECTORS: &[(u16, [u8; 2])] = &[
        (0, [0x00, 0x00]),
        (1, [0x01, 0x00]),
        (u16::MAX, [0xff, 0xff]),
        (0x1234, [0x34, 0x12]),
    ];
    const U32_VECTORS: &[(u32, [u8; 4])] = &[
        (0, [0x00, 0x00, 0x00, 0x00]),
        (1, [0x01, 0x00, 0x00, 0x00]),
        (u32::MAX, [0xff, 0xff, 0xff, 0xff]),
        (0x1234_5678, [0x78, 0x56, 0x34, 0x12]),
    ];
    const U64_VECTORS: &[(u64, [u8; 8])] = &[
        (0, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (1, [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (u64::MAX, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
        (
            0x0123_4567_89ab_cdef,
            [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01],
        ),
    ];
    const U128_VECTORS: &[(u128, [u8; 16])] = &[
        (
            0,
            [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        ),
        (
            1,
            [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        ),
        (
            u128::MAX,
            [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff,
            ],
        ),
        (
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
            [
                0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45,
                0x23, 0x01,
            ],
        ),
    ];

    #[test]
    fn encodes_u8_fixed_vectors_at_exact_width() {
        for &(value, expected) in U8_VECTORS {
            let encoded = encode_u8(value);
            assert_eq!(encoded, expected);
            assert_eq!(encoded.len(), 1);
        }
    }

    #[test]
    fn encodes_u16_fixed_vectors_at_exact_width() {
        for &(value, expected) in U16_VECTORS {
            let encoded = encode_u16(value);
            assert_eq!(encoded, expected);
            assert_eq!(encoded.len(), 2);
        }
    }

    #[test]
    fn encodes_u32_fixed_vectors_at_exact_width() {
        for &(value, expected) in U32_VECTORS {
            let encoded = encode_u32(value);
            assert_eq!(encoded, expected);
            assert_eq!(encoded.len(), 4);
        }
    }

    #[test]
    fn encodes_u64_fixed_vectors_at_exact_width() {
        for &(value, expected) in U64_VECTORS {
            let encoded = encode_u64(value);
            assert_eq!(encoded, expected);
            assert_eq!(encoded.len(), 8);
        }
    }

    #[test]
    fn encodes_u128_fixed_vectors_at_exact_width() {
        for &(value, expected) in U128_VECTORS {
            let encoded = encode_u128(value);
            assert_eq!(encoded, expected);
            assert_eq!(encoded.len(), 16);
        }
    }

    #[test]
    fn decodes_u8_fixed_vectors_and_advances_input() {
        for &(expected, encoded) in U8_VECTORS {
            assert_decodes_and_advances(&encoded, expected, decode_u8);
        }
    }

    #[test]
    fn decodes_u16_fixed_vectors_and_advances_input() {
        for &(expected, encoded) in U16_VECTORS {
            assert_decodes_and_advances(&encoded, expected, decode_u16);
        }
    }

    #[test]
    fn decodes_u32_fixed_vectors_and_advances_input() {
        for &(expected, encoded) in U32_VECTORS {
            assert_decodes_and_advances(&encoded, expected, decode_u32);
        }
    }

    #[test]
    fn decodes_u64_fixed_vectors_and_advances_input() {
        for &(expected, encoded) in U64_VECTORS {
            assert_decodes_and_advances(&encoded, expected, decode_u64);
        }
    }

    #[test]
    fn decodes_u128_fixed_vectors_and_advances_input() {
        for &(expected, encoded) in U128_VECTORS {
            assert_decodes_and_advances(&encoded, expected, decode_u128);
        }
    }

    #[test]
    fn rejects_every_truncated_u8_input_without_advancing() {
        assert_all_truncated_inputs_unchanged(1, decode_u8);
    }

    #[test]
    fn rejects_every_truncated_u16_input_without_advancing() {
        assert_all_truncated_inputs_unchanged(2, decode_u16);
    }

    #[test]
    fn rejects_every_truncated_u32_input_without_advancing() {
        assert_all_truncated_inputs_unchanged(4, decode_u32);
    }

    #[test]
    fn rejects_every_truncated_u64_input_without_advancing() {
        assert_all_truncated_inputs_unchanged(8, decode_u64);
    }

    #[test]
    fn rejects_every_truncated_u128_input_without_advancing() {
        assert_all_truncated_inputs_unchanged(16, decode_u128);
    }

    fn assert_decodes_and_advances<T: Debug + PartialEq>(
        encoded: &[u8],
        expected: T,
        decode: fn(&mut &[u8]) -> Result<T, SerializationError>,
    ) {
        let mut bytes = Vec::from(encoded);
        bytes.push(0xaa);
        let mut input = bytes.as_slice();

        assert_eq!(decode(&mut input), Ok(expected));
        assert_eq!(input, &[0xaa]);
    }

    fn assert_all_truncated_inputs_unchanged<T: Debug + PartialEq>(
        width: usize,
        decode: fn(&mut &[u8]) -> Result<T, SerializationError>,
    ) {
        const BYTES: [u8; 16] = [0xa5; 16];

        for length in 0..width {
            let mut input = &BYTES[..length];
            let original = input;

            assert_eq!(decode(&mut input), Err(SerializationError::UnexpectedEof));
            assert_eq!(input, original);
        }
    }
}
