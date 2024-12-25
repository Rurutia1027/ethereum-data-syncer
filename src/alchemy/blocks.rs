use super::decoders::*;

use super::{BlockHash, BlockNumber, Difficulty, TotalDifficulty};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    // Higest gas price seen, ~4000 Gwei, if we want 1000x to future proof,
    // we need to handle 4000 * 1000 * 1e9(Gwei) = 4e15, which needs 52 bits.
    // This value still fits within FLOAT8 (2^53).
    #[serde(default)]
    #[serde(deserialize_with = "from_u64_opt_hex_str")]
    pub base_fee_per_gas: Option<u64>,
    #[serde(deserialize_with = "from_u64_hex_str")]
    pub difficulty: Difficulty,
    // Started at 8M, current at 30M, seems to fit in 2^31 for the foreseeable future.
    #[serde(deserialize_with = "from_i32_hex_str")]
    pub gas_used: i32,
    #[serde(default)]
    #[serde(deserialize_with = "from_i32_opt_hex_str")]
    pub blob_gas_used: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "from_i32_opt_hex_str")]
    pub excess_blob_gas: Option<i32>,
    pub hash: BlockHash,
    #[serde(deserialize_with = "from_i32_hex_str")]
    pub number: BlockNumber,
    pub parent_hash: String,
    #[serde(deserialize_with = "from_unix_timestamp_hex_str")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    #[serde(deserialize_with = "from_u128_hex_str")]
    pub total_difficulty: TotalDifficulty,
    // Types for blocks coming from the node and from our DB should be split
    pub transactions: Vec<String>,
}
