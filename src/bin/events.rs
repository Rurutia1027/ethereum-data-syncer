use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use dotenv::dotenv;
use eyre::Result;
use futures_util::{stream, StreamExt};
use std::env;

// refer to: https://alloy.rs/examples/subscriptions/subscribe_blocks.html
// refer to: https://docs.alchemy.com/reference/newheads

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // here we replace the WsConnection's inner address into the 3rd party's RPC URL ADDRESS this should support
    // Metamask(previous Infura), Alchemy, Ankr and other 3rd platforms
    // we both add our own API_KEYs from Metamask, Alchemy to our local .env file
    // and add correspoinding API_KEYs as secrets variables to GitHub which let CI/CD pipeline's test cases goes correct.
    let api_key = env::var("ALCHEMY_API_KEY").expect("Expcted ALCHEMY_API_KEY should be set");
    let rpc_url = format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    // Create the provider.
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to block headers

    let subscription = provider.subscribe_blocks().await?;
    let mut stream = subscription.into_stream().take(2);

    while let Some(header) = stream.next().await {
        println!("Received block number: {:?}", header);
    }

    // poll for block headers
    let poller = provider.watch_blocks().await?;
    let mut stream = poller.into_stream().flat_map(stream::iter).take(2);

    while let Some(block_hash) = stream.next().await {
        println!("Polled for block header: {block_hash:?}");
    }

    Ok(())
}
