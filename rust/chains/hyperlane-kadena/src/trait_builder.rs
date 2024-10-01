use hyperlane_core::ChainCommunicationError;
use kadena_client::client::{ChainwebConf, ContractsConf, KadenaProxyClient, ProxyConf};
use std::num::ParseIntError;
use url::Url;

/// Kadena connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConf {
    /// Fully qualified string to connect to
    pub url: Url,

    /// Network ID
    #[allow(dead_code)]
    pub network_id: String,

    /// Chain ID
    #[allow(dead_code)]
    pub chain_id: u16,

    /// Chain ID
    #[allow(dead_code)]
    pub kadena_proxy_url: Url,

    /// Namespace
    #[allow(dead_code)]
    pub kadena_namespace: String,

    /// Account name
    #[allow(dead_code)]
    pub account_name: Option<String>,
}

/// An error type when parsing a connection configuration.
#[derive(thiserror::Error, Debug)]
pub enum ConnectionConfError {
    /// Missing `url` for connection configuration
    #[error("Missing `url` for connection configuration")]
    MissingConnectionUrl,
    /// Invalid `url` for connection configuration
    #[error("Invalid `url` for connection configuration: `{0}` ({1})")]
    InvalidConnectionUrl(String, url::ParseError),
    /// Missing `network_id` for connection configuration
    #[error("Missing `network_id` for connection configuration")]
    MissingNetworkId,
    /// Missing `chain_id` for connection configuration
    #[error("Missing `chain_id` for connection configuration")]
    MissingChainId,
    /// Invalid `chain_id` for connection configuration
    #[error("Invalid `chain_id` for connection configuration: `{0}` ({1})")]
    InvalidChainId(String, ParseIntError),
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
struct KadenaNewConnectionError(#[from] anyhow::Error);

impl From<KadenaNewConnectionError> for ChainCommunicationError {
    fn from(err: KadenaNewConnectionError) -> Self {
        ChainCommunicationError::from_other(err)
    }
}

impl Into<KadenaProxyClient> for &ConnectionConf {
    fn into(self) -> KadenaProxyClient {
        KadenaProxyClient::new(
            ProxyConf::new_with_url(self.kadena_proxy_url.clone()),
            ChainwebConf::new(self.url.clone(), self.network_id.clone(), self.chain_id),
            ContractsConf::new(self.kadena_namespace.clone(), self.account_name.clone()),
        )
    }
}
