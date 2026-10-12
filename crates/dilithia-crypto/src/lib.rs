//! Cryptographic primitives whose algorithms are already fixed by the Formal Specification.
//!
//! This crate does not select Dilithia's post-quantum authorization algorithms or implement
//! runtime Crypto Agility. Those remain separate specification decisions.

#![forbid(unsafe_code)]

use sha3::{Digest, Sha3_256};

/// Size in bytes of a SHA3-256 digest.
pub const SHA3_256_DIGEST_BYTES: usize = 32;

/// Fixed-width SHA3-256 digest bytes.
pub type Sha3_256Digest = [u8; SHA3_256_DIGEST_BYTES];

/// Validation failure for the canonical Constitution byte rules in Formal Specification §4.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstitutionCanonicalBytesError {
    /// The byte sequence starts with the forbidden UTF-8 BOM `EF BB BF`.
    Utf8Bom,
    /// The byte sequence is not strict UTF-8.
    InvalidUtf8,
    /// The byte sequence contains a carriage-return byte (`0D`).
    CarriageReturn,
    /// The byte sequence does not end with exactly one LF byte (`0A`).
    MissingFinalLf,
    /// The byte sequence ends with two or more consecutive LF bytes.
    MultipleFinalLf,
}

impl core::fmt::Display for ConstitutionCanonicalBytesError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let message = match self {
            Self::Utf8Bom => "canonical Constitution bytes must not start with a UTF-8 BOM",
            Self::InvalidUtf8 => "canonical Constitution bytes must be strict UTF-8",
            Self::CarriageReturn => {
                "canonical Constitution bytes must not contain carriage returns"
            }
            Self::MissingFinalLf => {
                "canonical Constitution bytes must end with exactly one LF byte"
            }
            Self::MultipleFinalLf => {
                "canonical Constitution bytes must not end with multiple consecutive LF bytes"
            }
        };
        f.write_str(message)
    }
}

impl std::error::Error for ConstitutionCanonicalBytesError {}

/// Validates the canonical Constitution byte rules defined by Formal Specification §4.1.
///
/// Validation is byte-exact. It performs no Unicode normalization, case folding, whitespace
/// rewriting, tab expansion, Markdown rendering, transcoding, or line-ending repair.
pub fn validate_canonical_constitution_bytes(
    bytes: &[u8],
) -> Result<(), ConstitutionCanonicalBytesError> {
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(ConstitutionCanonicalBytesError::Utf8Bom);
    }

    core::str::from_utf8(bytes).map_err(|_| ConstitutionCanonicalBytesError::InvalidUtf8)?;

    if bytes.contains(&b'\r') {
        return Err(ConstitutionCanonicalBytesError::CarriageReturn);
    }

    if bytes.last() != Some(&b'\n') {
        return Err(ConstitutionCanonicalBytesError::MissingFinalLf);
    }

    if bytes.len() >= 2 && bytes[bytes.len() - 2] == b'\n' {
        return Err(ConstitutionCanonicalBytesError::MultipleFinalLf);
    }

    Ok(())
}

/// Computes SHA3-256 over the exact supplied bytes.
///
/// The function performs no text normalization, framing, or domain separation. Callers must
/// supply the exact bytes required by the applicable Formal Specification rule.
#[must_use]
pub fn sha3_256(bytes: &[u8]) -> Sha3_256Digest {
    let digest = Sha3_256::digest(bytes);
    let mut output = [0_u8; SHA3_256_DIGEST_BYTES];
    output.copy_from_slice(&digest);
    output
}

/// Computes the constitution hash defined by Formal Specification section 4.
///
/// `canonical_constitution_bytes` must already be the exact canonical Constitution bytes. This
/// function deliberately does not read a filesystem path or normalize line endings, so the hash
/// commits to the caller-supplied bytes exactly.
#[must_use]
pub fn constitution_hash(canonical_constitution_bytes: &[u8]) -> Sha3_256Digest {
    sha3_256(canonical_constitution_bytes)
}

/// Validates Formal Specification §4.1 and then computes the Constitution SHA3-256 hash.
///
/// This helper does not normalize or repair input. Invalid canonical bytes are rejected before
/// hashing; valid bytes are hashed exactly as supplied.
pub fn constitution_hash_checked(
    constitution_bytes: &[u8],
) -> Result<Sha3_256Digest, ConstitutionCanonicalBytesError> {
    validate_canonical_constitution_bytes(constitution_bytes)?;
    Ok(constitution_hash(constitution_bytes))
}

#[cfg(test)]
mod tests {
    use super::{
        ConstitutionCanonicalBytesError, constitution_hash, constitution_hash_checked, sha3_256,
        validate_canonical_constitution_bytes,
    };

    #[test]
    fn sha3_256_matches_empty_message_vector() {
        assert_eq!(
            sha3_256(b""),
            [
                0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56, 0xa0, 0x61,
                0xd6, 0x62, 0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa, 0x82, 0xd8, 0x0a, 0x4b,
                0x80, 0xf8, 0x43, 0x4a,
            ]
        );
    }

    #[test]
    fn sha3_256_matches_abc_vector() {
        assert_eq!(
            sha3_256(b"abc"),
            [
                0x3a, 0x98, 0x5d, 0xa7, 0x4f, 0xe2, 0x25, 0xb2, 0x04, 0x5c, 0x17, 0x2d, 0x6b, 0xd3,
                0x90, 0xbd, 0x85, 0x5f, 0x08, 0x6e, 0x3e, 0x9d, 0x52, 0x5b, 0x46, 0xbf, 0xe2, 0x45,
                0x11, 0x43, 0x15, 0x32,
            ]
        );
    }

    #[test]
    fn sha3_256_matches_long_known_answer_vector() {
        assert_eq!(
            sha3_256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            [
                0x41, 0xc0, 0xdb, 0xa2, 0xa9, 0xd6, 0x24, 0x08, 0x49, 0x10, 0x03, 0x76, 0xa8, 0x23,
                0x5e, 0x2c, 0x82, 0xe1, 0xb9, 0x99, 0x8a, 0x99, 0x9e, 0x21, 0xdb, 0x32, 0xdd, 0x97,
                0x49, 0x6d, 0x33, 0x76,
            ]
        );
    }

    #[test]
    fn sha3_256_matches_rate_boundary_vectors() {
        assert_eq!(
            sha3_256(&[b'a'; 135]),
            [
                0x80, 0x94, 0xbb, 0x53, 0xc4, 0x4c, 0xfb, 0x1e, 0x67, 0xb7, 0xc3, 0x04, 0x47, 0xf9,
                0xa1, 0xc3, 0x36, 0x96, 0xd2, 0x46, 0x3e, 0xcc, 0x1d, 0x9c, 0x92, 0x53, 0x89, 0x13,
                0x39, 0x28, 0x43, 0xc9,
            ]
        );
        assert_eq!(
            sha3_256(&[b'a'; 136]),
            [
                0x3f, 0xc5, 0x55, 0x9f, 0x14, 0xdb, 0x8e, 0x45, 0x3a, 0x0a, 0x30, 0x91, 0xed, 0xbd,
                0x2b, 0xc2, 0x5e, 0x11, 0x52, 0x8d, 0x81, 0xc6, 0x6f, 0xa5, 0x70, 0xa4, 0xef, 0xdc,
                0xc2, 0x69, 0x5e, 0xe1,
            ]
        );
        assert_eq!(
            sha3_256(&[b'a'; 137]),
            [
                0xf8, 0xd6, 0x84, 0x6c, 0xed, 0xd2, 0xcc, 0xfa, 0xdf, 0x15, 0xc5, 0x87, 0x9e, 0xf9,
                0x5a, 0xf7, 0x24, 0xd7, 0x99, 0xee, 0xd7, 0x39, 0x1f, 0xb1, 0xc9, 0x1f, 0x95, 0x34,
                0x4e, 0x73, 0x86, 0x14,
            ]
        );
    }

    #[test]
    fn canonical_constitution_hash_preserves_distinct_valid_unicode_bytes() {
        let composed = "\u{00e9}\n".as_bytes();
        let decomposed = "e\u{0301}\n".as_bytes();

        assert_eq!(validate_canonical_constitution_bytes(composed), Ok(()));
        assert_eq!(validate_canonical_constitution_bytes(decomposed), Ok(()));
        assert_ne!(composed, decomposed);
        assert_ne!(
            constitution_hash_checked(composed),
            constitution_hash_checked(decomposed)
        );
    }

    #[test]
    fn constitution_hash_hashes_exact_bytes_without_normalization() {
        assert_eq!(constitution_hash(b"abc"), sha3_256(b"abc"));
        assert_ne!(constitution_hash(b"line\n"), constitution_hash(b"line\r\n"));
    }

    #[test]
    fn canonical_constitution_validation_accepts_exact_lf_utf8_bytes() {
        assert_eq!(
            validate_canonical_constitution_bytes("Dilithia §4.1\n".as_bytes()),
            Ok(())
        );
        assert_eq!(validate_canonical_constitution_bytes(b"\n"), Ok(()));
    }

    #[test]
    fn canonical_constitution_validation_rejects_utf8_bom() {
        assert_eq!(
            validate_canonical_constitution_bytes(b"\xef\xbb\xbfDilithia\n"),
            Err(ConstitutionCanonicalBytesError::Utf8Bom)
        );
    }

    #[test]
    fn canonical_constitution_validation_rejects_invalid_utf8() {
        assert_eq!(
            validate_canonical_constitution_bytes(&[0xff, b'\n']),
            Err(ConstitutionCanonicalBytesError::InvalidUtf8)
        );
    }

    #[test]
    fn canonical_constitution_validation_rejects_carriage_return() {
        assert_eq!(
            validate_canonical_constitution_bytes(b"line\r\n"),
            Err(ConstitutionCanonicalBytesError::CarriageReturn)
        );
        assert_eq!(
            validate_canonical_constitution_bytes(b"line\rnext\n"),
            Err(ConstitutionCanonicalBytesError::CarriageReturn)
        );
    }

    #[test]
    fn canonical_constitution_validation_rejects_missing_final_lf() {
        assert_eq!(
            validate_canonical_constitution_bytes(b"Dilithia"),
            Err(ConstitutionCanonicalBytesError::MissingFinalLf)
        );
        assert_eq!(
            validate_canonical_constitution_bytes(b""),
            Err(ConstitutionCanonicalBytesError::MissingFinalLf)
        );
    }

    #[test]
    fn canonical_constitution_validation_rejects_multiple_final_lf() {
        assert_eq!(
            validate_canonical_constitution_bytes(b"Dilithia\n\n"),
            Err(ConstitutionCanonicalBytesError::MultipleFinalLf)
        );
        assert_eq!(
            validate_canonical_constitution_bytes(b"\n\n"),
            Err(ConstitutionCanonicalBytesError::MultipleFinalLf)
        );
    }

    #[test]
    fn checked_constitution_hash_matches_exact_hash_for_valid_bytes() {
        let bytes = b"Dilithia\n";
        assert_eq!(
            constitution_hash_checked(bytes),
            Ok(constitution_hash(bytes))
        );
    }

    #[test]
    fn checked_constitution_hash_rejects_noncanonical_bytes() {
        assert_eq!(
            constitution_hash_checked(b"Dilithia\r\n"),
            Err(ConstitutionCanonicalBytesError::CarriageReturn)
        );
    }
}
