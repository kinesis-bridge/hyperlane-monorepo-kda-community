use std::sync::Arc;

use crate::{provider, KadenaProvider};

use anyhow::Result;
use async_trait::async_trait;
use kadena_client::{
    contract::{Contract, KadenaProxyProvider},
    contract_call::ContractCall,
    models::CommandDto,
};

pub struct ValidatorsAndThresholdCall<'a> {
    contract: &'a IMultisigIsm,
    gas_limit: Option<u64>,
}

impl ValidatorsAndThresholdCall<'_> {
    const METHOD_NAME: &'static str = "validators-and-threshold";
    pub fn new(contract: &IMultisigIsm) -> ValidatorsAndThresholdCall {
        ValidatorsAndThresholdCall {
            contract,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for ValidatorsAndThresholdCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto> {
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
pub struct IMultisigIsm {
    provider: Arc<KadenaProvider>,
}

impl IMultisigIsm {
    const MODULE_NAME: &'static str = "ism";

    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self { provider }
    }

    pub fn validators_and_threshold(&self) -> ValidatorsAndThresholdCall {
        ValidatorsAndThresholdCall::new(self)
    }
}

impl Contract for IMultisigIsm {
    fn module_name(&self) -> &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}
