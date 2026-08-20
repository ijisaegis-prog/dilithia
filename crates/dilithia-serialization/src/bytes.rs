//! Canonical fixed-size byte-array encoding and decoding.

use crate::{SerializationError, fixed_width::take_bytes};

/// Encodes a fixed-size byte array without a length prefix or padding.
pub fn encode_bytes<const N: usize>(value: &[u8; N]) -> [u8; N] {
    *value
}

/// Decodes exactly `N` bytes from the start of `input`.
pub fn decode_bytes<const N: usize>(input: &mut &[u8]) -> Result<[u8; N], SerializationError> {
    take_bytes(input)
}

#[cfg(test)]
mod tests {
    use super::{decode_bytes, encode_bytes};
    use crate::SerializationError;

    const EMPTY: [u8; 0] = [];
    const ONE: [u8; 1] = [0xa5];
    const FOUR: [u8; 4] = [0x00, 0x7f, 0x80, 0xff];
    const THIRTY_TWO: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ];

    #[test]
    fn encodes_literal_vectors_byte_for_byte_at_exact_width() {
        assert_eq!(encode_bytes(&EMPTY), EMPTY);
        assert_eq!(encode_bytes(&EMPTY).len(), 0);

        assert_eq!(encode_bytes(&ONE), ONE);
        assert_eq!(encode_bytes(&ONE).len(), 1);

        assert_eq!(encode_bytes(&FOUR), FOUR);
        assert_eq!(encode_bytes(&FOUR).len(), 4);

        assert_eq!(encode_bytes(&THIRTY_TWO), THIRTY_TWO);
        assert_eq!(encode_bytes(&THIRTY_TWO).len(), 32);
    }

    #[test]
    fn zero_length_decoding_succeeds_without_consuming_input() {
        let bytes = [0xaa, 0x55];
        let mut input = bytes.as_slice();
        let original = input;

        assert_eq!(decode_bytes::<0>(&mut input), Ok(EMPTY));
        assert_eq!(input, original);
    }

    #[test]
    fn decodes_literal_vectors_and_leaves_trailing_bytes_untouched() {
        assert_decodes_with_trailing_bytes(ONE);
        assert_decodes_with_trailing_bytes(FOUR);
        assert_decodes_with_trailing_bytes(THIRTY_TWO);
    }

    #[test]
    fn rejects_every_truncated_positive_width_without_advancing_cursor() {
        assert_rejects_all_truncations(&ONE);
        assert_rejects_all_truncations(&FOUR);
        assert_rejects_all_truncations(&THIRTY_TWO);
    }

    fn assert_decodes_with_trailing_bytes<const N: usize>(expected: [u8; N]) {
        let mut bytes = Vec::from(expected);
        bytes.extend_from_slice(&[0xaa, 0x55]);
        let mut input = bytes.as_slice();

        assert_eq!(decode_bytes::<N>(&mut input), Ok(expected));
        assert_eq!(input, &[0xaa, 0x55]);
    }

    fn assert_rejects_all_truncations<const N: usize>(bytes: &[u8; N]) {
        for length in 0..N {
            let mut input = &bytes[..length];
            let original = input;

            assert_eq!(
                decode_bytes::<N>(&mut input),
                Err(SerializationError::UnexpectedEof),
                "length {length} must be rejected for Bytes<{N}>"
            );
            assert_eq!(
                input, original,
                "length {length} must not advance the cursor for Bytes<{N}>"
            );
        }
    }
}
