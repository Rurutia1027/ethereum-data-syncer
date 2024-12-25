/**
* We have designed this architecture with the primary goal of maintaining consistency in our underlying data models,
* regardless of the platform or method used to fetch Ethereum-associated datasets. This approach ensures that,
* not matter which API service provider or data source we integrate -- whether it's through Alchemy, MetaMask, or a future SDK-based solution --
* the underlying data models will remain uniform across all platforms.
* The key reason for this design is to decouple the fetching mechanism from the data model.
* By doing so, we can easily switch among multiple platforms, data sources, or methodologies without incurring significant modification costs
* or affecting the integrity of our core data structures(models).
* This becomes especially important when external platforms or API keys are deprecated,
* changed to a paid model, or otherwise modified. In such cases, we can seamlessly switch to another platform with minimal effort,
* and without impacting the underlying data model or existing functionality.

* For instance, if a third-party provider decides to discontinue support or introduce new pricing tiers,
* we can switch to another provider without altering the data models or API methods used by the rest of the application.
* The same applies to changes in this endpoints or response formats at the lower level, Our data models will remain unchanged,
* ensuring that the upper layers of our application (whether that be other APIs, microservices, or client-side code) are not impacted by these shifts.
* This deisng pattern provides two major benefits:
* Minmal Modification Costs
* Seamless Integeration
*/
use super::BlockHash;
use super::BlockNumber;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum EthRpcMethodName {
    // Block
    ETH_BLOCKNUMBER,
    ETH_GETBLOCKBYHASH,
    ETH_GETBLOCKBYNUMBER,
    ETH_GETBLOCKRECEIPTS,
    ETH_GETBLOCKTRANSACTIONCOUNTBYHASH,
    ETH_GETBLOCKTRANSACTIONCOUNTBYNUMBER,
    //
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
    async fn eth_get_block_by_hash(&self, hash: BlockHash, full_transaction: bool) -> Value;

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

    // -- inner utils functions --
    fn to_hex_string(block_number: BlockNumber) -> String {
        format!("0x{:x}", block_number)
    }
}
