use async_trait::async_trait;

use hyperlane_core::{
    BlockInfo, ChainResult, HyperlaneChain, HyperlaneDomain, HyperlaneProvider, TxnInfo, H256,
};

/// A wrapper around a Kadena provider to get generic blockchain information.
#[derive(Debug)]
pub struct KadenaProvider {
    domain: HyperlaneDomain,
}

impl KadenaProvider {
    /// Create a new Kadena provider.
    pub fn new(domain: HyperlaneDomain) -> Self {
        KadenaProvider { domain }
    }
}

impl HyperlaneChain for KadenaProvider {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }

    fn provider(&self) -> Box<dyn HyperlaneProvider> {
        Box::new(KadenaProvider {
            domain: self.domain.clone(),
        })
    }
}

#[async_trait]
impl HyperlaneProvider for KadenaProvider {
    async fn get_block_by_hash(&self, _hash: &H256) -> ChainResult<BlockInfo> {
        todo!() // FIXME
    }

    async fn get_txn_by_hash(&self, _hash: &H256) -> ChainResult<TxnInfo> {
        todo!() // FIXME
    }

    async fn is_contract(&self, _address: &H256) -> ChainResult<bool> {
        // FIXME
        Ok(true)
    }
}
