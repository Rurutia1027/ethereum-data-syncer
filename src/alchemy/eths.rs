use super::BlockHash;
use super::BlockNumber;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum EthRpcMethodName {
    ETH_BLOCKNUMBER,
    ETH_GETBLOCKBYHASH,
    ETH_GETBLOCKBYNUMBER,
    ETH_GETBLOCKRECEIPTS,
    ETH_GETBLOCKTRANSACTIONCOUNTBYHASH,
    ETH_GETBLOCKTRANSACTIONCOUNTBYNUMBER,
}

impl EthRpcMethodName {
    pub fn name(&self) -> &str {
        match self {
            EthRpcMethodName::ETH_GETBLOCKBYHASH => "eth_getBlockByHash",
            EthRpcMethodName::ETH_BLOCKNUMBER => "eth_blockNumber",
            EthRpcMethodName::ETH_GETBLOCKBYNUMBER => "eth_getBlockByNumber",
            EthRpcMethodName::ETH_GETBLOCKRECEIPTS => "eth_getBlockReceipts",
            EthRpcMethodName::ETH_GETBLOCKTRANSACTIONCOUNTBYHASH => {
                "eth_getBlockTransactionCountByHash"
            }
            EthRpcMethodName::ETH_GETBLOCKTRANSACTIONCOUNTBYNUMBER => {
                "eth_getBlockTransactionCountByNumber"
            }
        }
    }
}

pub trait EthereumRpcMethods {
    // eth_getBlockByHash
    async fn eth_get_block_by_hash(&self, hash: BlockHash) -> Value;

    // eth_blockNumber
    async fn eth_block_number(&self) -> Value;

    // eth_getBlockByNumber
    async fn eth_get_block_by_number(
        &self,
        block_number: BlockNumber,
        full_transactions: bool,
    ) -> Value;

    // eth_getBlockReceipts
    async fn eth_get_block_receipts(&self, hash: BlockHash) -> Value;

    // eth_getBlockTransactionCountByHash
    async fn eth_get_block_transaction_count_by_hash(&self, hash: BlockHash) -> Value;

    // eth_getBlockTransactionCountByNumber
    async fn eth_get_block_transaction_count_by_number(&self, block_number: BlockNumber) -> Value;
}
