//! Canonical Unix timestamp encoding and decoding.

use crate::{SerializationError, decode_u64, encode_u64};

/// Encodes milliseconds since the Unix epoch using canonical `u64` encoding.
pub fn encode_unix_timestamp(value: u64) -> [u8; 8] {
    encode_u64(value)
}

/// Decodes milliseconds since the Unix epoch using canonical `u64` encoding.
pub fn decode_unix_timestamp(input: &mut &[u8]) -> Result<u64, SerializationError> {
    decode_u64(input)
}

#[cfg(test)]
mod tests {
    use super::{decode_unix_timestamp, encode_unix_timestamp};
    use crate::SerializationError;

    const VECTORS: &[(u64, [u8; 8])] = &[
        (0, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (1, [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (1_000, [0xe8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (u64::MAX, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
    ];

    #[test]
    fn encodes_fixed_vectors_at_exact_width() {
        for &(value, expected) in VECTORS {
            let encoded = encode_unix_timestamp(value);

            assert_eq!(encoded, expected, "value {value}");
            assert_eq!(encoded.len(), 8);
        }
    }

    #[test]
    fn decodes_fixed_vectors_and_leaves_trailing_bytes_untouched() {
        for &(expected, encoded) in VECTORS {
            let mut bytes = Vec::from(encoded);
            bytes.extend_from_slice(&[0xaa, 0x55]);
            let mut input = bytes.as_slice();

            assert_eq!(decode_unix_timestamp(&mut input), Ok(expected));
            assert_eq!(input, &[0xaa, 0x55]);
        }
    }

    #[test]
    fn rejects_every_truncated_input_without_advancing_cursor() {
        const ENCODED: [u8; 8] = [0xe8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        for length in 0..8 {
            let mut input = &ENCODED[..length];
            let original = input;

            assert_eq!(
                decode_unix_timestamp(&mut input),
                Err(SerializationError::UnexpectedEof),
                "length {length} must be rejected"
            );
            assert_eq!(
                input, original,
                "length {length} must not advance the cursor"
            );
        }
    }
}
