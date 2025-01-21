pub mod common;

use std::{fmt::format, str::FromStr, sync::Arc};

use common::prelude::*;
use hyperlane_kadena::contracts::{
    i_mailbox::{DispatchEventData, PactHyperlaneMessage},
    i_merkle_tree_hook::IMerlkeTreeHook,
};
use kadena_client::{
    contract_call::ContractCall,
    contracts::CoinContract,
    models::{
        CommandResultDtoResult, DecimalObject, EventDataDto, EventParam, IntObject, ModuleDto,
    },
    signers::{LocalWallet, VaultSigner},
    tx::{self, report_tx},
};
use primitive_types::H160;
use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[tokio::test]
pub async fn test_add_two_numbers_local_tx() {
    let a = 1;
    let b = 2;

    let test_contract =
        TestContract::new(Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())));
    let call = test_contract.add_two_numbers(a, b).await;
    let rep_res = tx::report_tx(&call).await.unwrap();
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

    let test_contract =
        TestContract::new(Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())));

    let call = test_contract.add_two_numbers(1, 2).await;

    let estimated_gas = call.estimate_gas().await.unwrap();
    println!("estimate_gas: {:?}", estimated_gas);

    let rep_res = tx::report_tx(&call).await.unwrap();
    println!("res: {:?}", rep_res);

    let actual_gas = rep_res.values().next().unwrap().gas;
    assert_ge!(estimated_gas + GAS_BUFFER, actual_gas);
}

#[tokio::test]
pub async fn test_add_two_numbers_vault_tx() {
    use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

    let a = 1;
    let b = 2;

    // Create a client
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(&CONTEXT.vault_address)
            .token(&CONTEXT.vault_token)
            .build()
            .unwrap(),
    )
    .unwrap();

    let vault_signer = VaultSigner::new(client, CONTEXT.vault_key_id.clone(), None as Option<&str>)
        .await
        .unwrap();

    let test_contract = TestContract::new(Arc::new(vault_signer));

    let call = test_contract.add_two_numbers(a, b).await;
    let rep_res = tx::report_tx(&call).await.unwrap();
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
pub async fn test_get_balance_local_tx() {
    let coin_contract = CoinContract::new(Arc::new(TestProvider {
        client: CONTEXT.client.clone(),
        signer: Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    }));
    let balance = coin_contract
        .get_balance("sender00".to_string())
        .await
        .unwrap();
    println!("res: {:?}", balance);

    let balance = coin_contract
        .get_balance(
            "k:94c35ab1bd70243ec670495077f7846373b4dc5e9779d7a6732b5ceb6fde059c".to_string(),
        )
        .await
        .unwrap();
    println!("res: {:?}", balance);
}
