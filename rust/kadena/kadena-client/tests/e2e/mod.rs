pub mod common;

use std::sync::Arc;

use common::prelude::*;
use hyperlane_kadena::contracts::i_mailbox::{DispatchEventData, PactHyperlaneMessage};
use kadena_client::{
    contract_call::ContractCall, contracts::CoinContract, models::{CommandResultDtoResult, DecimalObject, EventDataDto, EventParam, IntObject, ModuleDto}, signers::{
        LocalWallet,
        VaultSigner,
    }, tx::{self, report_tx}
};
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
    use vaultrs::client::{
        VaultClient,
        VaultClientSettingsBuilder,
    };

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

    let vault_signer = VaultSigner::new(
        client,
        CONTEXT.vault_key_id.clone(),
        None as Option<u64>,
        None as Option<&str>,
    )
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
    let coin_contract =
        CoinContract::new(Arc::new(TestProvider {
            client: CONTEXT.client.clone(),
            signer: Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
        }));
    let balance = coin_contract.get_balance("sender00".to_string()).await.unwrap();
    println!("res: {:?}", balance);

    let balance = coin_contract.get_balance("k:94c35ab1bd70243ec670495077f7846373b4dc5e9779d7a6732b5ceb6fde059c".to_string()).await.unwrap();
    println!("res: {:?}", balance);
}

#[tokio::test]
pub async fn test_new_process() {
    use hyperlane_kadena::contracts::i_mailbox;
    use hyperlane_kadena::KadenaProvider;
    use hyperlane_core::HyperlaneMessage;
    use hyperlane_core::RawHyperlaneMessage;
    use serde::{Deserialize, Serialize};
    use hyperlane_core::H256;

    use base64::prelude::{
        Engine as _,
        BASE64_URL_SAFE_NO_PAD,
    };


    let domain = hyperlane_core::HyperlaneDomain::Unknown {
        domain_id: 0,
        domain_name: "kadena".to_string(),
        domain_type: hyperlane_core::HyperlaneDomainType::Unknown,
        domain_protocol: hyperlane_core::HyperlaneDomainProtocol::Kadena,
    };

    let provider = Arc::new(KadenaProvider::new(
        domain,
        CONTEXT.client.clone(),
        Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    ));

    let msg_str = "AwAAAAAAAHppAAAAAAAAAAAAAAAAciB23PdvTc0wk7Alth8XcrurorkAAAJyZ2pfRVV6RFNPblRJek1yZ2xucl93WEtWSURURnp3NGUAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAYAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARWORgkT0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAX3sicHJlZCI6ImtleXMtYWxsIiwia2V5cyI6WyJlNWRiMzU5NzNmNTQ0NjQyY2I4YjE1MzljYjhiZGYwMzljZmUxMWU1ZjdlMTEyN2ExNDZiZDJhNmQxM2QyOGM0Il19AA";
    let msg = HyperlaneMessage::from(BASE64_URL_SAFE_NO_PAD.decode(msg_str).unwrap());

    let metadata_str = "AAAAAAAAAAAAAAAAN0asnTO8HoUOhVMT8nrsCfnA6CuIEteqlm10hGSGU5zzm162lVvwAmLOebwEJo1CQtdekwAAAACTyJbULVNtH3Hc1KcSuA2Q4x4RqOoJWXwecA9_LPGsVQBHI6kt7Y-Q-5O-5UsnUJBp3LiqMnyWxIleGiHVvFVWHA";
    let metadata = BASE64_URL_SAFE_NO_PAD.decode(metadata_str).unwrap();


    let mailbox_contract = i_mailbox::IMailbox::new(provider);
    let call = mailbox_contract.process(metadata, msg).await.unwrap();
    let res = call.local().await;
    //let res = tx::report_tx(call).await.unwrap();

    println!("res: {:?}", res);
}

#[tokio::test]
pub async fn test_cc_transfer() {
    use hyperlane_kadena::contracts::i_mailbox;
    use hyperlane_kadena::KadenaProvider;
    use hyperlane_core::HyperlaneMessage;
    use serde::{Deserialize, Serialize};
    use hyperlane_core::H256;
    use kadena_client::contract::Contract;

    use base64::prelude::{
        Engine as _,
        BASE64_URL_SAFE_NO_PAD,
    };


    let domain = hyperlane_core::HyperlaneDomain::Unknown {
        domain_id: 0,
        domain_name: "kadena".to_string(),
        domain_type: hyperlane_core::HyperlaneDomainType::Unknown,
        domain_protocol: hyperlane_core::HyperlaneDomainProtocol::Kadena,
    };

    let provider = Arc::new(KadenaProvider::new(
        domain,
        CONTEXT.client.clone(),
        Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    ));

    let mailbox_contract = i_mailbox::IMailbox::new(provider);
    let pact_id = "Qk9bT5UD-iElXdHU5pkIy5pPsmvyBOoEXPlJnOCOcXk";
    let destination_chain_id = 1;
    let step = 1;
    let rollback = false;

    let res = mailbox_contract.continue_transfer_remote(
        pact_id,
        destination_chain_id,
        step,
        rollback,
    ).await.unwrap();
    //let res = tx::report_tx(call).await.unwrap();

    println!("res: {:?}", res);
}


#[tokio::test]
pub async fn test_event_param() {
    use hyperlane_kadena::contracts::i_mailbox;
    use hyperlane_kadena::KadenaProvider;
    use hyperlane_core::HyperlaneMessage;
    use serde::{Deserialize, Serialize};
    use hyperlane_core::H256;
    use kadena_client::contract::Contract;

    use base64::prelude::{
        Engine as _,
        BASE64_URL_SAFE_NO_PAD,
    };

    let event_data_dto = EventDataDto {
        height: Some(1u64),
        name: "event_name".to_string(),
        params: vec![
            EventParam::IntObject(IntObject{ int: 3}), // version
            EventParam::IntObject(IntObject{ int: 0}), // nonce
            EventParam::String("AAA".to_string()), // sender
            EventParam::String("1".to_string()), // destination
            EventParam::String("36594b7a7170444e41546d5068554a7a63354131376d4a624658482d64426b56".to_string()), // recipient
            EventParam::String("36594b7a7170444e41546d5068554a7a63354131376d4a624658482d64426b56".to_string()), // recipient_tm
            //EventParam::DecimalObject(DecimalObject{ decimal: U256::from(1) } ), // amount
            EventParam::String("ds".to_string()), // amount
        ],
        module: Box::new(ModuleDto {namespace: Some("namespace".to_string()), name: "name".to_string()}),
        module_hash: "".to_string(),
    };

    let event_data = DispatchEventData::try_from(event_data_dto).unwrap();

    println!("event_data: {:?}", event_data);
}


