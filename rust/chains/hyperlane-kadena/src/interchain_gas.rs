#![allow(missing_docs)]

use std::ops::RangeInclusive;
use std::sync::Arc;

use async_trait::async_trait;

use hyperlane_core::{
    ChainCommunicationError, ChainResult, HyperlaneChain,
    HyperlaneContract, HyperlaneDomain, HyperlaneProvider, Indexer, InterchainGasPaymaster,
    InterchainGasPayment, LogMeta, SequenceIndexer, H256,
};
use kadena_client::event::Event;
use kadena_client::contract::{Contract, KadenaProxyProvider};

use kadena_client::signers::Signer;
use tracing::instrument;

use crate::contracts::i_interchain_gas_paymaster::IInterchainGasPaymaster;

use crate::{KadenaProvider, ConnectionConf};

#[derive(Debug)]
/// Struct that retrieves event data for an Ethereum InterchainGasPaymaster
pub struct KadenaInterchainGasPaymasterIndexer {
    contract: Arc<IInterchainGasPaymaster>,
    provider: Arc<KadenaProvider>,
    reorg_period: u32,
}

impl KadenaInterchainGasPaymasterIndexer {
    /// Create new KadenaInterchainGasPaymasterIndexer
    #[allow(dead_code)]
    pub fn new(conf: &ConnectionConf, domain: &HyperlaneDomain, signer: Arc<dyn Signer>, reorg_period: u32) -> Self {
        let (api_conf, proxy_conf) = conf.into();

        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(api_conf),
            Arc::new(proxy_conf),
            signer.clone(),
        ));  

        Self {
            contract: Arc::new(IInterchainGasPaymaster::new(
                provider.clone(),
            )),
            provider,
            reorg_period,
        }
    }
}

#[async_trait]
impl Indexer<InterchainGasPayment> for KadenaInterchainGasPaymasterIndexer {
    #[instrument(err, skip(self))]
    async fn fetch_logs(
        &self,
        range: RangeInclusive<u32>,
    ) -> ChainResult<Vec<(InterchainGasPayment, LogMeta)>> {
        let events = self
            .contract
            .gas_payment_event()
            .query_events_range(range)
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok(events
            .into_iter()
            .map(|event_data| {
                (
                    InterchainGasPayment {
                        message_id: H256::from(event_data.id),
                        destination: event_data.domain,
                        payment: event_data.kda_amount.into(),
                        gas_amount: event_data.gas_amount.into(),
                    },
                    // TODO: add actual log meta when implementing scraper
                    LogMeta::default(),
                )
            })
            .collect())
    }

    #[instrument(level = "debug", err, ret, skip(self))]
    async fn get_finalized_block_number(&self) -> ChainResult<u32> {
        Ok(
            (self
                .provider
                .get_block_number()
                .await
                .map_err(|_| ChainCommunicationError::from_other_str("Error while getting block number"))? as u32)
                    .saturating_sub(self.reorg_period)
        )
    }
}

#[async_trait]
impl SequenceIndexer<InterchainGasPayment> for KadenaInterchainGasPaymasterIndexer {
    async fn sequence_and_tip(&self) -> ChainResult<(Option<u32>, u32)> {
        // The InterchainGasPaymasterIndexerBuilder must return a `SequenceIndexer` type.
        // It's fine if only a blanket implementation is provided for EVM chains, since their
        // indexing only uses the `Index` trait, which is a supertrait of `SequenceIndexer`.
        // TODO: if `SequenceIndexer` turns out to not depend on `Indexer` at all, then the supertrait
        // dependency could be removed, even if the builder would still need to return a type that is both
        // ``SequenceIndexer` and `Indexer`.
        let tip = self.get_finalized_block_number().await?;
        Ok((None, tip))
    }
}

/// A reference to an InterchainGasPaymaster contract on some Kadena chain
#[derive(Debug)]
pub struct KadenaInterchainGasPaymaster {
    contract: Arc<IInterchainGasPaymaster>,
    domain: HyperlaneDomain,
}

impl KadenaInterchainGasPaymaster {
    /// Create a reference to an igp
    #[allow(unused)]
    pub fn new(conf: &ConnectionConf, domain: &HyperlaneDomain, signer: Arc<dyn Signer>) -> Self {
        let (api_conf, proxy_conf) = conf.into();

        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(api_conf),
            Arc::new(proxy_conf),
            signer.clone(),
        ));

        Self {
            contract: Arc::new(IInterchainGasPaymaster::new(
                provider.clone(),
            )),
            domain: domain.clone(),
        }
    }
}

impl HyperlaneChain for KadenaInterchainGasPaymaster {
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

impl HyperlaneContract for KadenaInterchainGasPaymaster {
    fn address(&self) -> H256 {
        self.contract.address().into()
    }
}

#[async_trait]
impl InterchainGasPaymaster for KadenaInterchainGasPaymaster {}
