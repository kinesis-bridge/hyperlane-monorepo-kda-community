pub mod common;

use ed25519_dalek::SigningKey;
use kadena_client::{
    apis::{
        configuration::{Configuration, ConnectionConf},
        kadena_proxy_api,
    },
    models::{BuildPactTxDto, CommandResultDtoResult},
    signers::{LocalWallet, Signer}, tx, contract_call::ContractCall,
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

    let local_wallet = LocalWallet::new(privkey);
    let sig = local_wallet.sign_transaction(&mut tx).await.unwrap();

    println!(
        "tx: {:?}, sig: {:?}",
        serde_json::to_string(&tx).unwrap(),
        sig
    );
    
    let call = ContractCall::new(conf.clone(), config, tx);
    let rep_res = tx::report_tx(call).await.unwrap();
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

    let local_wallet = LocalWallet::new(privkey);
    let sig = local_wallet.sign_transaction(&mut tx).await.unwrap();

    println!(
        "tx: {:?}, sig: {:?}",
        serde_json::to_string(&tx).unwrap(),
        sig
    );

    let call = ContractCall::new(conf.clone(), config, tx);

    let estimated_gas = call.estimate_gas().await.unwrap();
    println!("estimate_gas: {:?}", estimated_gas);

    let rep_res = tx::report_tx(call).await.unwrap();
    println!("res: {:?}", rep_res);

    let actual_gas = rep_res.values().next().unwrap().gas;
    assert_ge!(estimated_gas + GAS_BUFFER, actual_gas);
}

#[tokio::test]
pub async fn test_vault_signing() {
    use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
    use vaultrs::transit::key;
    use vaultrs::transit::data;
    use base64::{Engine as _, engine::general_purpose};

    const DEFAULT_TOKEN : &str = "hvs.cUQ8nCxqXaL4HQFHQMQIIU7d";
    const DEFAULT_ADRESS : &str = "http://127.0.0.1:8200";


    // Create a client
    let mut client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(DEFAULT_ADRESS)
            .token(DEFAULT_TOKEN)
            .build()
            .unwrap()
    ).unwrap();
    

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

    
    let hash_bin = general_purpose::URL_SAFE_NO_PAD.decode(tx.hash.as_str()).unwrap();
    let new_hash = general_purpose::STANDARD.encode(&hash_bin);
    let resp = data::sign(&client, "transit", "test-key", new_hash.as_str(), None).await.unwrap();
    let sig_str = resp.signature.rsplit_once(':').unwrap().1;
    println!("resp: {:?} {:?}", resp, sig_str);
    let sig_bin = general_purpose::STANDARD.decode(sig_str).unwrap();
    let sig = hex::encode(sig_bin);

    println!("resp: {:?}", sig);

    let privkey_bytes = hex::decode(default_privkey()).unwrap();
    let privkey = SigningKey::from_bytes(&privkey_bytes.try_into().unwrap());
    
    let local_wallet = LocalWallet::new(privkey);
    let sig = local_wallet.sign_transaction(&mut tx).await.unwrap();

    println!(
        "tx: {:?}, sig: {:?}",
        serde_json::to_string(&tx).unwrap(),
        sig
    );

    let resp = key::list(&mut client, "transit").await.unwrap();
    println!("resp: {:?}", resp);
    println!("test_vault_signing");
    assert!(true);
}
