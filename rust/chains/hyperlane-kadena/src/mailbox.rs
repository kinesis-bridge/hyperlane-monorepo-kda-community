use async_trait::async_trait;
use tracing::instrument;

use std::num::NonZeroU64;

use hyperlane_core::{
    accumulator::incremental::IncrementalMerkle, ChainResult, Checkpoint,
    HyperlaneChain, HyperlaneContract,
    HyperlaneDomain, HyperlaneMessage, HyperlaneProvider, Mailbox,
    TxCostEstimate, TxOutcome, H256, U256,
};

/// The Kadena mailbox contract.
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
    
        #[instrument(err, ret, skip(self))]
        async fn tree(&self, _lag: Option<NonZeroU64>) -> ChainResult<IncrementalMerkle> {
            todo!("Return the inbox tree")
        }
    
        #[instrument(err, ret, skip(self))]
        async fn latest_checkpoint(&self, _lag: Option<NonZeroU64>) -> ChainResult<Checkpoint> {
            todo!("Return the latest checkpoint")
        }
    
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
