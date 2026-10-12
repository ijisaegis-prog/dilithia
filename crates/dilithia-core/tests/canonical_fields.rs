use dilithia_core::CanonicalConsensusField;
use dilithia_serialization::{self as dcs, U256, UnixTimestamp};
use std::convert::Infallible;

fn equivalent<T: CanonicalConsensusField>(value: &T, expected: &[u8]) {
    let mut actual = vec![0xa5, 0x5a];
    value.append_canonical_field(&mut actual);
    assert_eq!(&actual[..2], [0xa5, 0x5a]);
    assert_eq!(&actual[2..], expected);
}

macro_rules! unsigned_vectors {
    ($name:ident, $ty:ty, $encode:path) => {
        #[test]
        fn $name() {
            for value in [0, 1, <$ty>::MAX / 2, <$ty>::MAX / 2 + 1, <$ty>::MAX] {
                equivalent(&value, &$encode(value));
            }
        }
    };
}

unsigned_vectors!(u8_equivalence, u8, dcs::encode_u8);
unsigned_vectors!(u16_equivalence, u16, dcs::encode_u16);
unsigned_vectors!(u32_equivalence, u32, dcs::encode_u32);
unsigned_vectors!(u64_equivalence, u64, dcs::encode_u64);
unsigned_vectors!(u128_equivalence, u128, dcs::encode_u128);
#[test]
fn unix_timestamp_equivalence() {
    for millis in [0, 1, u64::MAX / 2, u64::MAX / 2 + 1, u64::MAX] {
        let timestamp = UnixTimestamp::from_millis(millis);
        let encoded = dcs::encode_unix_timestamp(timestamp);
        equivalent(&timestamp, &encoded);
        assert_eq!(encoded, dcs::encode_u64(millis));
    }
}

#[test]
fn u256_equivalence() {
    for bytes in [[0; 32], [0xff; 32], std::array::from_fn(|i| i as u8)] {
        let value = U256::from_le_bytes(bytes);
        equivalent(&value, &dcs::encode_u256(&value));
    }
    for bit in 0..256 {
        let mut bytes = [0; 32];
        bytes[bit / 8] = 1 << (bit % 8);
        let value = U256::from_le_bytes(bytes);
        equivalent(&value, &dcs::encode_u256(&value));
    }
}

#[test]
fn bool_and_byte_array_equivalence() {
    for value in [false, true] {
        equivalent(&value, &dcs::encode_bool(value));
    }
    equivalent(&[], &dcs::encode_bytes(&[]));
    equivalent(&[0], &dcs::encode_bytes(&[0]));
    equivalent(&[0xff], &dcs::encode_bytes(&[0xff]));
    equivalent(
        &[0, 0x12, 0xff, 0x80],
        &dcs::encode_bytes(&[0, 0x12, 0xff, 0x80]),
    );
}

fn option_equivalent<T: CanonicalConsensusField + Clone, B: AsRef<[u8]>>(
    values: &[T],
    encode: impl Fn(T) -> B,
) {
    for value in std::iter::once(None).chain(values.iter().cloned().map(Some)) {
        let expected = dcs::encode_option(value.clone(), |v| Ok::<_, Infallible>(encode(v)))
            .expect("infallible DCS field encoder");
        equivalent(&value, &expected);
    }
}

#[test]
fn option_equivalence_for_every_field_family() {
    option_equivalent(&[0_u8, u8::MAX], dcs::encode_u8);
    option_equivalent(&[0_u16, u16::MAX], dcs::encode_u16);
    option_equivalent(&[0_u32, u32::MAX], dcs::encode_u32);
    option_equivalent(&[0_u64, u64::MAX], dcs::encode_u64);
    option_equivalent(&[0_u128, u128::MAX], dcs::encode_u128);
    option_equivalent(
        &[
            UnixTimestamp::from_millis(0),
            UnixTimestamp::from_millis(u64::MAX),
        ],
        dcs::encode_unix_timestamp,
    );
    option_equivalent(&[false, true], dcs::encode_bool);
    option_equivalent(&[U256::from_le_bytes([0xff; 32])], |v| dcs::encode_u256(&v));
    option_equivalent(&[[]], |v| dcs::encode_bytes(&v));
    option_equivalent(&[[0, 0xff]], |v| dcs::encode_bytes(&v));
}

#[test]
fn nested_option_equivalence_and_literal_vectors() {
    for (value, literal) in [
        (None, vec![0]),
        (Some(None), vec![1, 0]),
        (Some(Some(false)), vec![1, 1, 0]),
        (Some(Some(true)), vec![1, 1, 1]),
    ] {
        let expected = dcs::encode_option(value, |inner| {
            dcs::encode_option(inner, |v| Ok::<_, Infallible>(dcs::encode_bool(v)))
        })
        .expect("infallible nested option encoder");
        assert_eq!(expected, literal);
        equivalent(&value, &expected);
    }
}

#[test]
fn nested_u64_option_equivalence_and_literal_vectors() {
    for (value, literal) in [
        (None, vec![0]),
        (Some(None), vec![1, 0]),
        (Some(Some(0_u64)), vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
        (
            Some(Some(0x0123_4567_89ab_cdef_u64)),
            vec![1, 1, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01],
        ),
    ] {
        let expected = dcs::encode_option(value, |inner| {
            dcs::encode_option(inner, |v| Ok::<_, Infallible>(dcs::encode_u64(v)))
        })
        .expect("infallible nested u64 option encoder");
        assert_eq!(expected, literal);
        equivalent(&value, &expected);
    }
}
