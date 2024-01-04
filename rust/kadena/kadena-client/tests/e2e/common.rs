pub mod prelude {
    pub use more_asserts::*;
    use once_cell::sync::Lazy;
    use ed25519_dalek::SigningKey;
    use std::{path::Path, sync::Arc};
    use url::Url;
    use kadena_client::{apis::configuration::{Configuration as ProxyConf, ConnectionConf}, contract::{Contract, KadenaProxyProvider}, contract_call::ContractCall, models::CommandDto, signers::Signer};

    pub struct TestProvider {
        connection_conf: Arc<ConnectionConf>,
        proxy_conf: Arc<ProxyConf>,
        signer: Arc<dyn Signer>,
    }

    impl KadenaProxyProvider for TestProvider {
        fn connection_conf(&self) -> Arc<ConnectionConf> {
            self.connection_conf.clone()
        }

        fn kadena_proxy_config(&self) -> Arc<ProxyConf> {
            self.proxy_conf.clone()
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
        cmd: CommandDto,
    }

    impl AddTwoNumbersCall<'_> {
        pub async fn new(
            contract: &TestContract,
            a: u64,
            b: u64,
        ) -> AddTwoNumbersCall {
            let cmd = contract.build_pact_tx_with_expr(&format!("(+ {} {})", a, b)).await.unwrap();
            AddTwoNumbersCall {
                contract,
                a,
                b,
                cmd,
            }
        }
    }

    impl ContractCall for AddTwoNumbersCall<'_> {
        fn get_contract(&self) -> &dyn Contract {
            self.contract
        }

        fn get_cmd(&self) -> &CommandDto {
            &self.cmd
        }
    }


    pub struct TestContract {
        provider: Arc<dyn KadenaProxyProvider + Sync + Send>,
    }

    impl TestContract {
        const MODULE_NAME: &'static str = "test";

        pub fn new(signer: Arc<dyn Signer>) -> Self {
            let provider = Arc::new(TestProvider {
                connection_conf: Arc::new(CONTEXT.conf.clone()),
                proxy_conf: Arc::new(CONTEXT.proxy_conf.clone()),
                signer,
            });
            TestContract {
                provider
            }
        }

        pub async fn add_two_numbers(&self, a: u64, b: u64) -> AddTwoNumbersCall {
            AddTwoNumbersCall::new(self, a, b).await
        }
    }

    impl Contract for TestContract {
        fn get_module_name(&self) ->  &'static str {
            Self::MODULE_NAME
        }

        fn provider(&self) ->  Arc<dyn KadenaProxyProvider + Send + Sync> {
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

        pub proxy_conf: ProxyConf,
        pub conf: ConnectionConf,
    }

    pub static CONTEXT: Lazy<Context> = Lazy::new(|| {
        dotenvy::from_path(Path::new("tests/e2e/.env")).unwrap();
        let default_privkey_str = dotenvy::var("DEFAULT_PRIVKEY").unwrap();
        let privkey_bytes = hex::decode(&default_privkey_str).unwrap();
        let default_privkey = SigningKey::from_bytes(&privkey_bytes.try_into().unwrap());

        let kadena_proxy_url = dotenvy::var("KADENA_PROXY_URL").unwrap();
        let proxy_conf = ProxyConf::new_with_base_path(kadena_proxy_url).unwrap();

        let network_id = dotenvy::var("NETWORK_ID").unwrap();
        let chain_id = dotenvy::var("CHAIN_ID").unwrap();
        let rpc_url = Url::parse(&dotenvy::var("RPC_URL").unwrap()).unwrap();
        let conf = ConnectionConf::new(rpc_url, network_id, chain_id.parse().unwrap());

        Context {
            default_pubkey_str: dotenvy::var("DEFAULT_PUBKEY").unwrap(),
            default_privkey_str,
            default_privkey,
            vault_token: dotenvy::var("VAULT_TOKEN").unwrap(),
            vault_address: Url::parse(&std::env::var("VAULT_ADDRESS").unwrap()).unwrap(),
            vault_key_id: dotenvy::var("VAULT_KEY_ID").unwrap(),
            proxy_conf,
            conf,
        }
    });
}

