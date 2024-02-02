use std::sync::Arc;

use async_trait::async_trait;

use hyperlane_core::{
    BlockInfo, ChainResult, HyperlaneChain, HyperlaneDomain, HyperlaneProvider, TxnInfo, H256,
};

use kadena_client::{client::KadenaProxyClient, contract::KadenaProxyProvider, signers::Signer};

/// A wrapper around a Kadena provider to get generic blockchain information.
#[derive(Debug)]
pub struct KadenaProvider {
    domain: HyperlaneDomain,
    client: Arc<KadenaProxyClient>,
    signer: Arc<dyn Signer>,
}

impl KadenaProxyProvider for KadenaProvider {
    fn proxy_client(&self) -> Arc<KadenaProxyClient> {
        self.client.clone()
    }

    fn signer(&self) -> Arc<dyn Signer> {
        self.signer.clone()
    }
}

impl KadenaProvider {
    /// Create a new Kadena provider.
    pub fn new(
        domain: HyperlaneDomain,
        client: Arc<KadenaProxyClient>,
        signer: Arc<dyn Signer>,
    ) -> Self {
        KadenaProvider {
            domain,
            client,
            signer,
        }
    }

    /// Get the domain of the provider.
    pub fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }
}

impl HyperlaneChain for KadenaProvider {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }

    fn provider(&self) -> Box<dyn HyperlaneProvider> {
        Box::new(KadenaProvider {
            domain: self.domain.clone(),
            client: self.client.clone(),
            signer: self.signer.clone(),
        })
    }
}

#[async_trait]
impl HyperlaneProvider for KadenaProvider {
    async fn get_block_by_hash(&self, _hash: &H256) -> ChainResult<BlockInfo> {
        todo!() // FIXME: required by Scraper, it's out of the current scope
    }

    async fn get_txn_by_hash(&self, _hash: &H256) -> ChainResult<TxnInfo> {
        todo!() // FIXME: required by Scraper, it's out of the current scope
    }

    async fn is_contract(&self, _address: &H256) -> ChainResult<bool> {
        // FIXME: check whether the address is a module or not
        Ok(true)
    }
}
