pub mod common;

use std::sync::Arc;

use hyperlane_core::{
    HyperlaneDomain, HyperlaneDomainProtocol, HyperlaneDomainType, HyperlaneMessage,
};

use common::prelude::*;
use kadena_client::{
    contract::KadenaProxyProvider,
    contract_call::ContractCall,
    event::Event,
    models::CommandResultDtoResult,
    signers::{LocalWallet, VaultSigner},
    tx::{self, report_tx},
};

#[tokio::test]
pub async fn test_add_two_numbers_local_tx() {
    let a = 1;
    let b = 2;

    let test_contract =
        TestContract::new(Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())));
    let call = test_contract.add_two_numbers(a, b).await;
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

    let test_contract =
        TestContract::new(Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())));

    let call = test_contract.add_two_numbers(1, 2).await;

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
pub async fn test_ism_module_type() {
    use hyperlane_kadena::contracts::i_interchain_security_module::IInterchainSecurityModule;
    use hyperlane_kadena::KadenaProvider;

    let domain = HyperlaneDomain::Unknown {
        domain_id: 0,
        domain_name: "test".to_owned(),
        domain_type: HyperlaneDomainType::LocalTestChain,
        domain_protocol: HyperlaneDomainProtocol::Ethereum,
    };

    let kadena_provider = Arc::new(KadenaProvider::new(
        domain.clone(),
        Arc::new(CONTEXT.conf.clone()),
        Arc::new(CONTEXT.proxy_conf.clone()),
        Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    ));

    let ism_contract = IInterchainSecurityModule::new(kadena_provider.clone());

    let call = ism_contract.module_type();

    let res = call.local().await.unwrap();
    println!("res: {:?}", res);
}

#[tokio::test]
pub async fn test_ism_validators_and_threshold() {
    use hyperlane_kadena::contracts::i_multisig_ism::IMultisigIsm;
    use hyperlane_kadena::KadenaProvider;

    let domain = HyperlaneDomain::Unknown {
        domain_id: 0,
        domain_name: "test".to_owned(),
        domain_type: HyperlaneDomainType::LocalTestChain,
        domain_protocol: HyperlaneDomainProtocol::Ethereum,
    };

    let kadena_provider = Arc::new(KadenaProvider::new(
        domain.clone(),
        Arc::new(CONTEXT.conf.clone()),
        Arc::new(CONTEXT.proxy_conf.clone()),
        Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    ));

    let ism_contract = IMultisigIsm::new(kadena_provider.clone());

    let call = ism_contract.validators_and_threshold();

    let res = call.local().await.unwrap();
    println!("res: {:?}", res);
}

#[tokio::test]
pub async fn test_mailbox_process() {
    use hyperlane_kadena::contracts::i_mailbox::IMailbox;
    use hyperlane_kadena::KadenaProvider;

    let domain = HyperlaneDomain::Unknown {
        domain_id: 0,
        domain_name: "test".to_owned(),
        domain_type: HyperlaneDomainType::LocalTestChain,
        domain_protocol: HyperlaneDomainProtocol::Ethereum,
    };

    let kadena_provider = Arc::new(KadenaProvider::new(
        domain.clone(),
        Arc::new(CONTEXT.conf.clone()),
        Arc::new(CONTEXT.proxy_conf.clone()),
        Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    ));

    let mailbox_contract = IMailbox::new(kadena_provider.clone());

    //let message = "030000000000007a690000000000000000000000006171479a003d1d89915dd9e716576203138702830000027250c669ad5b2135c2e917157d3c7ad654080499d11be575283a4961daa1a44cb4000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005616c696365000000000000000000000000000000000000000000000000000000".to_owned();
    let message = "03000000000000000b0000000000000000000000007fa9385be102ac3eac297483dd6233d62b3e1496000002720000000000000000000000006c414e7a15088023e28af44ad0e1d593671e4b1500000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000008ac7230489e8000000000000000000000000000000000000000000000000000000000000000000426b3a33656263303863323265633538636237316430356235636331656366613164353462636530336465656437353632643766336136663966653839306132623232000000000000000000000000000000000000000000000000000000000000".to_owned();
    let metadata = "0000000000000000000000005af5561c3017722a1fe42338cf5bfc615eac78fff09e36cd92a13a24d1c89c19d995b4a8bb73005667d1b674d4b37b7ad69694db000000000774c0b75f7f7ff6b93a92c0b8de1988225da5e831aa83098a7be688e2cedc35336849721b45e9cb9037da6902832e79598a0e28ab139865908de54c21a563be1b".to_owned();

    let msg = HyperlaneMessage::from(hex::decode(&message).unwrap());

    println!("msg: {:?}", msg);

    return;

    let call = mailbox_contract.process(
        hex::decode(metadata).unwrap(),
        hex::decode(message).unwrap(),
    );

    let res = call.local().await.unwrap();
    println!("res: {:?}", res);
}

#[tokio::test]
async fn test_provider() {
    use hyperlane_kadena::contracts::i_mailbox::IMailbox;
    use hyperlane_kadena::KadenaProvider;

    let domain = HyperlaneDomain::Unknown {
        domain_id: 0,
        domain_name: "test".to_owned(),
        domain_type: HyperlaneDomainType::LocalTestChain,
        domain_protocol: HyperlaneDomainProtocol::Ethereum,
    };

    let kadena_provider = Arc::new(KadenaProvider::new(
        domain.clone(),
        Arc::new(CONTEXT.conf.clone()),
        Arc::new(CONTEXT.proxy_conf.clone()),
        Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    ));

    println!(
        "block {}",
        kadena_provider.get_block_number().await.unwrap()
    );
}

#[tokio::test]
pub async fn test_events() {
    use hyperlane_core::H160;
    use hyperlane_core::H256;
    use hyperlane_kadena::contracts::i_validator_announce::IValidatorAnnounce;
    use hyperlane_kadena::KadenaProvider;
    use std::str::FromStr;

    let domain = HyperlaneDomain::Unknown {
        domain_id: 0,
        domain_name: "test".to_owned(),
        domain_type: HyperlaneDomainType::LocalTestChain,
        domain_protocol: HyperlaneDomainProtocol::Kadena,
    };

    let kadena_provider = Arc::new(KadenaProvider::new(
        domain.clone(),
        Arc::new(CONTEXT.conf.clone()),
        Arc::new(CONTEXT.proxy_conf.clone()),
        Arc::new(LocalWallet::new(CONTEXT.default_privkey.clone())),
    ));

    let contract = IValidatorAnnounce::new(kadena_provider.clone());

    let validator = H160::from_str("0x71239e00AE942B394B3a91ab229E5264aD836f6f").unwrap();

    let cmd_res = contract
        .get_announced_storage_locations(vec![validator.into()])
        .local()
        .await
        .unwrap()
        .result()
        .unwrap_or_default();

    #[derive(serde::Deserialize, Debug, Default)]
    struct StorageLocationsJson {
        storage_locations: Vec<Vec<String>>,
    }

    let locations: StorageLocationsJson = serde_json::from_value(cmd_res).unwrap_or_default();

    println!("locations: {:?}", locations);
}
