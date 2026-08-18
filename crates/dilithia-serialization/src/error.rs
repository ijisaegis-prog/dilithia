//! Dilithia Canonical Serialization error definitions.

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationError {
    UnexpectedEof,
    NonCanonicalUleb128,
    Uleb128Overflow,
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "unexpected end of input"),
            Self::NonCanonicalUleb128 => write!(f, "non-canonical ULEB128 encoding"),
            Self::Uleb128Overflow => write!(f, "ULEB128 value exceeds target bounds"),
        }
    }
}

impl std::error::Error for SerializationError {}
