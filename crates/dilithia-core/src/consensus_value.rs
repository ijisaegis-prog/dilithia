//! Centralized enrollment locus. Sealing visibility never depends on cfg/features.

use dilithia_serialization::{self as dcs, U256, UnixTimestamp};

mod sealed {
    pub trait Field {}
    pub trait Value {
        fn append_consensus_fields(&self, output: &mut Vec<u8>);
    }
}

/// A closed set of bounded DCS field representations.
///
/// Field bytes alone do not certify a top-level protocol value or a domain tag.
pub trait CanonicalConsensusField: sealed::Field {
    /// Appends this field's DCS encoding, preserving the existing prefix.
    fn append_canonical_field(&self, output: &mut Vec<u8>);
}

/// A centrally enrolled consensus value. Production enrollment is currently empty.
///
/// Future enrollment requires separately reviewed normative field order. Neither
/// this capability nor its bytes establish raw-byte consumer bypass closure.
pub trait ConsensusValue: sealed::Value {
    /// Encodes only through the private, centrally owned field encoder.
    fn canonical_consensus_bytes(&self) -> CanonicalConsensusBytes {
        let mut bytes = Vec::new();
        self.append_consensus_fields(&mut bytes);
        CanonicalConsensusBytes { bytes }
    }
}

/// Opaque output of the sealed consensus-value encoding path.
///
/// No public unchecked constructor or mutable view is provided.
#[derive(Debug, PartialEq, Eq)]
pub struct CanonicalConsensusBytes {
    bytes: Vec<u8>,
}

impl CanonicalConsensusBytes {
    /// Borrows the encoding without permitting mutation.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

// Private implementation helper, never exported or available to downstream code.
macro_rules! fixed_field {
    ($ty:ty, $encode:path) => {
        impl sealed::Field for $ty {}
        impl CanonicalConsensusField for $ty {
            fn append_canonical_field(&self, output: &mut Vec<u8>) {
                output.extend_from_slice(&$encode(*self));
            }
        }
    };
}

fixed_field!(u8, dcs::encode_u8);
fixed_field!(u16, dcs::encode_u16);
fixed_field!(u32, dcs::encode_u32);
fixed_field!(u64, dcs::encode_u64);
fixed_field!(u128, dcs::encode_u128);
fixed_field!(bool, dcs::encode_bool);
fixed_field!(UnixTimestamp, dcs::encode_unix_timestamp);

impl sealed::Field for U256 {}
impl CanonicalConsensusField for U256 {
    fn append_canonical_field(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(&dcs::encode_u256(self));
    }
}

impl<const N: usize> sealed::Field for [u8; N] {}
impl<const N: usize> CanonicalConsensusField for [u8; N] {
    fn append_canonical_field(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(&dcs::encode_bytes(self));
    }
}

impl<T: CanonicalConsensusField> sealed::Field for Option<T> {}
impl<T: CanonicalConsensusField> CanonicalConsensusField for Option<T> {
    fn append_canonical_field(&self, output: &mut Vec<u8>) {
        // Byte-identical to encode_option, without an intermediate allocation.
        match self {
            None => output.extend_from_slice(&dcs::encode_u8(0)),
            Some(value) => {
                output.extend_from_slice(&dcs::encode_u8(1));
                value.append_canonical_field(output);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Neutral fixture only: deliberately encodes in explicit order, not layout order.
    struct NonProtocolFixture {
        second: Option<bool>,
        first: u16,
    }

    impl sealed::Value for NonProtocolFixture {
        fn append_consensus_fields(&self, output: &mut Vec<u8>) {
            self.first.append_canonical_field(output);
            self.second.append_canonical_field(output);
        }
    }
    impl ConsensusValue for NonProtocolFixture {}

    #[test]
    fn neutral_fixture_uses_explicit_field_order_and_immutable_view() {
        let fixture = NonProtocolFixture {
            second: Some(true),
            first: 0x1234,
        };
        let encoded = fixture.canonical_consensus_bytes();
        assert_eq!(encoded.as_bytes(), [0x34, 0x12, 1, 1]);
        assert_eq!(encoded, fixture.canonical_consensus_bytes());
    }
}
