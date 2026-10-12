use dilithia_serialization::{
    DecoderInput, SerializationError, decode_bool, decode_bytes, decode_exact, decode_option,
    decode_u8,
};

#[test]
fn ordinary_public_decoder_calls_remain_direct() {
    let bytes = [0xa5, 0x01, 0x2a];
    let mut input = bytes.as_slice();

    assert_eq!(decode_u8(&mut input), Ok(0xa5));
    assert_eq!(decode_option(&mut input, decode_u8), Ok(Some(0x2a)));
    assert!(input.is_empty());

    let mut bool_input = [0x01].as_slice();
    assert_eq!(decode_exact(&mut bool_input, decode_bool), Ok(true));
    assert!(bool_input.is_empty());
}

#[test]
fn public_callback_can_only_make_checked_forward_progress() {
    let bytes = [0xa5];
    let mut input = bytes.as_slice();

    let decoded = decode_exact(&mut input, |cursor| {
        let value = *cursor
            .remaining()
            .first()
            .ok_or(SerializationError::UnexpectedEof)?;
        cursor.advance(1)?;
        Ok(value)
    });

    assert_eq!(decoded, Ok(0xa5));
    assert!(input.is_empty());
}

#[test]
fn exact_zero_width_values_reject_trailing_bytes_without_advancing() {
    for bytes in [&[0xaa][..], &[0xaa, 0xbb][..]] {
        let mut input = bytes;
        assert_eq!(
            decode_exact(&mut input, decode_bytes::<0>),
            Err(SerializationError::TrailingBytes)
        );
        assert_eq!(input, bytes);
    }

    let bytes = [0x01];
    let mut input = bytes.as_slice();
    assert_eq!(
        decode_exact(&mut input, |cursor| {
            decode_option(cursor, decode_bytes::<0>)
        }),
        Ok(Some([]))
    );
    assert!(input.is_empty());

    let bytes = [0x01, 0xaa];
    let mut input = bytes.as_slice();
    assert_eq!(
        decode_exact(&mut input, |cursor| {
            decode_option(cursor, decode_bytes::<0>)
        }),
        Err(SerializationError::TrailingBytes)
    );
    assert_eq!(input, bytes);
}

#[test]
fn slice_and_callback_cursors_check_all_advance_boundaries() {
    fn check(cursor: &mut dyn DecoderInput) -> Result<(), SerializationError> {
        let original = cursor.remaining().to_vec();
        assert_eq!(cursor.advance(0), Ok(()));
        assert_eq!(cursor.remaining(), original);

        for count in [original.len() + 1, usize::MAX] {
            assert_eq!(
                cursor.advance(count),
                Err(SerializationError::UnexpectedEof)
            );
            assert_eq!(cursor.remaining(), original);
        }

        if !original.is_empty() {
            cursor.advance(1)?;
            let suffix = &original[1..];
            assert_eq!(cursor.advance(0), Ok(()));
            assert_eq!(cursor.remaining(), suffix);
            // At a nonzero offset, MAX also exercises checked-add overflow.
            for count in [suffix.len() + 1, usize::MAX] {
                assert_eq!(
                    cursor.advance(count),
                    Err(SerializationError::UnexpectedEof)
                );
                assert_eq!(cursor.remaining(), suffix);
            }
        }

        let remaining = cursor.remaining().len();
        assert_eq!(cursor.advance(remaining), Ok(()));
        assert!(cursor.remaining().is_empty());
        assert_eq!(cursor.advance(0), Ok(()));
        for count in [1, usize::MAX] {
            assert_eq!(
                cursor.advance(count),
                Err(SerializationError::UnexpectedEof)
            );
            assert!(cursor.remaining().is_empty());
        }
        Ok(())
    }

    let bytes = [0xaa, 0xbb, 0xcc];
    for source in [&bytes[..0], bytes.as_slice()] {
        let mut direct = source;
        assert_eq!(check(&mut direct), Ok(()));
        assert!(direct.is_empty());

        let mut exact = source;
        assert_eq!(decode_exact(&mut exact, check), Ok(()));
        assert!(exact.is_empty());

        let mut tagged = vec![0x01];
        tagged.extend_from_slice(source);
        let mut option = tagged.as_slice();
        assert_eq!(decode_option(&mut option, check), Ok(Some(())));
        assert!(option.is_empty());
    }
}

#[test]
fn nested_failures_restore_the_parent_cursor_before_recovery() {
    let bytes = [0x7e, 0x01, 0x01, 0xaa, 0xbb];
    let mut input = bytes.as_slice();

    let result = decode_exact(&mut input, |outer| {
        assert_eq!(decode_u8(outer)?, 0x7e);
        let before_nested = outer.remaining().to_vec();
        assert_eq!(
            decode_option(outer, |middle| {
                assert_eq!(middle.remaining(), &[0x01, 0xaa, 0xbb]);
                let failure = decode_option(middle, |inner| {
                    assert_eq!(decode_u8(inner)?, 0xaa);
                    inner.advance(usize::MAX)
                });
                assert_eq!(failure, Err(SerializationError::UnexpectedEof));
                assert_eq!(middle.remaining(), &[0x01, 0xaa, 0xbb]);
                failure
            }),
            Err(SerializationError::UnexpectedEof)
        );
        assert_eq!(outer.remaining(), before_nested);

        assert_eq!(
            decode_exact(outer, |cursor| {
                decode_option(cursor, |cursor| decode_option(cursor, decode_u8))
            }),
            Err(SerializationError::TrailingBytes)
        );
        assert_eq!(outer.remaining(), before_nested);

        // Both failures can be recovered from using the same parent cursor.
        assert_eq!(
            decode_option(outer, |cursor| decode_option(cursor, decode_u8))?,
            Some(Some(0xaa))
        );
        decode_u8(outer)
    });

    assert_eq!(result, Ok(0xbb));
    assert!(input.is_empty());
}

#[test]
fn outer_failure_rolls_back_successfully_committed_nested_decoders() {
    let bytes = [0x7e, 0x01, 0x01, 0xaa];
    let mut input = bytes.as_slice();

    let result = decode_exact(&mut input, |outer| {
        assert_eq!(decode_u8(outer)?, 0x7e);
        assert_eq!(
            decode_option(outer, |cursor| decode_option(cursor, decode_u8))?,
            Some(Some(0xaa))
        );
        assert!(outer.remaining().is_empty());
        outer.advance(1)
    });

    assert_eq!(result, Err(SerializationError::UnexpectedEof));
    assert_eq!(input, bytes);
}
