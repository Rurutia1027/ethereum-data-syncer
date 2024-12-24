//! Example of subscribing and listening for pending transactions in the public mempool by
//! `WebSocket` subscription.

use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use dotenv::dotenv;
use eyre::Result;
use futures_util::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("ALCHEMY_API_KEY").expect("Expcted ALCHEMY_API_KEY should be set");
    let rpc_url = format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    // Create the provider.
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to pending transactions.
    // Alteratively use `subscribe_full_pending_transactions` to get the full transaction details
    // directly if supported by the RPC provider.
    let sub = provider.subscribe_pending_transactions().await?;

    // Wait and take the next 3 transactions.
    let mut stream = sub.into_stream().take(3);

    println!("Awaiting pending transactions...");

    // Take the stream and print the pending transaction.
    let handle = tokio::spawn(async move {
        while let Some(tx_hash) = stream.next().await {
            // Get the transaction details.
            if let Ok(tx) = provider.get_transaction_by_hash(tx_hash).await {
                println!("Transaction details: {tx:#?}");
            }
        }
    });

    handle.await?;

    Ok(())
}
