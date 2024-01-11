use std::ops::RangeInclusive;

use async_trait::async_trait;

use crate::{
    apis::{
        kadena_proxy_api::{self, GetEventsError},
        Error,
    },
    contract::Contract,
    models::EventDataDto,
};

pub trait EventData: Send + Sync + TryFrom<EventDataDto> {

}

#[async_trait]
pub trait Event: Send + Sync {
    type DataType: EventData;

    fn event_name(&self) -> &'static str;
    fn contract(&self) -> &dyn Contract;

    async fn query_events_range<Idx: Into<u64> + Send + Sync + Copy>(
        &self,
        range: RangeInclusive<Idx>,
    ) -> Result<Vec<Self::DataType>, Error<GetEventsError>> {
        let contract = self.contract();
        let provider = self.contract().provider();
        let conn_conf = provider.connection_conf();
        let mut events = kadena_proxy_api::get_events(
            &provider.kadena_proxy_config(),
            &conn_conf.url.to_string(),
            &conn_conf.network_id,
            conn_conf.chain_id.into(),
            (*range.start()).into(),
            (*range.end()).into(),
        )
        .await?;
        events.retain(|event| {
            event.module.namespace.as_ref().is_some_and(|ns| ns == contract.namespace())
                && self.event_name() == event.name 
                && contract.module_name() == event.module.name
        });

        Ok(events.into_iter().filter_map(|event| Self::DataType::try_from(event).ok()).collect())
    }
}
