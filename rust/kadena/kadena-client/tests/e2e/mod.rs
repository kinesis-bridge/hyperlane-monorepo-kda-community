pub mod common;

use kadena_client::{
    models::CommandResultDtoResult,
    signers::{LocalWallet, VaultSigner, Signer}, tx, contract_call::ContractCall,
};
use common::prelude::*;

#[tokio::test]
pub async fn test_add_two_numbers_local_tx() {
    let a = 1;
    let b = 2;
    let pact_expr = format!("(+ {} {})", a, b);

    let mut tx = build_tx_with_pact_expr(&pact_expr).await;

    let local_wallet = LocalWallet::new(CONTEXT.default_privkey.clone());
    let sig = local_wallet.sign_transaction(&mut tx).await.unwrap();

    println!(
        "tx: {:?}, sig: {:?}",
        serde_json::to_string(&tx).unwrap(),
        sig
    );
    
    let call = ContractCall::new(CONTEXT.conf.clone(), CONTEXT.proxy_conf.clone(), tx);
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

    let mut tx = build_tx_with_pact_expr("(+ 1 2)").await;

    let local_wallet = LocalWallet::new(CONTEXT.default_privkey.clone());
    let sig = local_wallet.sign_transaction(&mut tx).await.unwrap();

    println!(
        "tx: {:?}, sig: {:?}",
        serde_json::to_string(&tx).unwrap(),
        sig
    );

    let call = ContractCall::new(CONTEXT.conf.clone(), CONTEXT.proxy_conf.clone(), tx);

    let estimated_gas = call.estimate_gas().await.unwrap();
    println!("estimate_gas: {:?}", estimated_gas);

    let rep_res = tx::report_tx(call).await.unwrap();
    println!("res: {:?}", rep_res);

    let actual_gas = rep_res.values().next().unwrap().gas;
    assert_ge!(estimated_gas + GAS_BUFFER, actual_gas);
}

#[tokio::test]
pub async fn test_add_two_numbers_vault_tx() {
    use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

    let a = 1;
    let b = 2;
    let pact_expr = format!("(+ {} {})", a, b);

    let mut tx = build_tx_with_pact_expr(&pact_expr).await;

    // Create a client
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(&CONTEXT.vault_address)
            .token(&CONTEXT.vault_token)
            .build()
            .unwrap()
    ).unwrap();
    

    let vault_signer = VaultSigner::new(
        client,
        CONTEXT.vault_key_id.clone(),
        None as Option<u64>,
        None as Option<&str>,
    ).await.unwrap();
    let sig = vault_signer.sign_transaction(&mut tx).await.unwrap();

    println!(
        "tx: {:?}, sig: {:?}",
        serde_json::to_string(&tx).unwrap(),
        sig
    );
    
    let call = ContractCall::new(CONTEXT.conf.clone(), CONTEXT.proxy_conf.clone(), tx);
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
