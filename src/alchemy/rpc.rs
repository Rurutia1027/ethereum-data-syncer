use reqwest::Error as ReqwestError;
use thiserror::Error;

use crate::env::ENV_CONFIG;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    params: Vec<String>,
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
    async fn call(&self, method: &str, params: Vec<String>) -> EthereumApiResult<Value> {
        // Create the request body
        let request_body = EthereumRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: method.to_string(),
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
}
