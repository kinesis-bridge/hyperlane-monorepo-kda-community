/// This is a simple example of how to use the kadena_client crate.
use anyhow::Result;
use ed25519_dalek::SigningKey;
use hyperlane_core::{HyperlaneMessage, RawHyperlaneMessage, H256};
use kadena_client::apis::configuration::Configuration;
use kadena_client::apis::kadena_proxy_api::build_tx;
use kadena_client::models::BuildPactTxDto;
use kadena_client::signer;
use std::str::FromStr;

async fn test_sign() -> Result<()> {
    let config = Configuration::new();
    let build_tx_dto = BuildPactTxDto::new(
        "https://api.testnet.chainweb.com".to_string(),
        "testnet04".to_string(),
        1,
        "(+ 1 2)".to_string(),
        "83a5cfcdcbec1d513a2d02ab0f0b61e30c9be22d9c09af001affe79885414450".to_string(),
        "83a5cfcdcbec1d513a2d02ab0f0b61e30c9be22d9c09af001affe79885414450".to_string(),
    );

    let mut unsigned_tx = build_tx(&config, build_tx_dto).await?;

    let key_str = "79e3b0161be8d6e200cb1d3736625eb0daf20a12186687c72cdcfeab18845444";
    let key_bytes = hex::decode(key_str)?;
    let key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());
    println!("key: {:?}", key);
    let sig = signer::sign_tx(&mut unsigned_tx, &key).unwrap();

    println!(
        "unsigned_tx: {:?}, sig: {:?}",
        serde_json::to_string(&unsigned_tx)?,
        sig
    );
    Ok(())
}

async fn test_hyperlane_msg_encoding() -> Result<()> {
    let msg = HyperlaneMessage {
        version: 1,
        nonce: 325,
        origin: 1,
        sender: H256::from_str("71C7656EC7ab88b098defB751B7401B5f6d8976F000000000000000000000000")?,
        destination: 626,
        recipient: H256::from_str(
            "6b622d7765746800000000000000000000000000000000000000000000000000"
        )?,
        body: vec![],
    };
    println!(
        "msg: {:?}",
        hex::encode(RawHyperlaneMessage::from(&msg))
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    //test_sign().await?;
    test_hyperlane_msg_encoding().await?;
    Ok(())
}
