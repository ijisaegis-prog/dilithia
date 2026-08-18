//! Canonical unsigned LEB128 encoding and decoding.

use crate::SerializationError;

/// Encodes a `u64` using its unique minimal ULEB128 representation.
pub fn encode_uleb128_u64(mut value: u64) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(10);

    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        encoded.push(byte);

        if value == 0 {
            return encoded;
        }
    }
}

/// Decodes one canonical ULEB128-encoded `u64`.
///
/// The input is advanced past the decoded value only on success. It is left
/// unchanged for every error.
pub fn decode_uleb128_u64(input: &mut &[u8]) -> Result<u64, SerializationError> {
    let original = *input;
    let mut value = 0_u64;

    for index in 0..10 {
        let byte = *original
            .get(index)
            .ok_or(SerializationError::UnexpectedEof)?;
        let payload = byte & 0x7f;

        if index == 9 && (payload > 1 || byte & 0x80 != 0) {
            return Err(SerializationError::Uleb128Overflow);
        }

        value |= u64::from(payload) << (index * 7);

        if byte & 0x80 == 0 {
            if index != 0 && payload == 0 {
                return Err(SerializationError::NonCanonicalUleb128);
            }

            *input = &original[index + 1..];
            return Ok(value);
        }
    }

    Err(SerializationError::Uleb128Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VECTORS: &[(u64, &[u8])] = &[
        (0, &[0x00]),
        (1, &[0x01]),
        (127, &[0x7f]),
        (128, &[0x80, 0x01]),
        (255, &[0xff, 0x01]),
        (16_383, &[0xff, 0x7f]),
        (16_384, &[0x80, 0x80, 0x01]),
        (624_485, &[0xe5, 0x8e, 0x26]),
        (
            u64::MAX,
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
        ),
    ];

    #[test]
    fn encodes_fixed_conformance_vectors() {
        for &(value, expected) in VECTORS {
            assert_eq!(encode_uleb128_u64(value), expected, "value {value}");
        }
    }

    #[test]
    fn decodes_fixed_conformance_vectors_and_advances_input() {
        for &(expected, encoded) in VECTORS {
            let mut bytes = Vec::from(encoded);
            bytes.push(0xaa);
            let mut input = bytes.as_slice();

            assert_eq!(
                decode_uleb128_u64(&mut input),
                Ok(expected),
                "bytes {encoded:02x?}"
            );
            assert_eq!(input, &[0xaa]);
        }
    }

    #[test]
    fn rejects_non_canonical_encodings_without_advancing_input() {
        assert_error_unchanged(&[0x80, 0x00], SerializationError::NonCanonicalUleb128);
        assert_error_unchanged(&[0x81, 0x00], SerializationError::NonCanonicalUleb128);
    }

    #[test]
    fn rejects_truncated_encoding_without_advancing_input() {
        assert_error_unchanged(&[], SerializationError::UnexpectedEof);
        assert_error_unchanged(&[0x80], SerializationError::UnexpectedEof);
    }

    #[test]
    fn rejects_overflow_without_advancing_input() {
        assert_error_unchanged(
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02],
            SerializationError::Uleb128Overflow,
        );
        assert_error_unchanged(
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x81],
            SerializationError::Uleb128Overflow,
        );
    }

    fn assert_error_unchanged(bytes: &[u8], expected: SerializationError) {
        let mut input = bytes;
        let original = input;

        assert_eq!(decode_uleb128_u64(&mut input), Err(expected));
        assert_eq!(input, original);
    }
}
