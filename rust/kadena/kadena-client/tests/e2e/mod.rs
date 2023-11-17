pub mod common;

use ed25519_dalek::SigningKey;
use kadena_client::{
    apis::{
        configuration::{Configuration, ConnectionConf},
        kadena_proxy_api,
    },
    models::{BuildPactTxDto, CommandResultDtoResult},
    signer, tx,
};
use common::prelude::*;

#[tokio::test]
pub async fn test_add_two_numbers_tx() {
    let a = 1;
    let b = 2;
    let pact_expr = format!("(+ {} {})", a, b);

    let conf: ConnectionConf = ConnectionConf::default();
    let config = Configuration::new();
    let build_tx_dto = BuildPactTxDto::new(
        conf.url.to_string(),
        conf.network_id.clone(),
        conf.chain_id as u32,
        pact_expr,
        default_pubkey().to_string(),
        default_pubkey().to_string(),
    );

    let mut tx = kadena_proxy_api::build_tx(&config, build_tx_dto)
        .await
        .unwrap();

    let privkey_bytes = hex::decode(default_privkey()).unwrap();
    let privkey = SigningKey::from_bytes(&privkey_bytes.try_into().unwrap());

    let sig = signer::sign_tx(&mut tx, &privkey).unwrap();
    println!(
        "tx: {:?}, sig: {:?}",
        serde_json::to_string(&tx).unwrap(),
        sig
    );

    //let estimate_gas = tx::estimate_gas(&conf, tx.clone()).await?;

    //println!("estimate_gas: {:?}", estimate_gas);

    let rep_res = tx::report_tx(&conf, vec![tx]).await.unwrap();
    println!("res: {:?}", rep_res);

    match *rep_res.values().next().unwrap().result {
        CommandResultDtoResult::Success(ref res) => {
            assert_eq!(res.data.as_u64().unwrap(), a + b);
        }
        CommandResultDtoResult::Error(ref err) => {
            panic!("Error in tx {:?}", err);
        }
    }
}

#[tokio::test]
pub async fn test_estimate_gas() {
    const GAS_BUFFER: u64 = 500;
    let a = 1;
    let b = 2;
    let pact_expr = format!("(+ {} {})", a, b);

    let conf: ConnectionConf = ConnectionConf::default();
    let config = Configuration::new();
    let build_tx_dto = BuildPactTxDto::new(
        conf.url.to_string(),
        conf.network_id.clone(),
        conf.chain_id as u32,
        pact_expr,
        default_pubkey().to_string(),
        default_pubkey().to_string(),
    );

    let mut tx = kadena_proxy_api::build_tx(&config, build_tx_dto)
        .await
        .unwrap();

    let privkey_bytes = hex::decode(default_privkey()).unwrap();
    let privkey = SigningKey::from_bytes(&privkey_bytes.try_into().unwrap());

    let sig = signer::sign_tx(&mut tx, &privkey).unwrap();
    println!(
        "tx: {:?}, sig: {:?}",
        serde_json::to_string(&tx).unwrap(),
        sig
    );

    let estimated_gas = tx::estimate_gas(&conf, tx.clone()).await.unwrap();
    println!("estimate_gas: {:?}", estimated_gas);

    let rep_res = tx::report_tx(&conf, vec![tx]).await.unwrap();
    println!("res: {:?}", rep_res);

    let actual_gas = rep_res.values().next().unwrap().gas;
    assert_ge!(estimated_gas + GAS_BUFFER, actual_gas);
}
