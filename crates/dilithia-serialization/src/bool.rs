//! Canonical Bool encoding and decoding.

use crate::SerializationError;

/// Encodes a Bool using its canonical one-byte representation.
pub fn encode_bool(value: bool) -> [u8; 1] {
    [u8::from(value)]
}

/// Decodes a canonically encoded Bool from the start of `input`.
pub fn decode_bool(input: &mut &[u8]) -> Result<bool, SerializationError> {
    let original = *input;
    let byte = *original.first().ok_or(SerializationError::UnexpectedEof)?;

    let value = match byte {
        0x00 => false,
        0x01 => true,
        _ => return Err(SerializationError::InvalidBool),
    };

    *input = &original[1..];
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{decode_bool, encode_bool};
    use crate::SerializationError;

    #[test]
    fn encodes_canonical_values() {
        assert_eq!(encode_bool(false), [0x00]);
        assert_eq!(encode_bool(true), [0x01]);
        assert_eq!(encode_bool(false).len(), 1);
        assert_eq!(encode_bool(true).len(), 1);
    }

    #[test]
    fn decodes_canonical_values_and_consumes_one_byte() {
        let false_bytes = [0x00, 0xaa, 0x55];
        let mut false_input = false_bytes.as_slice();
        assert_eq!(decode_bool(&mut false_input), Ok(false));
        assert_eq!(false_input, &[0xaa, 0x55]);

        let true_bytes = [0x01, 0xaa, 0x55];
        let mut true_input = true_bytes.as_slice();
        assert_eq!(decode_bool(&mut true_input), Ok(true));
        assert_eq!(true_input, &[0xaa, 0x55]);
    }

    #[test]
    fn rejects_empty_input_without_advancing_cursor() {
        let bytes = [];
        let mut input = bytes.as_slice();
        let original = input;

        assert_eq!(
            decode_bool(&mut input),
            Err(SerializationError::UnexpectedEof)
        );
        assert_eq!(input, original);
    }

    #[test]
    fn rejects_every_non_canonical_byte_without_advancing_cursor() {
        for byte in 0x02..=0xff {
            let bytes = [byte, 0xaa, 0x55];
            let mut input = bytes.as_slice();
            let original = input;

            assert_eq!(
                decode_bool(&mut input),
                Err(SerializationError::InvalidBool),
                "byte {byte:#04x} must be rejected"
            );
            assert_eq!(
                input, original,
                "byte {byte:#04x} must not advance the cursor"
            );
        }
    }
}
