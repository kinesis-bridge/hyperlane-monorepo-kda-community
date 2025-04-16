#![allow(warnings)]

pub mod common;

use std::num::NonZeroU64;
use std::{str::FromStr, sync::Arc};

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use common::prelude::*;
use hyperlane_core::{
    HyperlaneDomain, HyperlaneDomainProtocol, HyperlaneDomainType, MerkleTreeHook, MultisigIsm,
    ValidatorAnnounce, H160, H256,
};
use hyperlane_core::{HyperlaneMessage, RawHyperlaneMessage};
use hyperlane_kadena::contracts::i_mailbox::IMailbox;
use hyperlane_kadena::contracts::i_multisig_ism::{IMultisigIsm, ValidatorsAndThresholdCall};
use hyperlane_kadena::{
    ConnectionConf, KadenaMerkleTreeHook, KadenaMultisigIsm, KadenaProvider,
    KadenaValidatorAnnounce,
};
use kadena_client::client::{ChainwebConf, ContractsConf, KadenaProxyClient, ProxyConf};
use kadena_client::{
    contract_call::ContractCall,
    contracts::CoinContract,
    models::CommandResultDtoResult,
    signers::{LocalWallet, VaultSigner},
    tx::{self},
};
use vaultrs::database::connection;
use vaultrs::sys::wrapping::unwrap;

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

// HyperlaneMessage {
//     id: 0xe82467a0443b516dbec6b206398a20f8febc42817a749753ad347fd1726df3be,
//     version: 3, nonce: 1, origin: 626,
//     sender: 0x717a746f683731353275464c75686a4747796a374e6c3166383878525764422d,
//     destination: ethereum,
//     recipient: 0x9412e035f571d1a44058112e47ffe2ca6e36bbb7,
//     body: 0x0000000000000000000000000000000000000000000000000000000ba43b740000000000000000000000000000006dd5c9def68e8c48802210930d8fe3b7fef93f78
// }

// /// 1   Hyperlane version number
// pub version: u8,
// /// 4   Message nonce
// pub nonce: u32,
// /// 4   Origin domain ID
// pub origin: u32,
// /// 32  Address in origin convention
// pub sender: H256,
// /// 4   Destination domain ID
// pub destination: u32,
// /// 32  Address in destination convention
// pub recipient: H256,
// /// 0+  Message contents
// pub body: Vec<u8>,

#[tokio::test]
pub async fn test_validators_and_threshold() {
    let body_str = "0000000000000000000000000000000000000000000000000000000ba43b740000000000000000000000000000006dd5c9def68e8c48802210930d8fe3b7fef93f78";
    let body = hex::decode(body_str).unwrap();

    let h_message = HyperlaneMessage {
        version: 3,
        nonce: 1,
        origin: 626,
        sender: H256::from_str("717a746f683731353275464c75686a4747796a374e6c3166383878525764422d")
            .unwrap(),
        destination: 1,
        recipient: H256::from_str(
            "0000000000000000000000009412e035f571d1a44058112e47ffe2ca6e36bbb7",
        )
        .unwrap(),
        body,
    };

    let raw_h_message = RawHyperlaneMessage::from(&h_message);

    println!("raw_h_message: {:?}", hex::encode(raw_h_message.to_vec()));
}

#[tokio::test]
pub async fn test_decode_tm() {
    let body_str = "00000000000000000000000000000000000000000000000007b5bad595e238e300027b2270726564223a226b6579732d616c6c222c226b657973223a5b2265356462333539373366353434363432636238623135333963623862646630333963666531316535663765313132376131343662643261366431336432386334225d7d";
    //let body_str = "00000000000000000000000000000000000000000000000006f05b59d3b2000000027b2270726564223a226b6579732d616c6c222c226b657973223a5b2265356462333539373366353434363432636238623135333963623862646630333963666531316535663765313132376131343662643261366431336432386334225d7d";
    let body = hex::decode(body_str).unwrap();

    let h_message = HyperlaneMessage {
        version: 3,
        nonce: 1,
        origin: 626,
        sender: H256::from_str("717a746f683731353275464c75686a4747796a374e6c3166383878525764422d")
            .unwrap(),
        destination: 1,
        recipient: H256::from_str(
            "0000000000000000000000009412e035f571d1a44058112e47ffe2ca6e36bbb7",
        )
        .unwrap(),
        body: body.clone(),
    };

    let kadena_domain = HyperlaneDomain::Unknown {
        domain_id: 626,
        domain_name: "kadena".to_string(),
        domain_type: HyperlaneDomainType::Unknown,
        domain_protocol: HyperlaneDomainProtocol::Kadena,
    };

    let kadena_proxy_url = "http://localhost:3000".parse().unwrap();
    let rpc_url = "http://127.0.0.1:8080".parse().unwrap();
    let network_id = "development".to_string();
    let chain_id = 2;
    let namespace = "n_9b079bebc8a0d688e4b2f4279a114148d6760edf".to_string();

    let connection_conf = &ConnectionConf {
        url: rpc_url,
        network_id,
        chain_id,
        kadena_proxy_url: kadena_proxy_url,
        kadena_namespace: namespace,
        account_name: None,
    };

    let kadena_provider = Arc::new(KadenaProvider::new(
        kadena_domain,
        Arc::new(connection_conf.into()),
        Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    ));

    let mailbox_contract = IMailbox::new(kadena_provider.clone());

    println!("body is {}", BASE64_URL_SAFE_NO_PAD.encode(&body));

    let mut pact_tm = mailbox_contract
        .decode_token_message(body.clone())
        .local(None)
        .await
        .unwrap()
        .result()
        .unwrap();

    println!("pact tm before : {:?}", pact_tm);

    if let serde_json::Value::Number(ref num) = pact_tm["amount"] {
        if num.is_u64() {
            let amount = num.as_u64().unwrap();
            pact_tm["amount"] = serde_json::json!(amount as f64);
        } else if num.is_i64() {
            panic!("Amount is negative");
        }
    };

    println!("pact tm: {:?}", pact_tm);
}

#[tokio::test]
pub async fn test_count() {
    let body_str = "0000000000000000000000000000000000000000000000000000000ba43b740000000000000000000000000000006dd5c9def68e8c48802210930d8fe3b7fef93f78";
    let body = hex::decode(body_str).unwrap();

    let h_message = HyperlaneMessage {
        version: 3,
        nonce: 1,
        origin: 626,
        sender: H256::from_str("717a746f683731353275464c75686a4747796a374e6c3166383878525764422d")
            .unwrap(),
        destination: 1,
        recipient: H256::from_str(
            "0000000000000000000000009412e035f571d1a44058112e47ffe2ca6e36bbb7",
        )
        .unwrap(),
        body,
    };

    let kadena_domain = HyperlaneDomain::Unknown {
        domain_id: 626,
        domain_name: "kadena".to_string(),
        domain_type: HyperlaneDomainType::Unknown,
        domain_protocol: HyperlaneDomainProtocol::Kadena,
    };

    let kadena_proxy_url = "http://localhost:3000".parse().unwrap();
    let rpc_url = "http://127.0.0.1:8080".parse().unwrap();
    let network_id = "development".to_string();
    let chain_id = 2;
    let namespace = "n_9b079bebc8a0d688e4b2f4279a114148d6760edf".to_string();

    let connection_conf = ConnectionConf {
        url: rpc_url,
        network_id,
        chain_id,
        kadena_proxy_url: kadena_proxy_url,
        kadena_namespace: namespace,
        account_name: None,
    };

    let vals_and_t = KadenaMultisigIsm::new(
        &connection_conf,
        &kadena_domain,
        Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    );

    let res = vals_and_t
        .validators_and_threshold(&h_message)
        .await
        .unwrap();

    println!("res: {:?}", res);
}
