//! Complete-object decoding over composable prefix decoders.

use crate::{DecoderInput, SerializationError, cursor::transactional};

/// Decodes one complete object using the supplied prefix decoder.
///
/// DCS primitive and composed decoders consume one value from the start of
/// their input so that they remain composable. Use this boundary when `input`
/// is the complete encoding of one object: success requires the inner decoder
/// to consume every byte.
///
/// The inner decoder runs against a private source-and-offset cursor. The
/// caller's input is advanced to the empty remainder only on success and
/// remains unchanged if the inner decoder fails or leaves trailing bytes.
pub fn decode_exact<T, F>(input: &mut dyn DecoderInput, decode: F) -> Result<T, SerializationError>
where
    F: FnOnce(&mut dyn DecoderInput) -> Result<T, SerializationError>,
{
    transactional(input, |cursor| {
        let value = decode(cursor)?;
        if !cursor.remaining().is_empty() {
            return Err(SerializationError::TrailingBytes);
        }
        Ok(value)
    })
}

#[cfg(test)]
mod tests {
    use super::decode_exact;
    use crate::{
        SerializationError, U256, UnixTimestamp, decode_bool, decode_bytes, decode_option,
        decode_u8, decode_u16, decode_u32, decode_u64, decode_u128, decode_u256,
        decode_uleb128_u64, decode_unix_timestamp,
    };

    #[test]
    fn exactly_decodes_bool_and_consumes_complete_input() {
        let bytes = [0x01];
        let mut input = bytes.as_slice();

        assert_eq!(decode_exact(&mut input, decode_bool), Ok(true));
        assert!(input.is_empty());
    }

    #[test]
    fn rejects_trailing_bytes() {
        let bytes = [0x01, 0xaa];
        let mut input = bytes.as_slice();

        assert_eq!(
            decode_exact(&mut input, decode_bool),
            Err(SerializationError::TrailingBytes)
        );
    }

    #[test]
    fn leaves_input_unchanged_when_trailing_bytes_are_rejected() {
        let bytes = [0x01, 0xaa, 0x55];
        let mut input = bytes.as_slice();
        let original = input;

        assert_eq!(
            decode_exact(&mut input, decode_bool),
            Err(SerializationError::TrailingBytes)
        );
        assert_eq!(input, original);
    }

    #[test]
    fn leaves_input_unchanged_when_inner_decoder_fails_after_advancing() {
        let bytes = [0xaa, 0x55];
        let mut input = bytes.as_slice();
        let original = input;

        let result = decode_exact(&mut input, |cursor| {
            cursor.advance(1)?;
            Err::<u8, _>(SerializationError::UnexpectedEof)
        });

        assert_eq!(result, Err(SerializationError::UnexpectedEof));
        assert_eq!(input, original);
    }

    #[test]
    fn exactly_decodes_composed_option() {
        let bytes = [0x01, 0x01];
        let mut input = bytes.as_slice();

        assert_eq!(
            decode_exact(&mut input, |cursor| { decode_option(cursor, decode_bool) }),
            Ok(Some(true))
        );
        assert!(input.is_empty());
    }

    #[test]
    fn composed_option_rejects_trailing_bytes_without_advancing_input() {
        let bytes = [0x01, 0x01, 0xaa];
        let mut input = bytes.as_slice();
        let original = input;

        assert_eq!(
            decode_exact(&mut input, |cursor| { decode_option(cursor, decode_bool) }),
            Err(SerializationError::TrailingBytes)
        );
        assert_eq!(input, original);
    }

    #[test]
    fn empty_input_origins_decode_without_pointer_assumptions() {
        let first = Vec::<u8>::new();
        let second = Vec::<u8>::new();
        let allocated = Vec::<u8>::with_capacity(8);
        let mut cleared = vec![0xaa, 0xbb];
        cleared.clear();
        let backing = [0xaa, 0xbb];

        for mut input in [
            first.as_slice(),
            second.as_slice(),
            allocated.as_slice(),
            cleared.as_slice(),
            &backing[..0],
            &backing[1..1],
            &backing[backing.len()..],
        ] {
            assert_eq!(decode_exact(&mut input, decode_bytes::<0>), Ok([]));
            assert!(input.is_empty());
        }
    }

    #[test]
    fn decoder_error_precedes_trailing_bytes_and_rolls_back() {
        let bytes = [0x01, 0xaa];
        let mut input = bytes.as_slice();
        let original = input;

        let result = decode_exact(&mut input, |cursor| {
            cursor.advance(1)?;
            Err::<bool, _>(SerializationError::InvalidBool)
        });

        assert_eq!(result, Err(SerializationError::InvalidBool));
        assert_eq!(input, original);
    }

    #[test]
    fn truncated_inner_decoder_rolls_back_complete_input() {
        let bytes = [0x34];
        let mut input = bytes.as_slice();
        let original = input;

        assert_eq!(
            decode_exact(&mut input, decode_u16),
            Err(SerializationError::UnexpectedEof)
        );
        assert_eq!(input, original);
    }

    #[test]
    fn exactly_decodes_valid_builtin_boundaries_including_bytes_zero() {
        let mut u8_input = [0xa5].as_slice();
        assert_eq!(decode_exact(&mut u8_input, decode_u8), Ok(0xa5));

        let u16_bytes = 0x1234_u16.to_le_bytes();
        let mut u16_input = u16_bytes.as_slice();
        assert_eq!(decode_exact(&mut u16_input, decode_u16), Ok(0x1234));

        let u32_bytes = 0x1234_5678_u32.to_le_bytes();
        let mut u32_input = u32_bytes.as_slice();
        assert_eq!(decode_exact(&mut u32_input, decode_u32), Ok(0x1234_5678));

        let u64_bytes = 0x0123_4567_89ab_cdef_u64.to_le_bytes();
        let mut u64_input = u64_bytes.as_slice();
        assert_eq!(
            decode_exact(&mut u64_input, decode_u64),
            Ok(0x0123_4567_89ab_cdef)
        );

        let u128_bytes = u128::MAX.to_le_bytes();
        let mut u128_input = u128_bytes.as_slice();
        assert_eq!(decode_exact(&mut u128_input, decode_u128), Ok(u128::MAX));

        let u256_bytes = [0x5a; 32];
        let mut u256_input = u256_bytes.as_slice();
        assert_eq!(
            decode_exact(&mut u256_input, decode_u256),
            Ok(U256::from_le_bytes(u256_bytes))
        );

        let bytes = [0xde, 0xad, 0xbe, 0xef];
        let mut bytes_input = bytes.as_slice();
        assert_eq!(decode_exact(&mut bytes_input, decode_bytes::<4>), Ok(bytes));

        let empty = [];
        let mut empty_input = empty.as_slice();
        assert_eq!(decode_exact(&mut empty_input, decode_bytes::<0>), Ok([]));
        assert!(empty_input.is_empty());

        let timestamp = UnixTimestamp::from_millis(1_700_000_000_000);
        let timestamp_bytes = timestamp.as_millis().to_le_bytes();
        let mut timestamp_input = timestamp_bytes.as_slice();
        assert_eq!(
            decode_exact(&mut timestamp_input, decode_unix_timestamp),
            Ok(timestamp)
        );
    }

    #[test]
    fn enforces_canonical_uleb128_and_complete_consumption() {
        let canonical = [0x80, 0x01];
        let mut canonical_input = canonical.as_slice();
        assert_eq!(
            decode_exact(&mut canonical_input, decode_uleb128_u64),
            Ok(128)
        );

        let noncanonical = [0x80, 0x00];
        let mut noncanonical_input = noncanonical.as_slice();
        let original_noncanonical = noncanonical_input;
        assert_eq!(
            decode_exact(&mut noncanonical_input, decode_uleb128_u64),
            Err(SerializationError::NonCanonicalUleb128)
        );
        assert_eq!(noncanonical_input, original_noncanonical);

        let trailing = [0x80, 0x01, 0xaa];
        let mut trailing_input = trailing.as_slice();
        let original_trailing = trailing_input;
        assert_eq!(
            decode_exact(&mut trailing_input, decode_uleb128_u64),
            Err(SerializationError::TrailingBytes)
        );
        assert_eq!(trailing_input, original_trailing);
    }
}
