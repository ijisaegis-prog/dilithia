//! Canonical unsigned LEB128 encoding and decoding.

use crate::{DecoderInput, SerializationError};

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
pub fn decode_uleb128_u64(input: &mut dyn DecoderInput) -> Result<u64, SerializationError> {
    let original = input.remaining();
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

            input.advance(index + 1)?;
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

    // Boundary neighbors cover every encoded width; the fixed xorshift stream
    // supplies varied payloads without a random seed or an external dependency.
    fn deterministic_values() -> Vec<u64> {
        let mut values: Vec<u64> = (0..=255).collect();
        values.extend(VECTORS.iter().map(|&(value, _)| value));
        for bit in 0..64 {
            let boundary = 1_u64 << bit;
            values.extend([boundary - 1, boundary, boundary + 1]);
        }
        let mut state = 0x4d59_5df4_d0f3_3173_u64;
        for _ in 0..128 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            values.push(state);
        }
        values.sort_unstable();
        values.dedup();
        values
    }

    #[test]
    fn deterministic_values_round_trip_and_reject_every_truncation() {
        for value in deterministic_values() {
            let encoded = encode_uleb128_u64(value);
            let mut input = encoded.as_slice();
            let decoded = decode_uleb128_u64(&mut input).expect("canonical sample");
            assert_eq!(decoded, value);
            assert!(input.is_empty());
            assert_eq!(encode_uleb128_u64(decoded), encoded);

            for end in 0..encoded.len() {
                assert_error_unchanged(&encoded[..end], SerializationError::UnexpectedEof);
            }
        }
    }

    #[test]
    fn canonical_prefix_ignores_arbitrary_trailing_bytes() {
        for value in deterministic_values() {
            let encoded = encode_uleb128_u64(value);
            for byte in 0..=u8::MAX {
                // Include bytes that would be malformed as a separate ULEB value.
                let suffix = [byte, 0x80, 0x00, byte ^ 0xff];
                let mut bytes = encoded.clone();
                bytes.extend_from_slice(&suffix);
                let mut input = bytes.as_slice();

                assert_eq!(decode_uleb128_u64(&mut input), Ok(value));
                assert_eq!(input, suffix);
                assert_eq!(encode_uleb128_u64(value), bytes[..encoded.len()]);
            }
        }
    }

    #[test]
    fn continuation_bit_mutations_follow_canonical_prefix_rules() {
        for value in deterministic_values() {
            let encoded = encode_uleb128_u64(value);
            for index in 0..encoded.len() {
                let mut mutated = encoded.clone();
                mutated[index] ^= 0x80;

                if index + 1 < encoded.len() {
                    if index != 0 && mutated[index] == 0 {
                        assert_error_unchanged(&mutated, SerializationError::NonCanonicalUleb128);
                    } else {
                        // Clearing an interior continuation bit may produce a
                        // valid shorter prefix, with the original suffix intact.
                        let expected = value & ((1_u64 << (7 * (index + 1))) - 1);
                        let mut input = mutated.as_slice();
                        assert_eq!(decode_uleb128_u64(&mut input), Ok(expected));
                        assert_eq!(input, &mutated[index + 1..]);
                        assert_eq!(encode_uleb128_u64(expected), mutated[..=index]);
                    }
                } else if encoded.len() == 10 {
                    assert_error_unchanged(&mutated, SerializationError::Uleb128Overflow);
                } else {
                    assert_error_unchanged(&mutated, SerializationError::UnexpectedEof);
                    // A nonzero next group makes the extended value canonical.
                    mutated.push(0x01);
                    let expected = value | (1_u64 << (7 * encoded.len()));
                    let mut input = mutated.as_slice();
                    assert_eq!(decode_uleb128_u64(&mut input), Ok(expected));
                    assert!(input.is_empty());
                    assert_eq!(encode_uleb128_u64(expected), mutated);
                }
            }
        }
    }

    #[test]
    fn zero_terminal_payload_mutations_are_nonminimal() {
        for value in deterministic_values() {
            let encoded = encode_uleb128_u64(value);
            if encoded.len() > 1 {
                let mut mutated = encoded.clone();
                *mutated.last_mut().unwrap() = 0;
                assert_error_unchanged(&mutated, SerializationError::NonCanonicalUleb128);
            }

            if encoded.len() < 10 {
                let mut extended = encoded;
                *extended.last_mut().unwrap() |= 0x80;
                extended.push(0);
                assert_error_unchanged(&extended, SerializationError::NonCanonicalUleb128);
            }
        }
    }

    #[test]
    fn every_tenth_byte_obeys_u64_payload_and_continuation_boundaries() {
        for (prefix, lower_bits) in [([0x80; 9], 0), ([0xff; 9], u64::MAX >> 1)] {
            for tenth in 0..=u8::MAX {
                let mut bytes = prefix.to_vec();
                bytes.push(tenth);
                let suffix = [0x00, 0x01, 0x80, 0xff];
                bytes.extend_from_slice(&suffix);

                match tenth {
                    0 => assert_error_unchanged(&bytes, SerializationError::NonCanonicalUleb128),
                    1 => {
                        let expected = lower_bits | (1_u64 << 63);
                        let mut input = bytes.as_slice();
                        assert_eq!(decode_uleb128_u64(&mut input), Ok(expected));
                        assert_eq!(input, suffix);
                        assert_eq!(encode_uleb128_u64(expected), bytes[..10]);
                    }
                    _ => assert_error_unchanged(&bytes, SerializationError::Uleb128Overflow),
                }
            }
        }
    }

    #[test]
    fn more_than_ten_continuation_bytes_overflow_even_with_a_later_terminator() {
        // These lengths are test samples, not resource limits: u64 overflow
        // already occurs at the tenth continuation byte.
        for length in [11, 12, 16, 32] {
            for continuation in [0x80, 0xff] {
                let bytes = vec![continuation; length];
                assert_error_unchanged(&bytes, SerializationError::Uleb128Overflow);
                for terminal in [0x00, 0x01, 0x7f] {
                    let mut terminated = bytes.clone();
                    terminated.push(terminal);
                    assert_error_unchanged(&terminated, SerializationError::Uleb128Overflow);
                }
            }
        }
    }

    fn assert_error_unchanged(bytes: &[u8], expected: SerializationError) {
        let mut input = bytes;
        let original = input;

        assert_eq!(
            decode_uleb128_u64(&mut input),
            Err(expected),
            "{bytes:02x?}"
        );
        assert_eq!(input, original, "{bytes:02x?}");
    }
}
