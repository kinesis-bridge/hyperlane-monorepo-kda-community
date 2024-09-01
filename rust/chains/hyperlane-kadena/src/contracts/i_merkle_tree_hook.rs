use std::sync::Arc;

use crate::{provider, KadenaProvider};
use async_trait::async_trait;
use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use hyperlane_core::{accumulator::incremental::IncrementalMerkle, ChainCommunicationError, H256};
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
    type Output = u32;

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

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        Ok(self.local().await?.result()?.as_u64().ok_or_else(|| {
            KadenaClientError::TypeConversionError("Count is not a u64".to_string())
        })? as u32)
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
    type Output = ();

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

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        let res = self.local().await?.result()?;
        Ok(())
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
    type Output = IncrementalMerkle;

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

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        let tree = self.local().await?.result()?;
        let tree_array = tree.as_array().ok_or_else(|| {
            KadenaClientError::TypeConversionError("Tree is not an array".to_string())
        })?;

        let tree_array = tree_array
            .iter()
            .map(|x| {
                let base64_str = x.as_str().ok_or_else(|| {
                    KadenaClientError::TypeConversionError(
                        "Tree array element is not a string".to_string(),
                    )
                })?;
                let decoded_bytes = BASE64_URL_SAFE_NO_PAD
                    .decode(base64_str)
                    .map_err(KadenaClientError::from)?;
                if decoded_bytes.len() != H256::len_bytes() {
                    return Err(KadenaClientError::TypeConversionError(
                        "Decoded tree array element is not 32 bytes".to_string(),
                    ));
                }
                Ok(H256::from_slice(&decoded_bytes))
            })
            .collect::<Result<Vec<H256>, KadenaClientError>>()?;

        // Convert Vec<H256> to [H256; 32]
        let tree_array: [H256; 32] = tree_array.try_into().map_err(|_| {
            KadenaClientError::TypeConversionError(
                "Failed to convert tree array to [H256; 32]".to_string(),
            )
        })?;

        let count = self.contract.count().local_typed().await? as usize;

        let inc_tree = IncrementalMerkle {
            branch: tree_array,
            count,
        };

        Ok(inc_tree)
    }
}

#[derive(Clone, Debug)]
pub struct IMerlkeTreeHook {
    provider: Arc<KadenaProvider>,
}

impl IMerlkeTreeHook {
    // refers to the mailbox module unless the merkle tree hook is not implemented on the pact side
    const MODULE_NAME: &'static str = "merkle-tree-hook";

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
