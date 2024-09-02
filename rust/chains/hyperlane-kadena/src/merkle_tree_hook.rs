#![allow(missing_docs)]
use std::num::NonZeroU64;
use std::ops::RangeInclusive;
use std::sync::Arc;

use async_trait::async_trait;
use hyperlane_core::accumulator::incremental::IncrementalMerkle;
use kadena_client::contract_call::ContractCall;
use kadena_client::signers::Signer;
use tracing::instrument;

use hyperlane_core::{
    ChainCommunicationError, ChainResult, Checkpoint, HyperlaneChain, HyperlaneContract,
    HyperlaneDomain, HyperlaneProvider, Indexer, LogMeta, MerkleTreeHook, MerkleTreeInsertion,
    SequenceIndexer, H256,
};

use crate::contracts::i_mailbox::IMailbox;
use crate::contracts::i_merkle_tree_hook::IMerlkeTreeHook;
use crate::{ConnectionConf, KadenaProvider};
use kadena_client::contract::{Contract, KadenaProxyProvider};

#[derive(Debug)]
/// Struct that retrieves event data for a Kadena MerkleTreeHook
pub struct KadenaMerkleTreeHookIndexer {
    // The Kadena implementation doesn't have the merkleTreeHook contract yet, so we use the mailbox contract
    #[allow(dead_code)]
    contract: Arc<IMailbox>,
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
            contract: Arc::new(IMailbox::new(provider.clone())),
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
        Ok((self
            .provider
            .get_block_number()
            .await
            .map_err(ChainCommunicationError::from_other)? as u32)
            .saturating_sub(self.reorg_period))
    }
}

#[async_trait]
impl SequenceIndexer<MerkleTreeInsertion> for KadenaMerkleTreeHookIndexer {
    async fn sequence_and_tip(&self) -> ChainResult<(Option<u32>, u32)> {
        let tip = Indexer::<MerkleTreeInsertion>::get_finalized_block_number(self).await?;

        // TODO: block call is not supported yet
        //let sequence = self.contract.nonce().block(u64::from(tip)).call().await?;
        let sequence = self
            .contract
            .nonce()
            .local_typed()
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok((Some(sequence), tip))
    }
}

/// A reference to a merkleTreeHook contract on some Kadena chain
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
        let latest_checkpoint = self
            .contract
            .latest_checkpoint()
            .local_typed()
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok(latest_checkpoint)
    }

    #[instrument(skip(self))]
    #[allow(clippy::needless_range_loop)]
    async fn tree(&self, _maybe_lag: Option<NonZeroU64>) -> ChainResult<IncrementalMerkle> {
        let tree = self
            .contract
            .tree()
            .local_typed()
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok(tree)
    }

    #[instrument(skip(self))]
    async fn count(&self, _maybe_lag: Option<NonZeroU64>) -> ChainResult<u32> {
        let count = self
            .contract
            .count()
            .local_typed()
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok(count)
    }
}
