#![allow(warnings)]

use std::sync::Arc;
use crate::contracts::i_interchain_gas_paymaster::IInterchainGasPaymaster;

use async_trait::async_trait;
use tracing::{info, instrument, warn};
use kadena_client::contract::{Contract, KadenaProxyPovider};

use hyperlane_core::{
    Announcement, ChainCommunicationError, ChainResult, ContractLocator, HyperlaneChain,
    HyperlaneContract, HyperlaneDomain, SignedType, TxOutcome, H160, H256, H512,
    U256,
};

use crate::provider::KadenaProvider;

/// A reference to a KadenaInterchainGasPaymaster contract on some Kadena chain
#[derive(Debug)]
pub struct KadenaInterchainGasPaymaster {
    pub(crate) provider: Arc<KadenaProvider>,
    pub(crate) contract: Arc<IInterchainGasPaymaster>,
    pub(crate) domain: HyperlaneDomain,
}

impl KadenaInterchainGasPaymaster {
    /// Create a new KadenaInterchainGasPaymaster
    pub fn new(provider: Arc<KadenaProvider>, locator: &ContractLocator) -> Self {
        let contract = Arc::new(IInterchainGasPaymaster::new(provider.clone()));
        Self {
            provider,
            contract,
            domain: locator.domain.clone(),
        }
    }
}

impl HyperlaneContract for KadenaInterchainGasPaymaster {
    fn address(&self) -> H256 {
        // TODO: Implement when the contract and encoding are ready
        let mut addr_vec = self.contract.module_name().as_bytes().to_vec();
        addr_vec.resize(32, 0);
        let addr_vec: [u8;32] = addr_vec.try_into().unwrap_or([0;32]);
        H256::from(addr_vec)
    }
}

impl HyperlaneChain for KadenaInterchainGasPaymaster {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }

    fn provider(&self) -> Box<dyn hyperlane_core::HyperlaneProvider> {
        Box::new(
            KadenaProvider::new(
                self.provider.domain().clone(),
                self.provider.connection_conf().clone(),
                self.provider.kadena_proxy_config().clone(),
            )
        )
    }
}