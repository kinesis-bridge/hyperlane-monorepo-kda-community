use ed25519_dalek::SigningKey;
/// This is a simple example of how to use the kadena_client crate.

use kadena_client::apis::configuration::Configuration;
use kadena_client::apis::kadena_proxy_api::build_tx; 
use kadena_client::models::BuildPactTxDto;
use kadena_client::signer;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Configuration::new();
    let build_tx_dto = BuildPactTxDto::new(
        "https://api.testnet.chainweb.com".to_string(), 
        "testnet04".to_string(), 
        1, 
        "(+ 1 2)".to_string(), 
        "83a5cfcdcbec1d513a2d02ab0f0b61e30c9be22d9c09af001affe79885414450".to_string(), 
        "83a5cfcdcbec1d513a2d02ab0f0b61e30c9be22d9c09af001affe79885414450".to_string(),
    );

    let mut unsigned_tx  = build_tx(&config, build_tx_dto).await?;

    let key_str = "79e3b0161be8d6e200cb1d3736625eb0daf20a12186687c72cdcfeab18845444";
    let key_bytes = hex::decode(key_str)?;
    let key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());
    println!("key: {:?}", key);
    let sig = signer::sign_tx(&mut unsigned_tx, &key).unwrap();

    println!("unsigned_tx: {:?}, sig: {:?}", unsigned_tx, sig);
    Ok(())
}
