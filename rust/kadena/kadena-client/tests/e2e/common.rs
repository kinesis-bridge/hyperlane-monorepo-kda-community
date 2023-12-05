pub mod prelude {
    pub use more_asserts::*;
    use once_cell::sync::Lazy;
    use ed25519_dalek::SigningKey;
    use std::path::Path;
    use url::Url;
    use kadena_client::apis::{configuration::{Configuration as ProxyConf, ConnectionConf}, kadena_proxy_api};

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

    pub async fn build_tx_with_pact_expr(pact_expr: &str) -> kadena_client::models::CommandDto {
        let build_tx_dto = kadena_client::models::BuildPactTxDto::new(
            CONTEXT.conf.url.to_string(),
            CONTEXT.conf.network_id.clone(),
            CONTEXT.conf.chain_id as u32,
            pact_expr.to_owned(),
            CONTEXT.default_pubkey_str.clone(),
            format!("k:{}", CONTEXT.default_pubkey_str),
        );

        kadena_proxy_api::build_tx(&CONTEXT.proxy_conf, build_tx_dto)
            .await
            .unwrap()
    }
}

