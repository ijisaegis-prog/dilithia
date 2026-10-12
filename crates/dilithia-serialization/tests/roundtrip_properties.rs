use core::ptr;

use dilithia_serialization::{
    SerializationError, decode_exact, decode_u8, decode_u16, decode_u32, decode_u64, decode_u128,
    decode_uleb128_u64, encode_u8, encode_u16, encode_u32, encode_u64, encode_u128,
    encode_uleb128_u64,
};

fn reference_uleb128(mut value: u64) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            return out;
        }
    }
}

fn next_u64(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

#[test]
fn fixed_width_integer_roundtrips_match_rust_little_endian_oracles() {
    for value in u8::MIN..=u8::MAX {
        let encoded = encode_u8(value);
        assert_eq!(encoded, value.to_le_bytes());
        let mut input = encoded.as_slice();
        assert_eq!(decode_exact(&mut input, decode_u8), Ok(value));
        assert!(input.is_empty());
    }

    for value in u16::MIN..=u16::MAX {
        let encoded = encode_u16(value);
        assert_eq!(encoded, value.to_le_bytes());
        let mut input = encoded.as_slice();
        assert_eq!(decode_exact(&mut input, decode_u16), Ok(value));
        assert!(input.is_empty());
    }

    let mut state = 0xd1_17_41_a5_5e_ed_12_34_u64;
    for _ in 0..4096 {
        let value32 = next_u64(&mut state) as u32;
        let encoded32 = encode_u32(value32);
        assert_eq!(encoded32, value32.to_le_bytes());
        let mut input32 = encoded32.as_slice();
        assert_eq!(decode_exact(&mut input32, decode_u32), Ok(value32));
        assert!(input32.is_empty());

        let value64 = next_u64(&mut state);
        let encoded64 = encode_u64(value64);
        assert_eq!(encoded64, value64.to_le_bytes());
        let mut input64 = encoded64.as_slice();
        assert_eq!(decode_exact(&mut input64, decode_u64), Ok(value64));
        assert!(input64.is_empty());

        let value128 = (u128::from(next_u64(&mut state)) << 64) | u128::from(next_u64(&mut state));
        let encoded128 = encode_u128(value128);
        assert_eq!(encoded128, value128.to_le_bytes());
        let mut input128 = encoded128.as_slice();
        assert_eq!(decode_exact(&mut input128, decode_u128), Ok(value128));
        assert!(input128.is_empty());
    }
}

#[test]
fn uleb128_roundtrip_matches_independent_oracle_and_rejects_redundancy() {
    let mut values: Vec<u64> = (u16::MIN..=u16::MAX).map(u64::from).collect();
    values.extend([
        1_u64 << 21,
        (1_u64 << 28) - 1,
        1_u64 << 28,
        (1_u64 << 35) - 1,
        1_u64 << 35,
        (1_u64 << 42) - 1,
        1_u64 << 42,
        (1_u64 << 49) - 1,
        1_u64 << 49,
        (1_u64 << 56) - 1,
        1_u64 << 56,
        u64::MAX - 1,
        u64::MAX,
    ]);

    let mut state = 0xa9_6f_2c_84_13_57_b0_ed_u64;
    for _ in 0..4096 {
        values.push(next_u64(&mut state));
    }

    values.sort_unstable();
    values.dedup();

    for value in values {
        let expected = reference_uleb128(value);
        let encoded = encode_uleb128_u64(value);
        assert_eq!(encoded, expected, "value {value}");

        let mut exact = encoded.as_slice();
        assert_eq!(decode_exact(&mut exact, decode_uleb128_u64), Ok(value));
        assert!(exact.is_empty());

        if encoded.len() < 10 {
            let mut redundant = encoded.clone();
            let last = redundant.len() - 1;
            redundant[last] |= 0x80;
            redundant.push(0x00);
            let original = redundant.as_slice();
            let mut input = original;
            assert_eq!(
                decode_uleb128_u64(&mut input),
                Err(SerializationError::NonCanonicalUleb128),
                "redundant encoding for value {value}"
            );
            assert!(ptr::eq(input, original));
        }
    }
}
