
# Ethereum API Quickstart 

Alchemy is a platform that provides ways for developers to query multiple blockchain platforms(Solana, Ethereum, ...) dataset. Geth is a RPC CLI tool written in Golang implemented only for sending requests to urls configured Ethereum endpoints. However, Geth sometimes could not get latest datasets from Ethereum and the datasets' fetching not always stable for me as a beginner to Web3. So I decided to modify fetching functions from previous Geth into the third party Alchemy that provides stable, flexible ways to fetching datasets in both RESTful and Rpc streams across multiple blockchain platforms(like Solana, Ethereum, ...).

Alchemy provides two primary method for accessing Ethereum datasets: through the [Alchemy SDK] and via [Ethereum API Endpoints]. While Alchemy officially recommends using the SDK, which deploys on an RPC client in the local environment, we prefier utilizing the API Endpoitns. The API Endpoints allow us to interact with Alchemy's services using just an API key obtained from the platform, offering greater flexibility and a more Rust-firendly integration. 

This documennt provides notes on the various API Endpoints exposed by Alchemy and outlines the process of abstracting response body data into data models. These abstracted data models will be then be taken into consideration when creating the underlying schemas of current project [ethereum-data-syncer](../../ethereum-data-syncer/).

## Ethereum Standard APIs (via Postman)
### Getting Blocks 
Retrieves information from a particular block in the blockchain. 
#### `eth_getBlockByHash`
Returns information about a block by block hash. 

```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getBlockByHash",
  "params": [
    "0xe76d777791f48b5995d20789183514f4aa8bbf09e357383e9a44fae025c6c50a", // block hash 
    false // if true, it returns the full transaction object, otherwise only return the hashes of the transactions. 
  ]
}'
```

#### `eth_blocknumber`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_blockNumber"
}'
```

#### `eth_getBlockByNumber`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getBlockByNumber",
  "params": [
    "0x1b4", 
    true
  ]
}'
```

#### `eth_getBlockReceipts`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getBlockReceipts",
  "params": [
    "latest"
  ]
}'
```

#### `eth_getBlockTransactionCountByHash`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
    "jsonrpc": "2.0",
    "method": "eth_getBlockTransactionCountByHash",
    "params": [
        "0x8243343df08b9751f5ca0c5f8c9c0460d8a9b6351066fae0acbd4d3e776de8bb"
    ],
    "id": 0
}'
```

#### `eth_getBlockTransactionCountByNumber`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getBlockTransactionCountByNumber",
  "params": ["latest"]

}'
```

### Reading Transactions 
Retrieves information on the state data for addresses regardless of whether it is a user or a smart contract. 

#### `eth_getTransactionByHash`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getTransactionByHash",
  "params": [
    "0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b"
  ]
}'
```

#### `eth_getTransactionByBlockHashAndIndex`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "params": [
    "0xbf06c77f6ed9a65441795eb8c2ccd694b3fc9b4d1be6066bf7ed52c73c5ec97c",
    "0x64"
  ],
  "method": "eth_getTransactionByBlockHashAndIndex"
}'
```


#### `eth_getTransactionByBlockNumberAndIndex`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getTransactionByBlockNumberAndIndex",
  "params": ["earliest"]
}'
```
#### `eth_getTransactionReceipt`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getTransactionReceipt",
  "params": [
    "0x8fc90a6c3ee3001cdcbbb685b4fbe67b1fa2bec575b15b0395fea5540d0901ae"
  ]
}'
```
#### `eth_getTransactionCount`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "params": [
    "0xe5cB067E90D5Cd1F8052B83562Ae670bA4A211a8",
    "latest"
  ],
  "method": "eth_getTransactionCount"
}'
```

### Writing Transactions & EVM Execution 
Allows developers to send ETH from one address to another, write data on-chain, and interact with smart contracts. 

#### `eth_sendRawTransaction`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "params": [
    "0xf86d808504a817c800825208943535353535353535353535353535353535353535880de0b6b3a7640000801ca0e0d2e3f3d8de8a0c9f25d0b02c7a8a91e7e1c818e6f37a49d8b04f9a9a96a1a0620a6c8d95e0f289bfbf2b3d3e476de6b9bc6d0e974c06f1e4de5be7c5ef0e10"
  ],
  "method": "eth_sendRawTransaction"
}'
```
#### `eth_call`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_call",
  "params": [
    {
      "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
      "gas": "0x0",
      "gasPrice": "0x9184e72a000",
      "value": "0x0",
      "data": "0x"
    },
    {
      "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
      "gas": "0x0",
      "gasPrice": "0x9184e72a000",
      "value": "0x0",
      "data": "0x"
    }
  ]
}'
```

### Account Information 
Returns information regarding an address's stored on-chain data. 

#### `eth_getCode`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "params": [
    "0xe5cB067E90D5Cd1F8052B83562Ae670bA4A211a8",
    "latest"
  ],
  "method": "eth_getCode"
}'
```
#### `eth_getBalance`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "params": [
    "0xe5cB067E90D5Cd1F8052B83562Ae670bA4A211a8",
    "latest"
  ],
  "method": "eth_getBalance"
}'
```
#### `eth_accounts`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_accounts"
}'
```
#### `eth_getStorageAt`
```shell
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getStorageAt",
  "params": [
    "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
    "0x0",
    "latest"
  ]
}'
```

#### `eth_getProof`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
    "jsonrpc":"2.0",
    "method":"eth_getProof",
    "params":["0x7F0d15C7FAae65896648C8273B6d7E43f58Fa842",["0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"],"latest"],
    "id":1
}'
```

### Event Logs 
Returns logs which are records that denote/provide context on specific events within a smart contract, like a token transfer or a change of ownership. 
#### `eth_getLogs`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getLogs",
  "params": [
    {
      "address": [
        "0xb59f67a8bff5d8cd03f6ac17265c550ed8f33907"
      ],
      "fromBlock": "0x429d3b",
      "toBlock": "latest",
      "topics": [
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
        "0x00000000000000000000000000b46c2526e227482e2ebb8f4c69e4674d262e75",
        "0x00000000000000000000000054a2d42a40f51259dedd1978f6c118a0f0eff078"
      ]
    }
  ]
}'
```
#### `eth_newFilter`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_newFilter",
  "params": [
    {
      "address": [
        "0xb59f67a8bff5d8cd03f6ac17265c550ed8f33907"
      ],
      "fromBlock": "0x429d3b",
      "toBlock": "latest",
      "topics": [
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
        "0x00000000000000000000000000b46c2526e227482e2ebb8f4c69e4674d262e75",
        "0x00000000000000000000000054a2d42a40f51259dedd1978f6c118a0f0eff078"
      ]
    }
  ]
}'
```

#### `eth_newPendingTransactionFilter`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_newPendingTransactionFilter"
}'
```

#### `eth_newBlockFilter`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_newBlockFilter"
}'
```
#### `eth_getFilterChanges`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "params": [
    "0x1"
  ],
  "method": "eth_getFilterChanges"
}'
```

#### `eth_uninstallFilter`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_uninstallFilter",
  "params": ["0x0"]
}'
```

### Chain Information 
Returns information on the Ethereum network and internal settings. 

#### `eth_chainId`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_chainId"
}'
```

#### `eth_protocolVersion`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_protocolVersion"
}'
```


#### `net_listening`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "net_listening"
}'
```


#### `net_version`
- **eth-mainnet**
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "net_version"
}'
```

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "1"
}
```

- **eth-mainnet**
```shell 
curl --location 'https://eth-sepolia.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "net_version"
}'
```

```json 
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "11155111"
}
```


- **eth-mainnet**
```shell 
curl --location 'https://eth-holesky.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "net_version"
}'
```

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "17000"
}
```

### Getting Uncles 
Returns information on uncle blocks which are network rejected blocks and replaced by a canonical block instead. 

#### `eth_getUncleCountByBlockHash`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getUncleCountByBlockHash"
}'
```

#### `eth_getUncleByBlockNumberAndIndex`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "params": [
    "earliest",
    "0x0"
  ],
  "method": "eth_getUncleByBlockNumberAndIndex"
}'
```

#### `eth_getUncleByBlockHashAndIndex`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "params": [
    "0xb3e8c898cfbf4072eaad440e8606e578a33ca4fafc27d7936d83d7392ba3e939",
    "0x0"
  ],
  "method": "eth_getUncleByBlockHashAndIndex"
}'
```
#### `eth_getUncleCountByBlockNumber`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
  "id": 1,
  "jsonrpc": "2.0",
  "params": [
    "latest",
    "0x0"
  ],
  "method": "eth_getUncleByBlockNumberAndIndex"
}'
```

### Gas Estimation 
Returns information on the Ethereum network gas estimates. 
#### `eth_estimateGas`
```shell
 curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
    "jsonrpc":"2.0",
    "method":"eth_estimateGas",
    "params":[{
    "from": "0x8D97689C9818892B700e27F316cc3E41e17fBeb9",
    "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
    "value": "0x186a0"
    }],
    "id":1
}'
```

```json 
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x5208"
}
```

#### `eth_gasPrice`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
    "jsonrpc":"2.0",
    "method":"eth_gasPrice",
    "id": "1"
}'
```

```json 
{
    "jsonrpc": "2.0",
    "id": "1",
    "result": "0xff915c5c"
}
```

#### `eth_feeHistory`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
    "jsonrpc":"2.0",
    "method":"eth_feeHistory",
    "params":[4, "latest", [25, 75]],
    "id":1
}'
```

```json 
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "oldestBlock": "0x1479e2e",
        "reward": [
            [
                "0xc9787f3",
                "0x3393e451"
            ],
            [
                "0x1dda6768",
                "0x77359400"
            ],
            [
                "0x26dd5b32",
                "0x77359400"
            ],
            [
                "0x2faf080",
                "0x4c7a7c7d"
            ]
        ],
        "baseFeePerGas": [
            "0xfbd69f07",
            "0x118174c98",
            "0x10f7840a6",
            "0x1079dad3e",
            "0x10a6a2504"
        ],
        "gasUsedRatio": [
            0.9487425666666667,
            0.37687893333333333,
            0.38427716666666667,
            0.5424663666666667
        ],
        "baseFeePerBlobGas": [
            "0x437ef56d",
            "0x4beed42a",
            "0x556caec2",
            "0x4beed42a",
            "0x4ef9325c"
        ],
        "blobGasUsedRatio": [
            1,
            1,
            0,
            0.6666666666666666
        ]
    }
}
```
#### `eth_maxPriorityFeePerGas`
```shell 
curl --location 'https://eth-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
    "jsonrpc":"2.0",
    "method":"eth_maxPriorityFeePerGas",
    "id":1
}'
```

```json 
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x70ea40"
}
```

### Web3 
Returns Ethereum network configuration information. 
#### `web3_clientVersion`
```shell 
curl --location 'https://arb-mainnet.g.alchemy.com/v2/{{ALCHEMY_API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
    "jsonrpc":"2.0",
    "method":"web3_clientVersion",
    "params":[],
    "id":0
}'
```

```json 
{
    "jsonrpc": "2.0",
    "id": 0,
    "error": {
        "code": -32600,
        "message": "ARB_MAINNET is not enabled for this app."
    }
}
```

#### `web3_sha3`
```shell 
curl --location 'https://arb-mainnet.g.alchemy.com/v2/{{API_KEY}}' \
--header 'Content-Type: application/json' \
--header 'Cookie: _cfuvid=MO0U22rrPuh1n5aTvDYmo2y9ZboW0JiFoPaabqicrdI-1735023892227-0.0.1.1-604800000' \
--data '{
    "jsonrpc":"2.0",
    "method":"web3_sha3",
    "params":["0x68656c6c6f20776f726c64"],
    "id":64
}'
```

### Real-time Events 
This introduces WebSocket-based requests/responses which leverage a network connection allowing developers to listen for changes continuously without the need for HTTP polling. 

#### `eth_subscribe`
- **Endpoint**: `wss://eth-mainnet.g.alchemy.com/v2/{{ALCHEMY_API_KEY}} `

- **newHeads Request**
```json 
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "eth_subscribe",
    "params": [
        "newHeads"
    ]
}
```
- **newHeads Response** 

```json 
{
    "jsonrpc": "2.0",
    "method": "eth_subscription",
    "params": {
        "result": {
            "baseFeePerGas": "0x14b40c0ca",
            "blobGasUsed": "0x80000",
            "difficulty": "0x0",
            "excessBlobGas": "0x4140000",
            "extraData": "0x546974616e2028746974616e6275696c6465722e78797a29",
            "gasLimit": "0x1ca35ef",
            "gasUsed": "0x1065710",
            "hash": "0x555ae6e20be024aa7da30170754491de6d3c66e204c53853f71a1f8b3b72b94c",
            "logsBloom": "0xd7a395606d393045f635fb95d60bb12459e87ca11958004000910882596e5137511185e5c9200012f6b413d089ce57565283a47a9e0969aad7e9abcb24ed2b0075aa1c59e650af6dee164909d0405af00d29bb7b331d5a9c54ef7e2ebde72ee296c711858afec9e2611ce4b8002a6cc12d91816c23b4d85c322bf41b9a6b0483caac02d928799b492c293e690d7608660513a4b1cbeba4cdc8a25861a21a65eadfec33fb0cb4614e271b75ca691b3d780ed5f298ee3405ab9f72245694700b7f2fb003f244bc4d4b712525b4ccdd99bcee91bec2226d127e8dcbd7917bfe225648762dfac8273703736ccacdccebd78e575d5204529ae4f15d31e9aa320ef653",
            "miner": "0x4838b106fce9647bdf1e7877bf73ce8b0bad5f97",
            "mixHash": "0xfdde12dc627cec68673680ffa7b372b4228eb4d18f489ac07d6f85437a31a880",
            "nonce": "0x0000000000000000",
            "number": "0x1479d72",
            "parentBeaconBlockRoot": "0xc6edf82b4f653a9b0a593a1cc644d802eb3452d1ca77372c30bb269b380e7754",
            "parentHash": "0x2406132d7f8a231d53e5aaec369c9841d729da9bdfa7d1c2c76b1e692e3606d4",
            "receiptsRoot": "0x7ab7aad128146314ab7347b5ab00e43b9016d4d19c847bbce6e26f64413f28d4",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "size": "0x16516",
            "stateRoot": "0xa201ccabd5912589176c132db49428aed4f15590a844ac76df3757c30bc53100",
            "timestamp": "0x676a562b",
            "transactionsRoot": "0x05d08d5c06b09ac49ef7b630015ceb8901bd575d7e39eaf251937d1427024378",
            "withdrawals": [
                {
                    "index": "0x43aea0b",
                    "validatorIndex": "0xc1c6a",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x128328a"
                },
                {
                    "index": "0x43aea0c",
                    "validatorIndex": "0xc1c6b",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x1262d41"
                },
                {
                    "index": "0x43aea0d",
                    "validatorIndex": "0xc1c6c",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x127ef72"
                },
                {
                    "index": "0x43aea0e",
                    "validatorIndex": "0xc1c6d",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x12727aa"
                },
                {
                    "index": "0x43aea0f",
                    "validatorIndex": "0xc1c6e",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x127e627"
                },
                {
                    "index": "0x43aea10",
                    "validatorIndex": "0xc1c6f",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x1279144"
                },
                {
                    "index": "0x43aea11",
                    "validatorIndex": "0xc1c70",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x128bf38"
                },
                {
                    "index": "0x43aea12",
                    "validatorIndex": "0xc1c71",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x12908fc"
                },
                {
                    "index": "0x43aea13",
                    "validatorIndex": "0xc1c72",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x1286675"
                },
                {
                    "index": "0x43aea14",
                    "validatorIndex": "0xc1c73",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x1291120"
                },
                {
                    "index": "0x43aea15",
                    "validatorIndex": "0xc1c74",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x128a65f"
                },
                {
                    "index": "0x43aea16",
                    "validatorIndex": "0xc1c75",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x128d180"
                },
                {
                    "index": "0x43aea17",
                    "validatorIndex": "0xc1c76",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x128a7a6"
                },
                {
                    "index": "0x43aea18",
                    "validatorIndex": "0xc1c77",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x127eaf3"
                },
                {
                    "index": "0x43aea19",
                    "validatorIndex": "0xc1c78",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x128f716"
                },
                {
                    "index": "0x43aea1a",
                    "validatorIndex": "0xc1c79",
                    "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                    "amount": "0x128b81c"
                }
            ],
            "withdrawalsRoot": "0x549d97d094b3215b7d7cdf2ed672cbb35dc2bc4c43ea8ffa572412cbc57a3203"
        },
        "subscription": "0x07cea58aaa45362744b7bc0f403d1663"
    }
}
```

- **alchemy_minedTransactions Request**
```json 
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_subscribe",
  "params": ["alchemy_minedTransactions"]
}
```

- **alchemy_minedTransactions Response**
```json 
{
    "jsonrpc": "2.0",
    "method": "eth_subscription",
    "params": {
        "result": {
            "removed": false,
            "transaction": {
                "blockHash": "0x2eadafe61ddd10baefbdb24df09f4034d0270491f60cae1d4cd7e2d9604c2011",
                "blockNumber": "0x1479d86",
                "from": "0x93793bd1f3e35a0efd098c30e486a860a0ef7551",
                "gas": "0x3a9ee",
                "gasPrice": "0x27adb5c12",
                "maxFeePerGas": "0x27adb5c12",
                "maxPriorityFeePerGas": "0x150f3e53b",
                "hash": "0x2bddf9d792fbf4c8931b26914c577fc42ccd1cf4dab5db72c97eaeb871248a9a",
                "input": "0xa000000000000000000000000000000088e6a0c2ddd26feeb64f039a2c41296fcb3f56400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ac5e2e6cdbdeb40f00000000000000000000000000000000000000000000000000000009dab0e9dd000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                "nonce": "0x7a7ad",
                "to": "0x68d3a973e7272eb388022a5c6518d9b2a2e66fbf",
                "transactionIndex": "0x0",
                "value": "0x1479d86",
                "type": "0x2",
                "accessList": [],
                "chainId": "0x1",
                "v": "0x0",
                "r": "0x3e9008fe764b2b88eb1ce0bd6542d0c8f7bba6e52b5e28017b353e7b8083e894",
                "s": "0x20c904b64c96590835dc986249fe4425a2f1500d6959852dea6bc3629fcc66ad",
                "yParity": "0x0"
            }
        },
        "subscription": "0xe4b795e8a7aca965b8ea94c416a03590"
    }
}
```

- **alchemy_pendingTransactions Request**
```json 
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_subscribe",
  "params": ["alchemy_pendingTransactions"]
}
```

- **alchemy_pendingTransactions Response**
```json
{
    "jsonrpc": "2.0",
    "method": "eth_subscription",
    "params": {
        "result": {
            "type": "0x0",
            "chainId": "0x1",
            "nonce": "0x106cc",
            "gasPrice": "0x26cb69293",
            "gas": "0x5208",
            "to": "0xc3a39756bb13ecffb868ebf3cf977070f595d414",
            "value": "0x2451539bb5e77",
            "input": "0x",
            "r": "0x8f585adc4fe6054971975c3f0b511857342eb07c9d3e9abc592800a642a194b2",
            "s": "0x549a67681df244d7851144b27f42ee1adc0800268dc8dc3c8f92ec4d469dc6d9",
            "v": "0x25",
            "hash": "0x9e27ee1215a6a649f5ecbf53596c010be0174aaffd425a20d4ef50649492cf53",
            "blockHash": null,
            "blockNumber": null,
            "transactionIndex": null,
            "from": "0x2703983059c7b60515b4fab6fb8b301f7781f5c6"
        },
        "subscription": "0xec671dc63c95b869366c18b6b6613cc7"
    }
}
```

- **newPendingTransactions Request**
```json 
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_subscribe",
  "params": ["newPendingTransactions"]
}
```

- **newPendingTransactions Response**
```json 
{
    "jsonrpc": "2.0",
    "method": "eth_subscription",
    "params": {
        "result": "0xc7ea0e41007042e25b6a42f124a0f8163a65dbb28e6045520cf308d8d562c2a4",
        "subscription": "0x6a6bd0e8a4197b4a1f7b161aed8871bd"
    }
}
```

- **newPendingTransactions Request**
```json 
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_subscribe",
  "params": ["logs"]
}
```

- **newPendingTransactions Response**
```json 
{
    "jsonrpc": "2.0",
    "method": "eth_subscription",
    "params": {
        "result": {
            "address": "0xa5f565650890fba1824ee0f21ebbbf660a179934",
            "topics": [
                "0x936c2ca3b35d2d0b24057b0675c459e4515f48fe132d138e213ae59ffab7f53e"
            ],
            "data": "0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000205cff96434073dfceaf76b0903e21ee75ede21cfcd4bd2bdbc22ae7f59dcb07c7",
            "blockNumber": "0x1479da1",
            "transactionHash": "0xd7b5746ee3cd15cab91ce43cc9229ed2f74894a67e930106e64dfb0eeb39da2a",
            "transactionIndex": "0x80",
            "blockHash": "0x4fb24f730c049d0ecf5219dcbd975870611816f2796f3a517fe52b9f73d82765",
            "logIndex": "0x19e",
            "removed": false
        },
        "subscription": "0xd0a328339ab92983feb01a3f60ef8dfe"
    }
}
```

#### `eth_unsubscribe`
- **Logs Unsubscribe Request**

```json 
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_unsubscribe",
  "params": ["logs"]
}
```

- **Logs Unsubscribe Response**
```json 
{
    "id": 1,
    "result": false,
    "jsonrpc": "2.0"
}
```

- **newHeads Unsubscribe Request**
```json 
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_unsubscribe",
  "params": ["newHeads"]
}
```

- **newHeads Unsubscribe Response**
```json 
{
    "id": 1,
    "result": false,
    "jsonrpc": "2.0"
}
```

- **newPendingTransactions and other event types Unsubscribe Request**
```json 
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_unsubscribe",
  "params": ["newPendingTransactions", "alchemy_pendingTransactions", "alchemy_minedTransactions"]
}
```

- **newPendingTransactions Unsubscribe Response**
```json 
{
    "id": 1,
    "result": false,
    "jsonrpc": "2.0"
}
```

### Enhanced APIs 
```rust
// we do not care about this feature in this repo 
todo!()
```
## References
- **[Alchemy Ethereum API Quickstart](https://docs.alchemy.com/reference/ethereum-api-quickstart)**