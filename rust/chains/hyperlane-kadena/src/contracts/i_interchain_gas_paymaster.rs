use std::{str::FromStr, sync::Arc};

use crate::{provider, KadenaProvider};

use hyperlane_core::{H256, U256};
use kadena_client::{
    contract::{Contract, KadenaProxyProvider},
    error::KadenaClientError,
    event::{Event, EventData},
    models::{EventDataDto, EventParamMonoType, EventParamType},
};

use super::LogMetaProxy;
use super::U256Proxy;

#[derive(Clone, Debug)]
pub struct GasPaymentEventData {
    pub id: H256,
    pub domain: u32,
    pub gas_amount: U256,
    pub kda_amount: U256,
    pub log: LogMetaProxy,
}

impl TryFrom<EventDataDto> for GasPaymentEventData {
    type Error = KadenaClientError;
    fn try_from(mut event_data_dto: EventDataDto) -> Result<Self, Self::Error> {
        let args = Self::check_params(std::mem::take(&mut event_data_dto.params), Self::params())?;

        let id = H256::from_str(&args[0].to_string())
            .map_err(|e| KadenaClientError::OtherError(Box::new(e)))?;
        let domain = (&args[1]).try_into()?;
        let gas_amount = U256Proxy::try_from(&args[2])?.into();
        let kda_amount = U256Proxy::try_from(&args[3])?.into();

        Ok(Self {
            id,
            domain,
            gas_amount,
            kda_amount,
            log: event_data_dto.into(),
        })
    }
}

impl EventData for GasPaymentEventData {
    fn params() -> &'static [EventParamType] {
        &[
            EventParamType::MonoType(EventParamMonoType::String),
            EventParamType::MonoType(EventParamMonoType::String),
            EventParamType::MonoType(EventParamMonoType::Integer),
            EventParamType::MonoType(EventParamMonoType::Integer),
        ]
    }
}

pub struct GasPaymentEvent<'a> {
    contract: &'a IInterchainGasPaymaster,
}

impl<'a> GasPaymentEvent<'a> {
    const EVENT_NAME: &'static str = "GAS_PAYMENT";

    pub fn new(contract: &'a IInterchainGasPaymaster) -> Self {
        Self { contract }
    }
}

impl Event for GasPaymentEvent<'_> {
    type DataType = GasPaymentEventData;
    type Error = KadenaClientError;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

#[derive(Clone, Debug)]
pub struct IInterchainGasPaymaster {
    provider: Arc<KadenaProvider>,
}

impl IInterchainGasPaymaster {
    const MODULE_NAME: &'static str = "igp";

    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self { provider }
    }

    pub fn gas_payment_event(&self) -> GasPaymentEvent {
        GasPaymentEvent::new(self)
    }
}

impl Contract for IInterchainGasPaymaster {
    fn module_name(&self) -> &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}
