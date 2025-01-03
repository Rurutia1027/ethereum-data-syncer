pub mod blocks;
pub mod decoders;
pub mod eths;
pub mod rpc;
pub mod subscribe;
pub mod transactions;

pub type BlockNumber = i32;
pub type Difficulty = u64;
pub type TotalDifficulty = u128;
pub type BlockHash = String;
pub type TransactionHash = String;

pub use eths::{EthRpcMethodName, EthereumRpcMethods};
