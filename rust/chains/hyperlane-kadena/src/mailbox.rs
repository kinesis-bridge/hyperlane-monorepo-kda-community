use async_trait::async_trait;
use hyperlane_core::{ChainCommunicationError, Indexer, LogMeta};
use tracing::{instrument, info};
use crate::KadenaProvider;
use crate::contracts::i_mailbox::IMailbox;

use std::num::NonZeroU64;
use std::ops::RangeInclusive;
use std::sync::Arc;
use std::vec;

use hyperlane_core::{
    ChainResult,
    HyperlaneChain, HyperlaneContract,
    HyperlaneDomain, HyperlaneMessage, HyperlaneProvider, Mailbox,
    TxCostEstimate, TxOutcome, H256, U256,
};

use kadena_client::{apis::kadena_proxy_api, contract::Contract};

/// The Kadena mailbox contract.
#[derive(Clone)]
pub struct KadenaMailbox {
    domain: HyperlaneDomain,
}

impl KadenaMailbox {
    /// Creates a new instance of the Kadena mailbox contract.
    pub fn new() -> Self {
        todo!("Initialize the mailbox contract")
    }
}

impl HyperlaneContract for KadenaMailbox {
    fn address(&self) -> H256 {
        todo!("Return the address of the mailbox contract")
    }
}

impl HyperlaneChain for KadenaMailbox {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }

    fn provider(&self) -> Box<dyn HyperlaneProvider> {
        todo!()
    }
}

impl std::fmt::Debug for KadenaMailbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self as &dyn HyperlaneContract)
    }
}

#[async_trait]
impl Mailbox for KadenaMailbox {
        #[instrument(err, ret, skip(self))]
        async fn count(&self, _maybe_lag: Option<NonZeroU64>) -> ChainResult<u32> {
            todo!("Return the number of messages in the mailbox")
        }
    
        #[instrument(err, ret, skip(self))]
        async fn delivered(&self, _id: H256) -> ChainResult<bool> {
            todo!("Return whether the message has been delivered")
        }
    /* 
        #[instrument(err, ret, skip(self))]
        async fn tree(&self, _lag: Option<NonZeroU64>) -> ChainResult<IncrementalMerkle> {
            todo!("Return the inbox tree")
        }
    
        #[instrument(err, ret, skip(self))]
        async fn latest_checkpoint(&self, _lag: Option<NonZeroU64>) -> ChainResult<Checkpoint> {
            todo!("Return the latest checkpoint")
        }

    */
    
        #[instrument(err, ret, skip(self))]
        async fn default_ism(&self) -> ChainResult<H256> {
            todo!("Return the default ISM")
        }
    
        #[instrument(err, ret, skip(self))]
        async fn recipient_ism(&self, _recipient: H256) -> ChainResult<H256> {
            todo!("Return the recipient ISM")
        }
    
        #[instrument(err, ret, skip(self))]
        async fn process(
            &self,
            message: &HyperlaneMessage,
            metadata: &[u8],
            _tx_gas_limit: Option<U256>,
        ) -> ChainResult<TxOutcome> {
            todo!("Process the message")
        }
    
        #[instrument(err, ret, skip(self))]
        async fn process_estimate_costs(
            &self,
            _message: &HyperlaneMessage,
            _metadata: &[u8],
        ) -> ChainResult<TxCostEstimate> {
            todo!("Estimate the costs of processing the message")
        }
    
        fn process_calldata(&self, _message: &HyperlaneMessage, _metadata: &[u8]) -> Vec<u8> {
            todo!()
        } 
}

/// Struct that retrieves event data for a Kadena Mailbox contract
#[derive(Debug, Clone)]
pub struct KadenaMailboxIndexer {
    provider: Arc<KadenaProvider>,
    contract: Arc<IMailbox>,
}

impl KadenaMailboxIndexer {
    /// The number of blocks to wait for before considering a block finalized.
    const FINALIZED_BLOCK_DEPTH: u64 = 6;

    /// Creates a new instance of the Kadena mailbox indexer.
    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        let contract = Arc::new(IMailbox::new(provider.clone()));
        Self {
            provider,
            contract,
        }
    }
    
    #[instrument(level = "debug", err, ret, skip(self))]
    async fn get_finalized_block_number(&self) -> ChainResult<u32> {
        let conn_conf = self.provider.connection_conf();
        let block_number = kadena_proxy_api::get_height(
            &self.provider.kadena_proxy_config(),
            &conn_conf.url.to_string(),
            &conn_conf.network_id,
            Some(Self::FINALIZED_BLOCK_DEPTH),
        )
        .await
        .map_err(ChainCommunicationError::from_other)?;
        Ok(block_number as u32)
    }
}

#[async_trait]
impl Indexer<HyperlaneMessage> for KadenaMailboxIndexer {
    #[instrument(level = "debug", err, ret, skip(self))]
    async fn fetch_logs(
        &self,
        range: RangeInclusive<u32>,
    ) -> ChainResult<Vec<(HyperlaneMessage, LogMeta)>> {
        info!(
            ?range,
            "Fetching KadenaMailboxIndexer HyperlaneMessage logs"
        );

        let _events = self
            .contract
            .query_events_range(
                "Dispatch", // TODO: get the event name from the contract
                *range.start() as u64,
                *range.end() as u64,
            )
            .await
            .map_err(ChainCommunicationError::from_other)?;
        // TODO: Convert events to HyperlaneMessage logs when the Kadena mailbox contract is ready.

        let events = vec![];

        Ok(events)
    }

    #[instrument(level = "debug", err, ret, skip(self))]
    async fn get_finalized_block_number(&self) -> ChainResult<u32> {
        self.get_finalized_block_number().await
    }
}
