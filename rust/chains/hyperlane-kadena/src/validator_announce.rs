#![allow(warnings)]

use std::sync::Arc;
use crate::contracts::i_validator_announce::IValidatorAnnounce;

use async_trait::async_trait;
use tracing::{info, instrument, warn};
use kadena_client::contract::{Contract, KadenaProxyProvider};

use hyperlane_core::{
    Announcement, ChainCommunicationError, ChainResult, ContractLocator, HyperlaneChain,
    HyperlaneContract, HyperlaneDomain, SignedType, TxOutcome, ValidatorAnnounce, H160, H256, H512,
    U256,
};

use crate::provider::KadenaProvider;

/// A reference to a ValidatorAnnounce contract on some Kadena chain
#[derive(Debug)]
pub struct KadenaValidatorAnnounce {
    pub provider: Arc<KadenaProvider>,
    pub contract: Arc<IValidatorAnnounce>,
    pub domain: HyperlaneDomain,
}

impl KadenaValidatorAnnounce {
    /// Create a new Kadena ValidatorAnnounce
    pub fn new(provider: Arc<KadenaProvider>, locator: &ContractLocator) -> Self {
        let contract = Arc::new(IValidatorAnnounce::new(provider.clone()));
        Self {
            provider,
            contract,
            domain: locator.domain.clone(),
        }
    }
}

impl HyperlaneContract for KadenaValidatorAnnounce {
    fn address(&self) -> H256 {
        // TODO: Implement when the contract and encoding are ready
        let mut addr_vec = self.contract.get_module_name().as_bytes().to_vec();
        addr_vec.resize(32, 0);
        let addr_vec: [u8;32] = addr_vec.try_into().unwrap_or([0;32]);
        H256::from(addr_vec)
    }
}

impl HyperlaneChain for KadenaValidatorAnnounce {
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

#[async_trait]
impl ValidatorAnnounce for KadenaValidatorAnnounce {
    async fn get_announced_storage_locations(
        &self,
        validators: &[H256],
    ) -> ChainResult<Vec<Vec<String>>> {
        let mut storage_locations = vec![];

        // Implement when the contract is ready

        Ok(storage_locations)
    }

    async fn announce_tokens_needed(
        &self,
        _announcement: SignedType<Announcement>,
    ) -> Option<U256> {
        Some(U256::zero())
    }

    #[instrument(err, ret, skip(self))]
    async fn announce(
        &self,
        _announcement: SignedType<Announcement>,
        _tx_gas_limit: Option<U256>,
    ) -> ChainResult<TxOutcome> {
        warn!(
            "Announcing validator storage locations within the agents is not supported on Kadena"
        );
        Ok(TxOutcome {
            transaction_id: H512::zero(),
            executed: false,
            gas_used: U256::zero(),
            gas_price: U256::zero(),
        })
    }
}
