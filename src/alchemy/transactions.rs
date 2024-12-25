use super::decoders::from_i32_hex_str;
use super::{BlockHash, BlockNumber, TransactionHash};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    #[serde(deserialize_with = "from_i32_hex_str")]
    pub block_number: BlockNumber,
    pub block_hash: BlockHash,
    #[serde(deserialize_with = "from_i32_hex_str")]
    pub gas_used: i32,
    pub to: Option<String>,
    pub transaction_hash: TransactionHash,
}
