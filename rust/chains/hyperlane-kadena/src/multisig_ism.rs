#![allow(clippy::enum_variant_names)]
#![allow(missing_docs)]

use std::sync::Arc;

use async_trait::async_trait;
use kadena_client::signers::Signer;
use tracing::instrument;

use hyperlane_core::{
    ChainCommunicationError, ChainResult, HyperlaneChain, HyperlaneContract, HyperlaneDomain,
    HyperlaneMessage, HyperlaneProvider, MultisigIsm, H256,
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
        message: &HyperlaneMessage,
    ) -> ChainResult<(Vec<H256>, u8)> {
        let validators_and_threshold = self
            .contract
            .validators_and_threshold(message)
            .local_typed()
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok(validators_and_threshold)
    }
}
