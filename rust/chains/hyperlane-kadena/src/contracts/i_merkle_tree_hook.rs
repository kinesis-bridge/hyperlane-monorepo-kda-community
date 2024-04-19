use std::sync::Arc;

use crate::{provider, KadenaProvider};
use async_trait::async_trait;
use hyperlane_core::ChainCommunicationError;
use kadena_client::{
    contract::{Contract, KadenaProxyProvider},
    contract_call::ContractCall,
    error::KadenaClientError,
    event::{Event, EventData},
    models::{CommandDto, EventDataDto, EventParamMonoType, EventParamType},
};

pub struct InsertedIntoTreeEventData {
    message_id: String,
    index: String,
}

impl TryFrom<EventDataDto> for InsertedIntoTreeEventData {
    type Error = KadenaClientError;
    fn try_from(event_data_dto: EventDataDto) -> Result<Self, Self::Error> {
        let args = Self::check_params(event_data_dto.params, Self::params())?;
        let message_id = args[0].to_string();
        let index = args[1].to_string();

        Ok(Self { message_id, index })
    }
}

impl EventData for InsertedIntoTreeEventData {
    fn params() -> &'static [EventParamType] {
        &[
            EventParamType::MonoType(EventParamMonoType::String), 
            EventParamType::MonoType(EventParamMonoType::String),
        ]
    }
}

pub struct InsertedIntoTreeEvent<'a> {
    contract: &'a IMerlkeTreeHook,
}

impl<'a> InsertedIntoTreeEvent<'a> {
    const EVENT_NAME: &'static str = "INSERTED_INTO_TREE";

    pub fn new(contract: &'a IMerlkeTreeHook) -> Self {
        Self { contract }
    }
}

impl Event for InsertedIntoTreeEvent<'_> {
    type DataType = InsertedIntoTreeEventData;
    type Error = KadenaClientError;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

pub struct CountCall<'a> {
    contract: &'a IMerlkeTreeHook,
    gas_limit: Option<u64>,
}

impl CountCall<'_> {
    const METHOD_NAME: &'static str = "count";
    pub fn new(contract: &IMerlkeTreeHook) -> CountCall {
        CountCall {
            contract,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for CountCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{})",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }
}

pub struct LatestCheckpointCall<'a> {
    contract: &'a IMerlkeTreeHook,
    gas_limit: Option<u64>,
}

impl LatestCheckpointCall<'_> {
    const METHOD_NAME: &'static str = "latest-checkpoint";
    pub fn new(contract: &IMerlkeTreeHook) -> LatestCheckpointCall {
        LatestCheckpointCall {
            contract,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for LatestCheckpointCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{})",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }
}

pub struct TreeCall<'a> {
    contract: &'a IMerlkeTreeHook,
    gas_limit: Option<u64>,
}

impl TreeCall<'_> {
    const METHOD_NAME: &'static str = "tree";
    pub fn new(contract: &IMerlkeTreeHook) -> TreeCall {
        TreeCall {
            contract,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for TreeCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{})",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }
}

#[derive(Clone, Debug)]
pub struct IMerlkeTreeHook {
    provider: Arc<KadenaProvider>,
}

impl IMerlkeTreeHook {
    // refers to the mailbox module unless the merkle tree hook is not implemented on the pact side
    const MODULE_NAME: &'static str = "mailbox";

    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self { provider }
    }

    pub fn count(&self) -> CountCall {
        CountCall::new(self)
    }

    pub fn latest_checkpoint(&self) -> LatestCheckpointCall {
        LatestCheckpointCall::new(self)
    }

    pub fn tree(&self) -> TreeCall {
        TreeCall::new(self)
    }

    pub fn inserted_into_tree_event(&self) -> InsertedIntoTreeEvent {
        InsertedIntoTreeEvent::new(self)
    }
}

impl Contract for IMerlkeTreeHook {
    fn module_name(&self) -> &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}
