use std::sync::Arc;
use async_trait::async_trait;
use crate::models::{EventDataDto, CommandDto, BuildPactTxDto};
use crate::apis::kadena_proxy_api::get_events;
use crate::apis::Error;
use crate::apis::kadena_proxy_api::{self, GetEventsError, BuildTxError};
use crate::apis::configuration::{Configuration as KadenaProxyConf, ConnectionConf};
use crate::signers::Signer;

pub trait KadenaProxyProvider {
    fn connection_conf(&self) -> Arc<ConnectionConf>;
    fn kadena_proxy_config(&self) -> Arc<KadenaProxyConf>;
    fn signer(&self) -> Arc<dyn Signer>;
}

#[async_trait]
pub trait Contract: Send + Sync {
    fn get_module_name(&self) -> &'static str;

    fn get_namespace(&self) -> &'static str {
        "free"
    }

    fn get_pubkey(&self) -> String {
        "83a5cfcdcbec1d513a2d02ab0f0b61e30c9be22d9c09af001affe79885414450".to_string()
    }

    fn get_account(&self) -> String {
        format!("k:{}", self.get_pubkey())
    }

    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync>;

    async fn query_events_range(&self, event_name: &str, from: u64, to: u64) -> Result<Vec<EventDataDto>, Error<GetEventsError>> {
        let provider = self.provider();
        let conn_conf = provider.connection_conf();
        let mut events = get_events(
            &provider.kadena_proxy_config(),
            &conn_conf.url.to_string(),
            &conn_conf.network_id,
            conn_conf.chain_id.into(),
            from,
            to,
        )
        .await?;
        events.retain(|event| event.name == event_name && event.module.name == self.get_module_name());

        Ok(events)
    }

    async fn build_pact_tx_with_expr(&self, expr: &str) -> Result<CommandDto, Error<BuildTxError>> {
        let provider = self.provider();
        let conf = provider.connection_conf();
        let proxy_conf = provider.kadena_proxy_config();
        let build_tx_dto = BuildPactTxDto::new(
            conf.url.to_string(),
            conf.network_id.clone(),
            conf.chain_id as u32,
            expr.to_owned(),
            self.get_pubkey(),
            self.get_account(),
        );

        kadena_proxy_api::build_tx(&proxy_conf, build_tx_dto)
            .await
    }
}