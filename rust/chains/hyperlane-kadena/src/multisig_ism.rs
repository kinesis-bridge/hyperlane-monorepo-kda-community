#![allow(warnings)]

use std::sync::Arc;
use crate::contracts::i_multisig_ism::IMultisigIsm;

use async_trait::async_trait;
use tracing::{info, instrument, warn};
use kadena_client::contract::{Contract, KadenaProxyProvider};

use hyperlane_core::{
    Announcement, ChainCommunicationError, ChainResult, ContractLocator, HyperlaneChain,
    HyperlaneContract, HyperlaneDomain, SignedType, TxOutcome, H160, H256, H512,
    U256,
};

use crate::provider::KadenaProvider;

/// A reference to a KadenaMultisigIsm contract on some Kadena chain
#[derive(Debug)]
pub struct KadenaMultisigIsm {
    pub provider: Arc<KadenaProvider>,
    pub contract: Arc<IMultisigIsm>,
    pub domain: HyperlaneDomain,
}

impl KadenaMultisigIsm {
    /// Create a new KadenaMultisigIsm
    pub fn new(provider: Arc<KadenaProvider>, locator: &ContractLocator) -> Self {
        let contract = Arc::new(IMultisigIsm::new(provider.clone()));
        Self {
            provider,
            contract,
            domain: locator.domain.clone(),
        }
    }
}

impl HyperlaneContract for KadenaMultisigIsm {
    fn address(&self) -> H256 {
        // TODO: Implement when the contract and encoding are ready
        let mut addr_vec = self.contract.get_module_name().as_bytes().to_vec();
        addr_vec.resize(32, 0);
        let addr_vec: [u8;32] = addr_vec.try_into().unwrap_or([0;32]);
        H256::from(addr_vec)
    }
}

impl HyperlaneChain for KadenaMultisigIsm {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }

    fn provider(&self) -> Box<dyn hyperlane_core::HyperlaneProvider> {
        Box::new(
            KadenaProvider::new(
                self.provider.domain().clone(),
                self.provider.connection_conf().clone(),
                self.provider.kadena_proxy_config().clone(),
                self.provider.signer().clone(),
            )
        )
    }
}