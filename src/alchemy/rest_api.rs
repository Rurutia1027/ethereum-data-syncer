use crate::alchemy::blocks::Block;
use crate::alchemy::transactions::TransactionReceipt;
use crate::env::ENV_CONFIG;

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

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
pub struct RpcRequest {
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
pub struct RpcResponse<T> {
    jsonrpc: String,
    id: u32,
    result: Option<T>,
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
