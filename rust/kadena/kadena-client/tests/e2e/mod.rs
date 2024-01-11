pub mod common;

use std::sync::Arc;

use hyperlane_core::{HyperlaneDomain, HyperlaneDomainType, HyperlaneDomainProtocol};
use kadena_client::{
    models::CommandResultDtoResult,
    signers::{LocalWallet, VaultSigner}, tx, contract_call::ContractCall, contract::KadenaProxyProvider,
};
use common::prelude::*;

#[tokio::test]
pub async fn test_add_two_numbers_local_tx() {
    let a = 1;
    let b = 2;
    
    let test_contract = TestContract::new(Arc::new(
        LocalWallet::new(CONTEXT.default_privkey.clone()))
    );
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

    let test_contract = TestContract::new(Arc::new(
        LocalWallet::new(CONTEXT.default_privkey.clone())
    ));

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
            .unwrap()
    ).unwrap();
    

    let vault_signer = VaultSigner::new(
        client,
        CONTEXT.vault_key_id.clone(),
        None as Option<u64>,
        None as Option<&str>,
    ).await.unwrap();

    let test_contract = TestContract::new(Arc::new(
        vault_signer
    ));

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

    let kadena_provider = Arc::new(
        KadenaProvider::new(
            domain.clone(),
            Arc::new(CONTEXT.conf.clone()),
            Arc::new(CONTEXT.proxy_conf.clone()),
            Arc::new(
                LocalWallet::new(CONTEXT.default_privkey.clone())
            )
        )
    );

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

    let kadena_provider = Arc::new(
        KadenaProvider::new(
            domain.clone(),
            Arc::new(CONTEXT.conf.clone()),
            Arc::new(CONTEXT.proxy_conf.clone()),
            Arc::new(
                LocalWallet::new(CONTEXT.default_privkey.clone())
            )
        )
    );

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

    let kadena_provider = Arc::new(
        KadenaProvider::new(
            domain.clone(),
            Arc::new(CONTEXT.conf.clone()),
            Arc::new(CONTEXT.proxy_conf.clone()),
            Arc::new(
                LocalWallet::new(CONTEXT.default_privkey.clone())
            )
        )
    );

    let mailbox_contract = IMailbox::new(kadena_provider.clone());

    let message = "0x03000000000000000b0000000000000000000000007fa9385be102ac3eac297483dd6233d62b3e1496000002720000000000000000000000006c414e7a15088023e28af44ad0e1d593671e4b1500000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000008ac7230489e8000000000000000000000000000000000000000000000000000000000000000000426b3a33656263303863323265633538636237316430356235636331656366613164353462636530336465656437353632643766336136663966653839306132623232000000000000000000000000000000000000000000000000000000000000".to_owned();
    let metadata = "0x0000000000000000000000002e234dae75c793f67a35089c9d99245e1c58470b7ca57d36281685aab33bce3c94766d3c206629baa030c43d29d3ae2ce4c0ef5a00000000dfdb2cb5fc128e08e27574e135a252b75519d47a9d71cff7655a6ebfd8477cca023fdb5a8a80fd447ce63ce280c86d704af94a2cc30b6429f89d1ee6e74a4cdb1b".to_owned();

    let call = mailbox_contract.process(hex::decode(metadata).unwrap(), hex::decode(message).unwrap());

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

    let kadena_provider = Arc::new(
        KadenaProvider::new(
            domain.clone(),
            Arc::new(CONTEXT.conf.clone()),
            Arc::new(CONTEXT.proxy_conf.clone()),
            Arc::new(
                LocalWallet::new(CONTEXT.default_privkey.clone())
            )
        )
    );

    println!("block {}", kadena_provider.get_block_number().await.unwrap());    
}