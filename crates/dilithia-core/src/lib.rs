//! Preparatory canonical-value capabilities; no production values are enrolled.

#![forbid(unsafe_code)]

mod consensus_value;

pub use consensus_value::{CanonicalConsensusBytes, CanonicalConsensusField, ConsensusValue};
