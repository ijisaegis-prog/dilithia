//! Dilithia Canonical Serialization.

#![forbid(unsafe_code)]

mod r#bool;
mod bytes;
pub mod error;
mod fixed_width;
mod option;
mod u256;
pub mod uleb128;
mod unix_timestamp;

pub use r#bool::{decode_bool, encode_bool};
pub use bytes::{decode_bytes, encode_bytes};
pub use error::SerializationError;
pub use fixed_width::{
    decode_u8, decode_u16, decode_u32, decode_u64, decode_u128, encode_u8, encode_u16, encode_u32,
    encode_u64, encode_u128,
};
pub use option::{decode_option, encode_option};
pub use u256::{U256, decode_u256, encode_u256};
pub use uleb128::{decode_uleb128_u64, encode_uleb128_u64};
pub use unix_timestamp::{decode_unix_timestamp, encode_unix_timestamp};
