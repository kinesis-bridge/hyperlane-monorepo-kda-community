#![allow(clippy::enum_variant_names)]
#![allow(missing_docs)]

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use kadena_client::signers::Signer;
use tracing::instrument;

use hyperlane_core::{
    ChainCommunicationError, ChainResult, HyperlaneChain, HyperlaneContract, HyperlaneDomain,
    HyperlaneMessage, HyperlaneProvider, MultisigIsm, H160, H256,
};

use crate::contracts::i_multisig_ism::IMultisigIsm;
use crate::{ConnectionConf, KadenaProvider};
use kadena_client::contract::Contract;
use kadena_client::contract_call::ContractCall;

/// A reference to an MultisigIsm contract on some Kadena chain
#[derive(Debug)]
pub struct KadenaMultisigIsm {
    contract: Arc<IMultisigIsm>,
    domain: HyperlaneDomain,
}

impl KadenaMultisigIsm {
    /// Create a reference to a kadena multisig ism
    pub fn new(conf: &ConnectionConf, domain: &HyperlaneDomain, signer: Arc<dyn Signer>) -> Self {
        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(conf.into()),
            signer.clone(),
        ));

        Self {
            contract: Arc::new(IMultisigIsm::new(provider)),
            domain: domain.clone(),
        }
    }
}

impl HyperlaneChain for KadenaMultisigIsm {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }

    fn provider(&self) -> Box<dyn HyperlaneProvider> {
        Box::new(KadenaProvider::new(
            self.domain.clone(),
            self.contract.provider().proxy_client().clone(),
            self.contract.provider().signer().clone(),
        ))
    }
}

impl HyperlaneContract for KadenaMultisigIsm {
    fn address(&self) -> H256 {
        self.contract.address().into()
    }
}

#[async_trait]
impl MultisigIsm for KadenaMultisigIsm {
    #[instrument(err)]
    async fn validators_and_threshold(
        &self,
        _message: &HyperlaneMessage,
    ) -> ChainResult<(Vec<H256>, u8)> {
        #[derive(Debug, serde::Deserialize)]
        struct ValidatorsAndThresholdJson {
            validators: Vec<String>,
            threshold: HashMap<String, u8>,
        }

        let validators_and_threshold_value = self
            .contract
            .validators_and_threshold()
            .local()
            .await
            .map_err(ChainCommunicationError::from_other)?
            .result()
            .map_err(ChainCommunicationError::from_other)?;

        let validators_and_threshold: ValidatorsAndThresholdJson =
            serde_json::from_value(validators_and_threshold_value).map_err(|_| {
                ChainCommunicationError::from_other_str(
                    "Error returned while parsing validators and threshold",
                )
            })?;

        let validators: Vec<String> = validators_and_threshold.validators;

        let decoded_validators = validators
            .iter()
            .map(|validator| {
                let validator = validator
                    .trim()
                    .strip_prefix("0x")
                    .unwrap_or(&validator);
                let bytes = hex::decode(validator)
                    .map_err(|_| ChainCommunicationError::from_other_str("Invalid hex string"))?;
                let bytes_array: [u8; 20] = bytes[..]
                    .try_into()
                    .map_err(|_| ChainCommunicationError::from_other_str("Invalid byte length"))?;
                Ok(H256::from(H160::from(bytes_array)))
            })
            .collect::<Result<Vec<H256>, ChainCommunicationError>>()?;

        let threshhold = validators_and_threshold
            .threshold
            .iter()
            .next()
            .ok_or(ChainCommunicationError::from_other_str(
                "No threshold found",
            ))?
            .1;

        Ok((decoded_validators, *threshhold))
    }
}
