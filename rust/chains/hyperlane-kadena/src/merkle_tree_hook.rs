#![allow(missing_docs)]
use std::num::NonZeroU64;
use std::ops::RangeInclusive;
use std::sync::Arc;

use async_trait::async_trait;
use hyperlane_core::accumulator::incremental::IncrementalMerkle;
use kadena_client::signers::Signer;
use tracing::instrument;

use hyperlane_core::{
    ChainResult, Checkpoint, HyperlaneChain, HyperlaneContract, HyperlaneDomain, HyperlaneProvider,
    Indexer, LogMeta, MerkleTreeHook, MerkleTreeInsertion, SequenceIndexer, H256,
};

use crate::contracts::i_merkle_tree_hook::IMerlkeTreeHook;
use crate::{ConnectionConf, KadenaProvider};
use kadena_client::contract::Contract;

#[derive(Debug)]
/// Struct that retrieves event data for a Kadena MerkleTreeHook
pub struct KadenaMerkleTreeHookIndexer {
    #[allow(dead_code)]
    contract: Arc<IMerlkeTreeHook>,
    #[allow(dead_code)]
    provider: Arc<KadenaProvider>,
    #[allow(dead_code)]
    reorg_period: u32,
}

impl KadenaMerkleTreeHookIndexer {
    /// Create new KadenaMerkleTreeHookIndexer
    #[allow(unused)]
    pub fn new(
        conf: &ConnectionConf,
        domain: &HyperlaneDomain,
        signer: Arc<dyn Signer>,
        reorg_period: u32,
    ) -> Self {
        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(conf.into()),
            signer.clone(),
        ));

        Self {
            contract: Arc::new(IMerlkeTreeHook::new(provider.clone())),
            provider,
            reorg_period,
        }
    }
}

#[async_trait]
impl Indexer<MerkleTreeInsertion> for KadenaMerkleTreeHookIndexer {
    #[instrument(err, skip(self))]
    async fn fetch_logs(
        &self,
        _range: RangeInclusive<u32>,
    ) -> ChainResult<Vec<(MerkleTreeInsertion, LogMeta)>> {
        Ok(vec![])
    }

    #[instrument(level = "debug", err, ret, skip(self))]
    async fn get_finalized_block_number(&self) -> ChainResult<u32> {
        Ok(0)
    }
}

#[async_trait]
impl SequenceIndexer<MerkleTreeInsertion> for KadenaMerkleTreeHookIndexer {
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

/// A reference to a Mailbox contract on some Kadena chain
#[derive(Debug)]
pub struct KadenaMerkleTreeHook {
    contract: Arc<IMerlkeTreeHook>,
    domain: HyperlaneDomain,
    #[allow(dead_code)]
    provider: Arc<KadenaProvider>,
}

impl KadenaMerkleTreeHook {
    /// Create a reference to a mailbox at a specific Kadena address on some
    /// chain

    #[allow(unused)]
    pub fn new(conf: &ConnectionConf, domain: &HyperlaneDomain, signer: Arc<dyn Signer>) -> Self {
        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(conf.into()),
            signer.clone(),
        ));

        Self {
            contract: Arc::new(IMerlkeTreeHook::new(provider.clone())),
            domain: domain.clone(),
            provider,
        }
    }
}

impl HyperlaneChain for KadenaMerkleTreeHook {
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

impl HyperlaneContract for KadenaMerkleTreeHook {
    fn address(&self) -> H256 {
        self.contract.address().into()
    }
}

#[async_trait]
impl MerkleTreeHook for KadenaMerkleTreeHook {
    #[instrument(skip(self))]
    async fn latest_checkpoint(&self, _maybe_lag: Option<NonZeroU64>) -> ChainResult<Checkpoint> {
        unimplemented!()
        /*
        let call =
            call_with_lag(self.contract.latest_checkpoint(), &self.provider, maybe_lag).await?;

        let (root, index) = call.call().await?;
        Ok(Checkpoint {
            merkle_tree_hook_address: self.address(),
            mailbox_domain: self.domain.id(),
            root: root.into(),
            index,
        })
        */
    }

    #[instrument(skip(self))]
    #[allow(clippy::needless_range_loop)]
    async fn tree(&self, _maybe_lag: Option<NonZeroU64>) -> ChainResult<IncrementalMerkle> {
        unimplemented!()
        /*
        let call = call_with_lag(self.contract.tree(), &self.provider, maybe_lag).await?;

        Ok(call.call().await?.into())

        */
    }

    #[instrument(skip(self))]
    async fn count(&self, _maybe_lag: Option<NonZeroU64>) -> ChainResult<u32> {
        unimplemented!()
        /*
        let call = call_with_lag(self.contract.count(), &self.provider, maybe_lag).await?;
        let count = call.call().await?;
        Ok(count)
        */
    }
}
