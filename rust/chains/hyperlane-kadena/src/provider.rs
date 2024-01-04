use std::sync::Arc;

use async_trait::async_trait;

use hyperlane_core::{
    BlockInfo, ChainResult, HyperlaneChain, HyperlaneDomain, HyperlaneProvider, TxnInfo, H256,
};

use kadena_client::{apis::configuration::{Configuration as KadenaProxyConf, ConnectionConf}, contract::KadenaProxyProvider, signers::Signer};


/// A wrapper around a Kadena provider to get generic blockchain information.
#[derive(Debug)]
pub struct KadenaProvider {
    domain: HyperlaneDomain,
    connection_conf: Arc<ConnectionConf>,
    kadena_proxy_config: Arc<KadenaProxyConf>,
    signer: Arc<dyn Signer>,
}

impl KadenaProxyProvider for KadenaProvider {
    fn connection_conf(&self) -> Arc<ConnectionConf> {
        self.connection_conf.clone()
    }

    fn kadena_proxy_config(&self) -> Arc<KadenaProxyConf> {
        self.kadena_proxy_config.clone()
    }

    fn signer(&self) -> Arc<dyn Signer> {
        self.signer.clone()
    }
}

impl KadenaProvider {
    /// Create a new Kadena provider.
    pub fn new(
        domain: HyperlaneDomain,
        connection_conf: Arc<ConnectionConf>,
        kadena_proxy_config: Arc<KadenaProxyConf>,
        signer: Arc<dyn Signer>,
    ) -> Self {
        KadenaProvider { domain, connection_conf, kadena_proxy_config, signer }
    }

    /// Get the connection configuration.
    pub fn connection_conf(&self) -> Arc<ConnectionConf> {
        self.connection_conf.clone()
    }

    /// Get the Kadena proxy configuration.
    pub fn kadena_proxy_config(&self) -> Arc<KadenaProxyConf> {
        self.kadena_proxy_config.clone()
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
            connection_conf: self.connection_conf.clone(),
            kadena_proxy_config: self.kadena_proxy_config.clone(),
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
