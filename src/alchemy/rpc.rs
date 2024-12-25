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
    async fn eth_get_block_by_hash(&self, hash: BlockHash) -> Value {
        let method = EthRpcMethodName::ETH_GETBLOCKBYHASH;
        // todo!(), we need to create two id_pool one for subscribe stream,
        // the other for this rpc-json api request
        // every time, in call function, once receive response body,
        // the first thing is check whether response#id
        // value match with the request's id field's value

        // but for now, take 1 for temporary
        self.call(method, json!((hash)), 1)
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
        self.call(method, json!((block_number, full_transaction)), 1)
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
        self.call(method, json!((block_number)), 1)
            .await
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
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
}
