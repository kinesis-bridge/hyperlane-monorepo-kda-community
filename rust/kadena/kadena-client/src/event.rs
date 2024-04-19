use std::ops::RangeInclusive;

use async_trait::async_trait;
use tracing::warn;

use crate::{
    contract::Contract,
    error::KadenaClientError,
    models::{EventDataDto, EventParam, EventParamMonoType, EventParamType},
};

pub trait EventData: Send + Sync + TryFrom<EventDataDto> {
    fn params() -> &'static [EventParamType];

    fn check_params(
        args: Vec<EventParam>,
        params: &[EventParamType],
    ) -> Result<Vec<EventParam>, KadenaClientError> {
        if args.len() != params.len() {
            return Err(KadenaClientError::EventParamsCountMismatchError {
                expected: params.len(),
                actual: args.len(),
            });
        }

        for (arg, param) in args.iter().zip(params.iter()) {
            match param {
                EventParamType::MonoType(mono_type) => {
                    if EventParamMonoType::from(arg) != *mono_type {
                        return Err(KadenaClientError::EventParamsTypeMismatchError {
                            expected: EventParamType::MonoType(*mono_type),
                            actual: arg.clone(),
                        });
                    }
                }
                EventParamType::Number => {
                    if !matches!(
                        arg,
                        EventParam::Integer(_)
                            | EventParam::Float(_)
                            | EventParam::IntObject(_)
                            | EventParam::DecimalObject(_)
                    ) {
                        return Err(KadenaClientError::EventParamsTypeMismatchError {
                            expected: param.clone(),
                            actual: arg.clone(),
                        });
                    }
                }
            }
        }
        Ok(args)
    }
}

#[async_trait]
pub trait Event: Send + Sync {
    type DataType: EventData + TryFrom<EventDataDto, Error = Self::Error>;
    type Error: std::fmt::Debug + Send + Sync + 'static;

    fn event_name(&self) -> &'static str;
    fn contract(&self) -> &dyn Contract;

    async fn query_events_range<Idx: Into<u64> + Send + Sync + Copy>(
        &self,
        range: RangeInclusive<Idx>,
    ) -> Result<Vec<Self::DataType>, KadenaClientError> {
        let contract = self.contract();
        let provider = self.contract().provider();
        let client = provider.proxy_client();
        let mut events = client
            .events((*range.start()).into(), (*range.end()).into())
            .await?;
        events.retain(|event| {
            event
                .module
                .namespace
                .as_ref()
                .is_some_and(|ns| ns == contract.namespace())
                && self.event_name() == event.name
                && contract.module_name() == event.module.name
        });

        Ok(events
            .into_iter()
            .filter_map(|event| {
                let event_name = event.name.clone();
                match Self::DataType::try_from(event) {
                    Ok(data) => Some(data),
                    Err(e) => {
                        warn!(
                            "Failed to convert event data to {} for event {} with error: {:?}",
                            std::any::type_name::<Self::DataType>(),
                            event_name,
                            e,
                        );
                        None
                    }
                }
            })
            .collect())
    }
}
