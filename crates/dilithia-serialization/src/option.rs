//! Canonical `Option<T>` encoding and decoding.

use crate::SerializationError;

/// Encodes an `Option<T>` using the canonical tag followed by encoded `T`.
pub fn encode_option<T, F, B, E>(value: Option<T>, encode_value: F) -> Result<Vec<u8>, E>
where
    F: FnOnce(T) -> Result<B, E>,
    B: AsRef<[u8]>,
{
    let Some(value) = value else {
        return Ok(vec![0x00]);
    };

    let encoded_value = encode_value(value)?;
    let mut encoded = vec![0x01];
    encoded.extend_from_slice(encoded_value.as_ref());
    Ok(encoded)
}

/// Decodes an `Option<T>` using the supplied canonical decoder for `T`.
///
/// The caller's input is advanced only after the complete option decodes
/// successfully. It remains unchanged for every error.
pub fn decode_option<T, F>(
    input: &mut &[u8],
    decode_value: F,
) -> Result<Option<T>, SerializationError>
where
    F: FnOnce(&mut &[u8]) -> Result<T, SerializationError>,
{
    let mut cursor = *input;
    let (&tag, remaining) = cursor
        .split_first()
        .ok_or(SerializationError::UnexpectedEof)?;
    cursor = remaining;

    let value = match tag {
        0x00 => None,
        0x01 => Some(decode_value(&mut cursor)?),
        _ => return Err(SerializationError::InvalidOptionTag),
    };

    *input = cursor;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use core::{cell::Cell, convert::Infallible};

    use super::{decode_option, encode_option};
    use crate::{
        SerializationError, decode_bool, decode_bytes, decode_u8, decode_u64, encode_bool,
        encode_bytes, encode_u8, encode_u64,
    };

    #[derive(Debug, PartialEq, Eq)]
    enum TestEncodeError {
        Rejected,
    }

    #[test]
    fn encodes_none_without_invoking_nested_encoder() {
        let encoded = encode_option::<u8, _, [u8; 1], Infallible>(None, |_| {
            panic!("nested encoder must not be invoked for None")
        })
        .expect("infallible encoding must succeed");

        assert_eq!(encoded, [0x00]);
    }

    #[test]
    fn encodes_some_values_with_literal_canonical_vectors() {
        let encoded_true =
            encode_option(Some(true), |value| Ok::<_, Infallible>(encode_bool(value)))
                .expect("infallible encoding must succeed");
        assert_eq!(encoded_true, [0x01, 0x01]);

        let encoded_false =
            encode_option(Some(false), |value| Ok::<_, Infallible>(encode_bool(value)))
                .expect("infallible encoding must succeed");
        assert_eq!(encoded_false, [0x01, 0x00]);

        let encoded_u8 =
            encode_option(Some(0xa5_u8), |value| Ok::<_, Infallible>(encode_u8(value)))
                .expect("infallible encoding must succeed");
        assert_eq!(encoded_u8, [0x01, 0xa5]);

        let encoded_u64 = encode_option(Some(0x0123_4567_89ab_cdef_u64), |value| {
            Ok::<_, Infallible>(encode_u64(value))
        })
        .expect("infallible encoding must succeed");
        assert_eq!(
            encoded_u64,
            [0x01, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]
        );

        let value = [0xde, 0xad, 0xbe, 0xef];
        let encoded_bytes = encode_option(Some(&value), |value| {
            Ok::<_, Infallible>(encode_bytes(value))
        })
        .expect("infallible encoding must succeed");
        assert_eq!(encoded_bytes, [0x01, 0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn encodes_nested_options_with_literal_canonical_vectors() {
        let encoded_none = encode_option(Some(None::<u8>), |value| {
            encode_option(value, |value| Ok::<_, Infallible>(encode_u8(value)))
        })
        .expect("infallible encoding must succeed");
        assert_eq!(encoded_none, [0x01, 0x00]);

        let encoded_some = encode_option(Some(Some(0xa5_u8)), |value| {
            encode_option(value, |value| Ok::<_, Infallible>(encode_u8(value)))
        })
        .expect("infallible encoding must succeed");
        assert_eq!(encoded_some, [0x01, 0x01, 0xa5]);
    }

    #[test]
    fn propagates_nested_encoder_error_after_exactly_one_call() {
        let calls = Cell::new(0);
        let result = encode_option(Some(0xa5_u8), |value| {
            calls.set(calls.get() + 1);
            assert_eq!(value, 0xa5);
            Err::<[u8; 1], _>(TestEncodeError::Rejected)
        });

        assert_eq!(result, Err(TestEncodeError::Rejected));
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn decodes_none_and_leaves_trailing_bytes_untouched() {
        let bytes = [0x00, 0xaa, 0x55];
        let mut input = bytes.as_slice();

        let decoded = decode_option::<u8, _>(&mut input, |_| {
            panic!("nested decoder must not be invoked for None")
        });

        assert_eq!(decoded, Ok(None));
        assert_eq!(input, &[0xaa, 0x55]);
    }

    #[test]
    fn decodes_some_values_and_leaves_trailing_bytes_untouched() {
        let u8_bytes = [0x01, 0xa5, 0xaa, 0x55];
        let mut u8_input = u8_bytes.as_slice();
        assert_eq!(decode_option(&mut u8_input, decode_u8), Ok(Some(0xa5)));
        assert_eq!(u8_input, &[0xaa, 0x55]);

        let u64_bytes = [
            0x01, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01, 0xaa, 0x55,
        ];
        let mut u64_input = u64_bytes.as_slice();
        assert_eq!(
            decode_option(&mut u64_input, decode_u64),
            Ok(Some(0x0123_4567_89ab_cdef))
        );
        assert_eq!(u64_input, &[0xaa, 0x55]);

        let bytes4 = [0x01, 0xde, 0xad, 0xbe, 0xef, 0xaa, 0x55];
        let mut bytes4_input = bytes4.as_slice();
        assert_eq!(
            decode_option(&mut bytes4_input, decode_bytes::<4>),
            Ok(Some([0xde, 0xad, 0xbe, 0xef]))
        );
        assert_eq!(bytes4_input, &[0xaa, 0x55]);

        let bool_bytes = [0x01, 0x01, 0xaa, 0x55];
        let mut bool_input = bool_bytes.as_slice();
        assert_eq!(decode_option(&mut bool_input, decode_bool), Ok(Some(true)));
        assert_eq!(bool_input, &[0xaa, 0x55]);
    }

    #[test]
    fn rejects_empty_input_without_advancing_cursor() {
        let bytes = [];
        let mut input = bytes.as_slice();
        let original = input;

        assert_eq!(
            decode_option(&mut input, decode_u8),
            Err(SerializationError::UnexpectedEof)
        );
        assert_eq!(input, original);
    }

    #[test]
    fn rejects_every_invalid_tag_without_advancing_cursor() {
        for tag in 0x02..=0xff {
            let bytes = [tag, 0xaa, 0x55];
            let mut input = bytes.as_slice();
            let original = input;

            let result = decode_option::<u8, _>(&mut input, |_| {
                panic!("nested decoder must not be invoked for an invalid tag")
            });

            assert_eq!(
                result,
                Err(SerializationError::InvalidOptionTag),
                "tag {tag:#04x} must be rejected"
            );
            assert_eq!(
                input, original,
                "tag {tag:#04x} must not advance the cursor"
            );
        }
    }

    #[test]
    fn rolls_back_outer_cursor_for_every_nested_decoder_error() {
        let truncated_u64 = [0x01, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23];
        assert_nested_error_unchanged(
            &truncated_u64,
            SerializationError::UnexpectedEof,
            decode_u64,
        );

        let invalid_bool = [0x01, 0x02, 0xaa];
        assert_nested_error_unchanged(&invalid_bool, SerializationError::InvalidBool, decode_bool);

        let truncated_bytes = [0x01, 0xde, 0xad, 0xbe];
        assert_nested_error_unchanged(
            &truncated_bytes,
            SerializationError::UnexpectedEof,
            decode_bytes::<4>,
        );

        let invalid_inner_option = [0x01, 0x02, 0xaa];
        let mut input = invalid_inner_option.as_slice();
        let original = input;
        assert_eq!(
            decode_option(&mut input, |input| { decode_option(input, decode_u8) }),
            Err(SerializationError::InvalidOptionTag)
        );
        assert_eq!(input, original);

        let locally_advanced = [0x01, 0xa5, 0xaa];
        let mut input = locally_advanced.as_slice();
        let original = input;
        assert_eq!(
            decode_option(&mut input, |input| {
                *input = &input[1..];
                Err::<u8, _>(SerializationError::UnexpectedEof)
            }),
            Err(SerializationError::UnexpectedEof)
        );
        assert_eq!(input, original);
    }

    #[test]
    fn decodes_nested_options_and_leaves_trailing_bytes_untouched() {
        let nested_none = [0x01, 0x00, 0xaa, 0x55];
        let mut none_input = nested_none.as_slice();
        assert_eq!(
            decode_option(&mut none_input, |input| { decode_option(input, decode_u8) }),
            Ok(Some(None))
        );
        assert_eq!(none_input, &[0xaa, 0x55]);

        let nested_some = [0x01, 0x01, 0xa5, 0xaa, 0x55];
        let mut some_input = nested_some.as_slice();
        assert_eq!(
            decode_option(&mut some_input, |input| { decode_option(input, decode_u8) }),
            Ok(Some(Some(0xa5)))
        );
        assert_eq!(some_input, &[0xaa, 0x55]);
    }

    fn assert_nested_error_unchanged<T: core::fmt::Debug + PartialEq>(
        bytes: &[u8],
        expected: SerializationError,
        decode_value: impl FnOnce(&mut &[u8]) -> Result<T, SerializationError>,
    ) {
        let mut input = bytes;
        let original = input;

        assert_eq!(decode_option(&mut input, decode_value), Err(expected));
        assert_eq!(input, original);
    }
}
