use core::ptr;

use dilithia_serialization::{
    DecoderInput, SerializationError, UnixTimestamp, decode_exact, decode_option, decode_u64,
    decode_uleb128_u64, decode_unix_timestamp, encode_u64, encode_uleb128_u64,
    encode_unix_timestamp,
};

type U64Decoder = fn(&mut dyn DecoderInput) -> Result<u64, SerializationError>;

// Compare against a remaining-length model, without adding an untrusted count
// to the offset. Even the largest counts use only a tiny backing allocation.
fn check_advance(
    cursor: &mut dyn DecoderInput,
    source: &[u8],
    offset: usize,
    count: usize,
) -> Result<(), SerializationError> {
    cursor.advance(offset)?;
    let before = &source[offset..];
    assert!(ptr::eq(cursor.remaining(), before));
    let result = cursor.advance(count);
    if count <= before.len() {
        assert_eq!(result, Ok(()));
        assert!(ptr::eq(cursor.remaining(), &before[count..]));
    } else {
        assert_eq!(result, Err(SerializationError::UnexpectedEof));
        assert!(ptr::eq(cursor.remaining(), before));
    }
    result
}

#[test]
fn advance_matches_remaining_length_at_every_pointer_width_boundary() {
    let mut counts: Vec<usize> = (0..=8).collect();
    for bit in 0..usize::BITS {
        let boundary = 1_usize << bit;
        counts.extend([boundary - 1, boundary, boundary + 1]);
    }
    // At nonzero offsets, these straddle checked-add overflow and would wrap
    // to small in-bounds offsets. On 64-bit, the powers above also catch u32
    // narrowing (for example, 2^32 + 1 must never become an advance of 1).
    counts.extend((0..=8).map(|distance| usize::MAX - distance));
    counts.sort_unstable();
    counts.dedup();

    let backing = [0xaa, 0x01, 0x01, 0x01, 0x01, 0x01, 0xbb];
    for start in 1..backing.len() {
        for end in start..backing.len() {
            let source = &backing[start..end];
            for offset in 0..=source.len() {
                for &count in &counts {
                    let fits = count <= source.len() - offset;
                    let expected = if fits {
                        Ok(())
                    } else {
                        Err(SerializationError::UnexpectedEof)
                    };

                    let mut direct = source;
                    assert_eq!(check_advance(&mut direct, source, offset, count), expected);

                    let mut exact = source;
                    let result = decode_exact(&mut exact, |cursor| {
                        check_advance(cursor, source, offset, count)
                    });
                    let complete = fits && count == source.len() - offset;
                    assert_eq!(
                        result,
                        if complete {
                            Ok(())
                        } else if fits {
                            Err(SerializationError::TrailingBytes)
                        } else {
                            expected
                        }
                    );
                    assert!(ptr::eq(
                        exact,
                        if complete {
                            &source[source.len()..]
                        } else {
                            source
                        }
                    ));

                    // The tag adds a hidden nonzero tracked offset, including
                    // when the callback's payload offset is zero.
                    let tagged = &backing[start - 1..end];
                    if tagged[0] == 0x01 {
                        let mut option = tagged;
                        assert_eq!(
                            decode_option(&mut option, |cursor| {
                                check_advance(cursor, source, offset, count)
                            }),
                            expected.map(Some)
                        );
                        assert!(ptr::eq(
                            option,
                            if fits {
                                &source[offset + count..]
                            } else {
                                tagged
                            }
                        ));
                    }
                }
            }
        }
    }
}

#[test]
fn u64_wire_vectors_cross_u32_boundary_without_host_sized_narrowing() {
    // Literal oracles, independent of either encoder and identical on i686
    // and x86_64. These are integer values, not allocation or protocol limits.
    let vectors = [
        (
            0xffff_fffe_u64,
            [0xfe, 0xff, 0xff, 0xff, 0, 0, 0, 0],
            [0xfe, 0xff, 0xff, 0xff, 0x0f],
        ),
        (
            0xffff_ffff,
            [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0],
            [0xff, 0xff, 0xff, 0xff, 0x0f],
        ),
        (
            0x1_0000_0000,
            [0, 0, 0, 0, 1, 0, 0, 0],
            [0x80, 0x80, 0x80, 0x80, 0x10],
        ),
        (
            0x1_0000_0001,
            [1, 0, 0, 0, 1, 0, 0, 0],
            [0x81, 0x80, 0x80, 0x80, 0x10],
        ),
    ];
    for (value, fixed, variable) in vectors {
        assert_eq!(encode_u64(value), fixed);
        let timestamp = UnixTimestamp::from_millis(value);
        assert_eq!(encode_unix_timestamp(timestamp), fixed);
        assert_eq!(encode_uleb128_u64(value), variable);

        let decoders: [(U64Decoder, &[u8]); 2] =
            [(decode_u64, &fixed), (decode_uleb128_u64, &variable)];
        for (decode, bytes) in decoders {
            let mut exact = bytes;
            assert_eq!(decode_exact(&mut exact, decode), Ok(value));
            assert!(ptr::eq(exact, &bytes[bytes.len()..]));
            for end in 0..bytes.len() {
                let original = &bytes[..end];
                let mut truncated = original;
                assert_eq!(
                    decode_exact(&mut truncated, decode),
                    Err(SerializationError::UnexpectedEof)
                );
                assert!(ptr::eq(truncated, original));
            }
        }

        let timestamp_bytes = fixed.as_slice();
        let mut exact_timestamp = timestamp_bytes;
        assert_eq!(
            decode_exact(&mut exact_timestamp, decode_unix_timestamp),
            Ok(timestamp)
        );
        assert!(ptr::eq(
            exact_timestamp,
            &timestamp_bytes[timestamp_bytes.len()..]
        ));
        for end in 0..timestamp_bytes.len() {
            let original = &timestamp_bytes[..end];
            let mut truncated = original;
            assert_eq!(
                decode_exact(&mut truncated, decode_unix_timestamp),
                Err(SerializationError::UnexpectedEof)
            );
            assert!(ptr::eq(truncated, original));
        }
    }
}
