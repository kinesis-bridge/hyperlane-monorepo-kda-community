#![allow(clippy::enum_variant_names)]
#![allow(missing_docs)]

use std::sync::Arc;

use async_trait::async_trait;
use kadena_client::signers::Signer;
use tracing::instrument;

use hyperlane_core::{
    ChainResult, HyperlaneChain, HyperlaneContract, HyperlaneDomain,
    HyperlaneMessage, HyperlaneProvider, MultisigIsm, H256, ChainCommunicationError,
};

use crate::contracts::i_multisig_ism::IMultisigIsm;
use crate::{KadenaProvider, ConnectionConf};
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
        let (api_conf, proxy_conf) = conf.into();

        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(api_conf),
            Arc::new(proxy_conf),
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
            self.contract.provider().connection_conf().clone(),
            self.contract.provider().kadena_proxy_config().clone(),
            self.contract.provider().signer().clone()
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
        #[derive(serde::Deserialize)]
        struct ValidatorsAndThresholdJson
        {
            validators: Vec<String>,
            threshold: u8
        }

        let validators_and_threshold_value = self
            .contract
            .validators_and_threshold()
            .local()
            .await
            .map_err(|_| ChainCommunicationError::from_other_str("Error returned while doing local"))?
            .result()
            .map_err(|_| ChainCommunicationError::from_other_str("Error returned while calling nonce"))?;

        let validators_and_threshold: ValidatorsAndThresholdJson = serde_json::from_value(validators_and_threshold_value)
            .map_err(|_| ChainCommunicationError::from_other_str("Error returned while parsing validators and threshold"))?;
   
        
        let validators: Vec<String> = validators_and_threshold.validators;

        let decoded_validators: Result<Vec<H256>, ChainCommunicationError> = validators.iter()
        .map(|validator| {
            let bytes = hex::decode(validator).map_err(|_| ChainCommunicationError::from_other_str("Invalid hex string"))?;
            let bytes_array: [u8; 32] = bytes[..].try_into().map_err(|_| ChainCommunicationError::from_other_str("Invalid byte length"))?;
            Ok(H256::from(bytes_array))
        })
        .collect();
            
        let decoded_validators = decoded_validators?;

        Ok((decoded_validators, validators_and_threshold.threshold))
    }
}
