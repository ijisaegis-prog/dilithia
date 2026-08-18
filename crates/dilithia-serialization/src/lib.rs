//! Dilithia Canonical Serialization.

#![forbid(unsafe_code)]

pub mod error;
pub mod uleb128;

pub use error::SerializationError;
pub use uleb128::{decode_uleb128_u64, encode_uleb128_u64};
