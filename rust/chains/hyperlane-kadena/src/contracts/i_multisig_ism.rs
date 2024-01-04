use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::{contract::{Contract, KadenaProxyProvider}, models::CommandDto, contract_call::ContractCall};
use anyhow::Result;


pub struct ValidatorsAndThresholdCall<'a> {
    contract: &'a IMultisigIsm,
    message: String,
    cmd: CommandDto,
}

impl ValidatorsAndThresholdCall<'_> {
    const METHOD_NAME: &'static str = "validators-and-threshold";
    pub async fn new(
        contract: &IMultisigIsm,
        message: String,
    ) -> Result<ValidatorsAndThresholdCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} {})",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
                message,
            ),
        ).await?;

        Ok(ValidatorsAndThresholdCall {
            contract,
            message,
            cmd,
        })
    }
}

impl ContractCall for ValidatorsAndThresholdCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}

#[derive(Clone, Debug)]
pub struct IMultisigIsm {
    provider: Arc<KadenaProvider>,
}

impl IMultisigIsm {
    const MODULE_NAME: &'static str = "multisig-ism";
    
    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }

    pub async fn validators_and_threshold(&self, message: String) -> Result<ValidatorsAndThresholdCall> {
        ValidatorsAndThresholdCall::new(
            self,
            message,
        ).await
    }
}

impl Contract for IMultisigIsm {
    fn get_module_name(&self) ->  &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) ->  Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}