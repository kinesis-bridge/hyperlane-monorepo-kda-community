use std::sync::Arc;

use crate::{provider, KadenaProvider};

use anyhow::Result;
use hyperlane_core::U256;
use kadena_client::{
    contract::{Contract, KadenaProxyProvider},
    event::{Event, EventData},
    models::EventDataDto,
};

use super::LogMetaProxy;
use super::U256Proxy;

#[derive(Clone, Debug)]
pub struct GasPaymentEventData {
    pub id: [u8; 32],
    pub domain: u32,
    pub gas_amount: U256,
    pub kda_amount: U256,
    pub log: LogMetaProxy,
}

impl TryFrom<EventDataDto> for GasPaymentEventData {
    type Error = anyhow::Error;
    fn try_from(event_data_dto: EventDataDto) -> Result<Self> {
        let params = event_data_dto.params.clone();

        let id = params.get(0).ok_or(anyhow::anyhow!("ID is missing"))?;
        let id_str = id.to_string();
        let id_vec = hex::decode(id_str.strip_prefix("0x").unwrap_or(&id_str))
            .map_err(|_| anyhow::anyhow!("Invalid hex string"))?;
        let mut id = [0u8; 32];
        id[..id_vec.len()].copy_from_slice(&id_vec);

        let domain = params.get(1).ok_or(anyhow::anyhow!("Domain is missing"))?;
        let domain = TryInto::<u64>::try_into(domain.clone())? as u32;

        let gas_amount = params
            .get(2)
            .ok_or(anyhow::anyhow!("Gas amount is missing"))?;
        let gas_amount = U256Proxy::try_from(gas_amount.clone())?.into();

        let kda_amount = params
            .get(3)
            .ok_or(anyhow::anyhow!("KDA amount is missing"))?;
        let kda_amount = U256Proxy::try_from(kda_amount.clone())?.into();

        Ok(Self {
            id,
            domain,
            gas_amount,
            kda_amount,
            log: LogMetaProxy::from(event_data_dto),
        })
    }
}

impl EventData for GasPaymentEventData {}

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
