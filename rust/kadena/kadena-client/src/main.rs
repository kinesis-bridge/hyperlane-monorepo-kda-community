/// This is a simple example of how to use the kadena_client crate.

use kadena_client::apis::configuration::Configuration;
use kadena_client::apis::kadena_proxy_api::get_block_by_height; 
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Configuration::new();
    let block_str = get_block_by_height(&config, "https://api.testnet.chainweb.com", "testnet04", 1, 3_000_000).await?;
    println!("{:?}", block_str);
    Ok(())
}
