use std::sync::Arc;
use async_trait::async_trait;
use crate::models::{EventDataDto, CommandDto, BuildPactTxDto};
use crate::apis::kadena_proxy_api::get_events;
use crate::apis::Error;
use crate::apis::kadena_proxy_api::{self, GetEventsError, BuildTxError};
use crate::apis::configuration::{Configuration as KadenaProxyConf, ConnectionConf};
use crate::signers::Signer;
use anyhow::Result;

const CONFIRMATION_DEPTH: u64 = 0;
pub const DEFAULT_GAS_LIMIT: u64 = 100_000;

#[async_trait]
pub trait KadenaProxyProvider {
    fn connection_conf(&self) -> Arc<ConnectionConf>;
    fn kadena_proxy_config(&self) -> Arc<KadenaProxyConf>;
    fn signer(&self) -> Arc<dyn Signer>;

    fn get_gas_price(&self) -> u64 {
        // 1e-12 is the smallest unit of KDA
        1_000_000 // 1e-6 KDA
    }
    
    async fn get_block_number(&self) -> Result<u64> {
        let conf = self.connection_conf();
        let proxy_conf = self.kadena_proxy_config();
        kadena_proxy_api::get_height(&proxy_conf, &conf.url.to_string(), &conf.network_id, Some(CONFIRMATION_DEPTH)).await.map_err(|e| e.into())
    }
}

#[async_trait]
pub trait Contract: Send + Sync {
    fn module_name(&self) -> &'static str;

    // Since Pact smart contract don't have address, we use module name as address for Hyperlane compatibility
    fn address(&self) -> [u8; 32] {
        let module_name = self.module_name();
        let module_name_bytes = module_name.as_bytes();
        let mut address = [0; 32];
        address[..module_name_bytes.len()].copy_from_slice(module_name_bytes);
        address
    }

    fn namespace(&self) -> &'static str {
        "free"
    }

    fn pubkey(&self) -> String {
        "83a5cfcdcbec1d513a2d02ab0f0b61e30c9be22d9c09af001affe79885414450".to_string()
    }

    fn account_name(&self) -> String {
        format!("k:{}", self.pubkey())
    }

    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync>;

    async fn build_pact_tx_with_expr(&self, expr: &str, gas_limit: Option<u64>) -> Result<CommandDto, Error<BuildTxError>> {
        let provider = self.provider();
        let conf = provider.connection_conf();
        let proxy_conf = provider.kadena_proxy_config();
        let build_tx_dto = BuildPactTxDto::new(
            conf.url.to_string(),
            conf.network_id.clone(),
            conf.chain_id as u32,
            expr.to_owned(),
            self.pubkey(),
            self.account_name(),
            gas_limit.unwrap_or(DEFAULT_GAS_LIMIT),
        );

        kadena_proxy_api::build_tx(&proxy_conf, build_tx_dto)
            .await
    }
}