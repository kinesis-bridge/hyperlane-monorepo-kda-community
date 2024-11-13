use std::collections::HashMap;
use std::sync::Arc;

use crate::{provider, KadenaProvider};

use async_trait::async_trait;
use hyperlane_core::{HyperlaneMessage, H160, H256};
use kadena_client::{
    contract::{Contract, KadenaProxyProvider},
    contract_call::ContractCall,
    error::KadenaClientError,
    models::CommandDto,
};

use super::PactHyperlaneMessage;

pub struct ValidatorsAndThresholdCall<'a> {
    message: HyperlaneMessage,
    contract: &'a IMultisigIsm,
    gas_limit: Option<u64>,
}

impl<'a> ValidatorsAndThresholdCall<'a> {
    const METHOD_NAME: &'static str = "validators-and-threshold";
    pub fn new(
        contract: &'a IMultisigIsm,
        message: &'a HyperlaneMessage,
    ) -> ValidatorsAndThresholdCall<'a> {
        ValidatorsAndThresholdCall {
            message: message.clone(),
            contract,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for ValidatorsAndThresholdCall<'_> {
    type Output = (Vec<H256>, u8);

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
        let pact_msg = PactHyperlaneMessage::from((self.message.clone()));
        let pact_msg_str = serde_json::to_string(&pact_msg)
            .map_err(|e| KadenaClientError::OtherError(Box::new(e)))?;
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{} {})",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                    pact_msg_str
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        #[derive(Debug, serde::Deserialize)]
        struct ValidatorsAndThresholdJson {
            validators: Vec<String>,
            threshold: HashMap<String, u8>,
        }

        let validators_and_threshold_value = self.local().await?.result()?;

        let validators_and_threshold: ValidatorsAndThresholdJson =
            serde_json::from_value(validators_and_threshold_value)
                .map_err(KadenaClientError::from)?;

        let validators: Vec<String> = validators_and_threshold.validators;

        let decoded_validators = validators
            .iter()
            .map(|validator| {
                let validator = validator.trim().strip_prefix("0x").unwrap_or(&validator);
                let bytes = hex::decode(validator).map_err(|_| {
                    KadenaClientError::TypeConversionError("Invalid hex string".to_string())
                })?;
                let bytes_array: [u8; 20] = bytes[..].try_into().map_err(|_| {
                    KadenaClientError::TypeConversionError("Invalid byte length".to_string())
                })?;
                Ok(H256::from(H160::from(bytes_array)))
            })
            .collect::<Result<Vec<H256>, KadenaClientError>>()?;

        let threshhold = validators_and_threshold
            .threshold
            .iter()
            .next()
            .ok_or(KadenaClientError::TypeConversionError(
                "Threshold not found".to_string(),
            ))?
            .1;

        Ok((decoded_validators, *threshhold))
    }
}

#[derive(Clone, Debug)]
pub struct IMultisigIsm {
    provider: Arc<KadenaProvider>,
}

impl IMultisigIsm {
    const MODULE_NAME: &'static str = "merkle-tree-ism";

    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self { provider }
    }

    pub fn validators_and_threshold<'a>(
        &'a self,
        message: &'a HyperlaneMessage,
    ) -> ValidatorsAndThresholdCall {
        ValidatorsAndThresholdCall::new(self, message)
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
