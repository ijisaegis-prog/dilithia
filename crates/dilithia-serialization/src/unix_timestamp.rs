//! Canonical Unix timestamp encoding and decoding.

use crate::{DecoderInput, SerializationError, decode_u64, encode_u64};

/// Milliseconds since the Unix epoch, UTC, with canonical `u64` wire encoding.
///
/// This type only establishes the DCS representation and unit. It does not
/// impose protocol-level clock, range, or freshness validity rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnixTimestamp(u64);

impl UnixTimestamp {
    /// Constructs a timestamp from milliseconds since the Unix epoch.
    pub const fn from_millis(value: u64) -> Self {
        Self(value)
    }

    /// Returns the represented milliseconds since the Unix epoch.
    pub const fn as_millis(self) -> u64 {
        self.0
    }
}

/// Encodes a Unix timestamp using canonical `u64` encoding.
pub fn encode_unix_timestamp(value: UnixTimestamp) -> [u8; 8] {
    encode_u64(value.as_millis())
}

/// Decodes a Unix timestamp using canonical `u64` encoding.
pub fn decode_unix_timestamp(
    input: &mut dyn DecoderInput,
) -> Result<UnixTimestamp, SerializationError> {
    decode_u64(input).map(UnixTimestamp::from_millis)
}

#[cfg(test)]
mod tests {
    use super::{UnixTimestamp, decode_unix_timestamp, encode_unix_timestamp};
    use crate::{SerializationError, encode_u64};

    const VECTORS: &[(u64, [u8; 8])] = &[
        (0, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (1, [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (1_000, [0xe8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (u64::MAX, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
    ];

    #[test]
    fn encodes_fixed_vectors_at_exact_width() {
        for &(millis, expected) in VECTORS {
            let timestamp = UnixTimestamp::from_millis(millis);
            let encoded = encode_unix_timestamp(timestamp);

            assert_eq!(encoded, expected, "value {millis}");
            assert_eq!(encoded, encode_u64(millis));
            assert_eq!(encoded.len(), 8);
        }
    }

    #[test]
    fn decodes_fixed_vectors_and_leaves_trailing_bytes_untouched() {
        for &(millis, encoded) in VECTORS {
            let mut bytes = Vec::from(encoded);
            bytes.extend_from_slice(&[0xaa, 0x55]);
            let mut input = bytes.as_slice();

            assert_eq!(
                decode_unix_timestamp(&mut input),
                Ok(UnixTimestamp::from_millis(millis))
            );
            assert_eq!(input, &[0xaa, 0x55]);
        }
    }

    #[test]
    fn millis_roundtrip_preserves_all_bits() {
        for &(millis, _) in VECTORS {
            assert_eq!(UnixTimestamp::from_millis(millis).as_millis(), millis);
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
