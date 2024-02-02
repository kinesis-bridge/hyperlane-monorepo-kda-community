pub mod prelude {
    use async_trait::async_trait;
    use ed25519_dalek::SigningKey;
    use kadena_client::{
        client::{
            ChainwebConf,
            KadenaProxyClient,
            ProxyConf,
        },
        contract::{
            Contract,
            KadenaProxyProvider,
        },
        contract_call::ContractCall,
        error::KadenaClientError,
        models::CommandDto,
        signers::Signer,
    };
    pub use more_asserts::*;
    use once_cell::sync::Lazy;
    use std::{
        path::Path,
        sync::Arc,
    };
    use url::Url;

    pub struct TestProvider {
        client: Arc<KadenaProxyClient>,
        signer: Arc<dyn Signer>,
    }

    impl KadenaProxyProvider for TestProvider {
        fn proxy_client(&self) -> Arc<KadenaProxyClient> {
            self.client.clone()
        }

        fn signer(&self) -> Arc<dyn Signer> {
            self.signer.clone()
        }
    }

    pub struct AddTwoNumbersCall<'a> {
        contract: &'a TestContract,
        #[allow(dead_code)]
        a: u64,
        #[allow(dead_code)]
        b: u64,
        gas_limit: Option<u64>,
    }

    impl AddTwoNumbersCall<'_> {
        pub async fn new(contract: &TestContract, a: u64, b: u64) -> AddTwoNumbersCall {
            AddTwoNumbersCall {
                contract,
                a,
                b,
                gas_limit: None,
            }
        }
    }

    #[async_trait]
    impl ContractCall for AddTwoNumbersCall<'_> {
        fn contract(&self) -> &dyn Contract {
            self.contract
        }

        fn gas_limit(&self) -> Option<u64> {
            self.gas_limit
        }

        fn set_gas_limit(&mut self, gas_limit: u64) {
            self.gas_limit = Some(gas_limit);
        }

        async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
            self.contract
                .build_pact_tx_with_expr(&format!("(+ {} {})", self.a, self.b), self.gas_limit)
                .await
                .map_err(|e| e.into())
        }
    }

    pub struct TestContract {
        provider: Arc<dyn KadenaProxyProvider + Sync + Send>,
    }

    impl TestContract {
        const MODULE_NAME: &'static str = "test";

        pub fn new(signer: Arc<dyn Signer>) -> Self {
            let provider = Arc::new(TestProvider {
                client: CONTEXT.client.clone(),
                signer,
            });
            TestContract { provider }
        }

        pub async fn add_two_numbers(&self, a: u64, b: u64) -> AddTwoNumbersCall {
            AddTwoNumbersCall::new(self, a, b).await
        }
    }

    impl Contract for TestContract {
        fn module_name(&self) -> &'static str {
            Self::MODULE_NAME
        }

        fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync> {
            self.provider.clone()
        }
    }

    #[derive(Debug)]
    pub struct Context {
        pub default_pubkey_str: String,
        pub default_privkey_str: String,
        pub default_privkey: SigningKey,

        pub vault_token: String,
        pub vault_address: Url,
        pub vault_key_id: String,

        pub client: Arc<KadenaProxyClient>,
    }

    pub static CONTEXT: Lazy<Context> = Lazy::new(|| {
        dotenvy::from_path(Path::new("tests/e2e/.env")).unwrap();
        let default_privkey_str = dotenvy::var("DEFAULT_PRIVKEY").unwrap();
        let privkey_bytes = hex::decode(&default_privkey_str).unwrap();
        let default_privkey = SigningKey::from_bytes(&privkey_bytes.try_into().unwrap());

        let kadena_proxy_url = dotenvy::var("KADENA_PROXY_URL").unwrap().parse().unwrap();

        let network_id = dotenvy::var("NETWORK_ID").unwrap();
        let chain_id = dotenvy::var("CHAIN_ID").unwrap().parse().unwrap();
        let rpc_url = dotenvy::var("RPC_URL").unwrap().parse().unwrap();

        let proxy_client = KadenaProxyClient::new(
            ProxyConf::new_with_url(kadena_proxy_url),
            ChainwebConf::new(rpc_url, network_id, chain_id),
        );

        Context {
            default_pubkey_str: dotenvy::var("DEFAULT_PUBKEY").unwrap(),
            default_privkey_str,
            default_privkey,
            vault_token: dotenvy::var("VAULT_TOKEN").unwrap(),
            vault_address: Url::parse(&std::env::var("VAULT_ADDRESS").unwrap()).unwrap(),
            vault_key_id: dotenvy::var("VAULT_KEY_ID").unwrap(),
            client: Arc::new(proxy_client),
        }
    });
}
