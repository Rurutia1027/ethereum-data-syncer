use std::process::id;

use reqwest::Error as ReqwestError;
use thiserror::Error;

use super::{BlockHash, BlockNumber, EthRpcMethodName, EthereumRpcMethods};
use crate::env::ENV_CONFIG;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// rest_api will abstract HTTP requests to fetch Ethereum-related data through Alchemy's third-party API platform.

/**
 * request body structure:
 *
 * {
 *
 * "jsonrpc": "2.0",
 * "method": "eth_getBlockByHash",
 * # this can use id_pool to generate global unique id to distinguish response body inner type
 * "id": 1,
 * "params": []
 * }
 *
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct EthereumRequest {
    jsonrpc: String,
    id: u32,
    method: String,
    params: Value,
}

/**
 * Error response body structure:
 * {
 *  "jsonrpc": "2.0",
 *  "id": 1,
 *  "error": {
 *      "code": -32000,
 *      "message": "invalid arguments; neither block nor hash specified"
 *  }
 * }
 *
 * Success response body structure:
 * {
 * "jsonrpc": "2.0",
 *  "id": 1,
 *  "result": {
 *   ...
 *  }
 * }
*/
#[derive(Deserialize, Debug)]
pub struct EthereumResponse {
    jsonrpc: String,
    id: u32,
    result: Option<Value>,
}

pub enum ClientType {
    MetaMask,
    Alchemy,
}

pub struct ApiClient {
    client: Client,
    api_url: String,
    api_key: String,
    client_type: ClientType,
}

#[derive(Error, Debug)]
pub enum EthereumApiError {
    #[error("HTTP request error: {0}")]
    HttpError(#[from] ReqwestError),

    #[error("Ethereum RPC error: code {code}, message: {message}")]
    RpcError { code: i32, message: String },

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

pub type EthereumApiResult<T> = Result<T, EthereumApiError>;

impl ApiClient {
    pub fn new(client_type: Option<ClientType>) -> Self {
        let client_type = client_type.unwrap_or(ClientType::Alchemy);
        let (api_url, api_key) = match &client_type {
            ClientType::MetaMask => (
                ENV_CONFIG.metamask_url.to_string(),
                ENV_CONFIG.metamask_api_key.to_string(),
            ),
            ClientType::Alchemy => (
                ENV_CONFIG.alchemy_url.to_string(),
                ENV_CONFIG.alchemy_api_key.to_string(),
            ),
        };

        Self {
            client: reqwest::Client::new(),
            api_url,
            api_key,
            client_type,
        }
    }

    /// Sends an HTTP request to the Ethereum JSON-RPC based on the client's configured 3rd platform's API address.
    /// # Arguments
    /// * `method` - The RPC method to be called (e.g., "eth_getBlockByHash").
    /// * `params` - The parameters to add to the request body
    /// # Returns
    /// A `Result` containing the parsed success or error response.
    async fn call(
        &self,
        method: EthRpcMethodName,
        params: Value,
        id: u32,
    ) -> EthereumApiResult<Value> {
        // Create the request body
        let request_body = EthereumRequest {
            jsonrpc: "2.0".to_string(),
            id: id,
            method: method.name().to_string(),
            params,
        };

        // Send the request
        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        // Parse the response as JSON
        let response_body: Value = response.json().await?;

        // Check if response contains an "error" field, if so extract it's inner fields
        // to create EthereumApiError::RpcError's inner fields
        if let Some(error) = response_body.get("error") {
            let code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(0) as i32;
            let message = error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error")
                .to_string();
            return Err(EthereumApiError::RpcError { code, message });
        }

        // Extract the "result" field as return value
        if let Some(result) = response_body.get("result") {
            return Ok(result.clone());
        }

        // if neither "error" nor "result" exsits, return an unknow error
        Err(EthereumApiError::UnknownError(
            "Response contains neither `result` nor `error` field".to_string(),
        ))
    }
}

impl EthereumRpcMethods for ApiClient {
    async fn eth_get_block_by_hash(&self, hash: BlockHash, full_transaction: bool) -> Value {
        let method = EthRpcMethodName::ETH_GETBLOCKBYHASH;
        // todo!(), we need to create two id_pool one for subscribe stream,
        // the other for this rpc-json api request
        // every time, in call function, once receive response body,
        // the first thing is check whether response#id
        // value match with the request's id field's value

        // but for now, take 1 for temporary
        self.call(method, json!((hash, full_transaction)), 1)
            .await
            .unwrap_or_default()
    }

    async fn eth_block_number(&self) -> Value {
        let method = EthRpcMethodName::ETH_BLOCKNUMBER;
        self.call(method, json!(()), 1).await.unwrap_or_default()
    }

    async fn eth_get_block_by_number(
        &self,
        block_number: BlockNumber,
        full_transaction: bool,
    ) -> Value {
        let method = EthRpcMethodName::ETH_GETBLOCKBYNUMBER;
        self.call(
            method,
            json!((Self::to_hex_string(block_number), full_transaction)),
            1,
        )
        .await
        .unwrap_or_default()
    }

    async fn eth_get_block_receipts(&self, hash: BlockHash) -> Value {
        let method = EthRpcMethodName::ETH_GETBLOCKRECEIPTS;
        self.call(method, json!((hash)), 1)
            .await
            .unwrap_or_default()
    }

    async fn eth_get_block_transaction_count_by_hash(&self, hash: BlockHash) -> Value {
        let method = EthRpcMethodName::ETH_GETBLOCKTRANSACTIONCOUNTBYHASH;
        self.call(method, json!((hash)), 1)
            .await
            .unwrap_or_default()
    }

    async fn eth_get_block_transaction_count_by_number(&self, block_number: BlockNumber) -> Value {
        let method = EthRpcMethodName::ETH_GETBLOCKTRANSACTIONCOUNTBYNUMBER;
        self.call(method, json!((Self::to_hex_string(block_number))), 1)
            .await
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::{num::ParseIntError, process::exit};

    use serde::de::value;

    use super::*;

    #[test]
    fn test_create_api_client() {
        let api_client = ApiClient::new(Some(ClientType::Alchemy));
        assert!(api_client.api_key.len() > 0);
        assert_eq!(api_client.api_key, ENV_CONFIG.alchemy_api_key);
        assert_eq!(api_client.api_url, ENV_CONFIG.alchemy_url);

        // if not set type of the client, use Alchemy as default
        let api_client = ApiClient::new(None);
        assert!(api_client.api_key.len() > 0);
        assert_eq!(api_client.api_key, ENV_CONFIG.alchemy_api_key);
        assert_eq!(api_client.api_url, ENV_CONFIG.alchemy_url);
    }

    #[tokio::test]
    async fn test_eth_block_number() {
        // first create instance of ApiClient
        let api_client = ApiClient::new(None);
        let ret = api_client.eth_block_number().await;
        assert!(ret.is_string());
    }

    #[tokio::test]
    async fn test_eth_get_block_by_number() {
        // first create instance of ApiClient
        let api_client = ApiClient::new(None);

        // get a block number
        let mut ret = api_client.eth_block_number().await;
        let retry_max_time = 3;
        let mut retry_cnt = 0;

        // some times remote api is not always return a valid block number
        while retry_cnt < retry_max_time && !ret.is_string() {
            ret = api_client.eth_block_number().await;
            retry_cnt += 1;
        }

        if !ret.is_string() && retry_cnt >= retry_max_time {
            eprintln!("Retry 3 times but all failed to fetch a valid block number value, skipping this test.");
            return;
        }

        // have to say here's logic is a bit complex...
        // what we want to do is the extract received valid block number like 0x...
        // and converted it back into a valid value in type of i32 which declared as the BlockNumber
        // which invoke function of eth_get_block_by_number requires
        // but, again... in the scope of eth_get_block_by_number function it conveerted the i32 into string with prefix of '0x' again ...

        let block_number_i32 = hex_string_to_i32(&ret);
        match block_number_i32 {
            Ok(block_number_int) => {
                // println!("block_number_i32 {}", block_number_int);
                let ret = api_client
                    .eth_get_block_by_number(block_number_int as BlockNumber, true)
                    .await;
                // println!("ret content {:?}", ret);
            }
            Err(err) => {
                eprintln!("failed to convert {:?} into i32 with error {:?}", ret, err);
                return;
            }
        }
    }

    // add a todo!() here
    // imple api_client instance to test context in the scope of mod tests
    // so that we do not need to create && initialize multiple instances for it
    // all test cases share the same instance of api_client, and make sure api_client is thread-safe and let it protected by
    // Mutex and Arc
    #[tokio::test]
    async fn test_eth_get_block_by_hash() {
        // first create instance of ApiClient
        let api_client = ApiClient::new(None);

        // get a block number
        let mut ret = api_client.eth_block_number().await;
        let retry_max_time = 3;
        let mut retry_cnt = 0;

        // some times remote api is not always return a valid block number
        while retry_cnt < retry_max_time && !ret.is_string() {
            ret = api_client.eth_block_number().await;
            retry_cnt += 1;
        }

        if !ret.is_string() && retry_cnt >= retry_max_time {
            eprintln!("Retry 3 times but all failed to fetch a valid block number value, skipping this test.");
            return;
        }

        let block_number_ret = hex_string_to_i32(&ret);

        let block_number = match block_number_ret {
            Ok(block_number_i32) => block_number_i32,
            Err(err) => {
                eprintln!("failed to convert {:?} into i32 with error {:?}", ret, err);
                return;
            }
        };

        let block_query_by_block_number = api_client
            .eth_get_block_by_number(block_number, false)
            .await;

        let block_1_block_number = if let Some(block_number_value) = block_query_by_block_number
            .get("number")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        {
            block_number_value
        } else {
            eprintln!("No 'number' field found in response json value");
            String::new()
        };

        let block_1_hash = if let Some(block_hash_value) = block_query_by_block_number
            .get("hash")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        {
            block_hash_value
        } else {
            eprint!("No 'hash' field found in response json value");
            String::new()
        };

        let block_query_by_hash_value = api_client
            .eth_get_block_by_hash(block_1_hash.clone() as BlockHash, false)
            .await;

        // after we got block result, continue with parsing the inner field of its block number and hash
        let block_2_hash = if let Some(block_hash_value) = block_query_by_hash_value
            .get("hash")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        {
            block_hash_value
        } else {
            eprintln!("No 'hash' field found in response json value");
            String::new()
        };

        let block_2_block_number = if let Some(block_number_value) = block_query_by_hash_value
            .get("number")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        {
            block_number_value
        } else {
            eprintln!("No 'number' field found in response json value");
            String::new()
        };

        // finally, compare with the first block's block number, hash value should be matched with the second block's block number
        // and both of the block number value should be matched with the first query's eth_block_number's block number value
        assert_eq!(block_2_block_number, block_1_block_number);
        assert_eq!(block_1_hash, block_2_hash);
        assert_eq!(
            block_number,
            i32::from_str_radix(&block_2_block_number[2..], 16)
                .expect("block number as 0x... should be parsed correctly")
        );
    }

    #[tokio::test]
    async fn test_eth_get_block_receipts() {
        // first get block number value via get_blockNumber API request
        let api_client = ApiClient::new(None);

        // fetch a block number from remote(Alchemy)
        let mut ret = api_client.eth_block_number().await;
        let retry_max_time = 3;
        let mut retry_cnt = 0;

        while retry_cnt < retry_max_time && !ret.is_string() {
            ret = api_client.eth_block_number().await;
            retry_cnt += 1;
        }

        if !ret.is_string() {
            eprint!("Retry 3 times but all failed to fetch a valid block number value, skipping this test.");
            return;
        }

        let block_number_ret = hex_string_to_i32(&ret)
            .expect("ret value of hex string of block number should be parsed into i32 correctly");
        assert!(block_number_ret > 0);

        // then take the queried fresh block number query eth_blockTransactionCountByNumber
        let ret = api_client
            .eth_get_block_transaction_count_by_number(block_number_ret)
            .await;

        assert!(ret.is_string());
        let transaction_count_in_given_block_number = hex_string_to_i32(&ret)
            .expect("transaction count value should be parsed correctly from hex string");
        assert!(transaction_count_in_given_block_number > 0);
        println!(
            "transaction_count_in_given_block_number : {:?}",
            transaction_count_in_given_block_number
        );
    }

    #[tokio::test]
    async fn test_eth_get_block_transaction_count_by_hash() {
        // first, get block number value via get_blockNumber API request
        // then, query block item via eth_getBlockByNumber
        // then, extract the hash field from the fresh block item, and query eth_blockTransactionCountByHash
    }

    #[tokio::test]
    async fn test_eth_get_block_transaction_count_by_number() {
        // first, get block number value via get_blockNumber API request
        // then, query block item via eth_getBlockByNumber
        // then, extract the block number field from the fresh block item, and query eth_blockTransactionCountByHash
    }

    // -- util functions --
    fn hex_string_to_i32(ret: &Value) -> Result<i32, ParseIntError> {
        let block_number: String = if let Some(value) = ret.as_str() {
            value.to_string()
        } else {
            String::new()
        };

        assert!(block_number.len() > 0);
        i32::from_str_radix(&block_number[2..], 16)
    }
}
