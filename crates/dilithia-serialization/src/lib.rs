//! Dilithia Canonical Serialization.

#![forbid(unsafe_code)]

pub mod error;
mod fixed_width;
pub mod uleb128;

pub use error::SerializationError;
pub use fixed_width::{
    decode_u8, decode_u16, decode_u32, decode_u64, decode_u128, encode_u8, encode_u16, encode_u32,
    encode_u64, encode_u128,
};
pub use uleb128::{decode_uleb128_u64, encode_uleb128_u64};
