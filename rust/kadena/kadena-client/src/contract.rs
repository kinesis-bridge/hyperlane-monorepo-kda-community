use std::sync::Arc;
use async_trait::async_trait;
use crate::models::EventDataDto;
use crate::apis::kadena_proxy_api::get_events;
use crate::apis::Error;
use crate::apis::kadena_proxy_api::GetEventsError;
use crate::apis::configuration::{Configuration as KadenaProxyConf, ConnectionConf};

pub trait KadenaProxyPovider {
    fn connection_conf(&self) -> &ConnectionConf;
    fn kadena_proxy_config(&self) -> &KadenaProxyConf;
}

#[async_trait]
pub trait Contract {
    fn module_name(&self) -> &str;
    fn provider(&self) -> Arc<dyn KadenaProxyPovider + Send + Sync>;
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
        events.retain(|event| event.name == event_name && event.module.name == self.module_name());

        Ok(events)
    }
}