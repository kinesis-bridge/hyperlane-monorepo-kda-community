#![allow(clippy::enum_variant_names)]
#![allow(missing_docs)]

use std::sync::Arc;

use async_trait::async_trait;
use kadena_client::signers::Signer;
use tracing::{instrument, warn};

use hyperlane_core::{
    ChainCommunicationError, ChainResult, HyperlaneChain, HyperlaneContract, HyperlaneDomain,
    HyperlaneMessage, HyperlaneProvider, InterchainSecurityModule, ModuleType, H256, U256,
};
use num_traits::cast::FromPrimitive;

use crate::contracts::i_interchain_security_module::IInterchainSecurityModule;
use crate::{ConnectionConf, KadenaProvider};
use kadena_client::contract::Contract;
use kadena_client::contract_call::ContractCall;

/// A reference to an InterchainSecurityModule contract on some Kadena chain
#[derive(Debug)]
pub struct KadenaInterchainSecurityModule {
    contract: Arc<IInterchainSecurityModule>,
    domain: HyperlaneDomain,
}

impl KadenaInterchainSecurityModule {
    /// Create a reference to isp
    #[allow(unused)]
    pub fn new(conf: &ConnectionConf, domain: &HyperlaneDomain, signer: Arc<dyn Signer>) -> Self {
        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(conf.into()),
            signer.clone(),
        ));
        Self {
            contract: Arc::new(IInterchainSecurityModule::new(provider)),
            domain: domain.clone(),
        }
    }
}

impl HyperlaneChain for KadenaInterchainSecurityModule {
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

impl HyperlaneContract for KadenaInterchainSecurityModule {
    fn address(&self) -> H256 {
        self.contract.address().into()
    }
}

#[async_trait]
impl InterchainSecurityModule for KadenaInterchainSecurityModule {
    #[instrument]
    async fn module_type(&self) -> ChainResult<ModuleType> {
        let module = self
            .contract
            .module_type()
            .local_typed()
            .await
            .map_err(ChainCommunicationError::from_other)?;

        if let Some(module_type) = ModuleType::from_u8(module as u8) {
            Ok(module_type)
        } else {
            warn!(%module, "Unknown module type");
            Ok(ModuleType::Unused)
        }
    }

    #[instrument]
    async fn dry_run_verify(
        &self,
        message: &HyperlaneMessage,
        metadata: &[u8],
    ) -> ChainResult<Option<U256>> {
        unimplemented!("Required by Aggregation ISM which is not supported yet")
        /*
        let tx = self.contract.verify(
            metadata.to_owned().into(),
            RawHyperlaneMessage::from(message).to_vec().into(),
        );
        let (verifies, gas_estimate) = try_join(tx.call(), tx.estimate_gas()).await?;
        if verifies {
            Ok(Some(gas_estimate.into()))
        } else {
            Ok(None)
        }
        */
    }
}
