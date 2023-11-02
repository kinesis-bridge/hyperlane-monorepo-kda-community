//use std::net;

use hyperlane_core::config::{ConfigPath, ConfigResult, FromRawConf, ConfigErrResultExt};
use kadena_client::apis::configuration::ConnectionConf;
use url::Url;
use std::num::ParseIntError;

/// Raw Kadena connection configuration used for better deserialization errors.
#[derive(Debug, serde::Deserialize)]
pub struct DeprecatedRawConnectionConf {
    #[allow(dead_code)]
    url: Option<String>,
    #[allow(dead_code)]
    network_id: Option<String>,
    #[allow(dead_code)]
    chain_id: Option<String>,
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

impl FromRawConf<DeprecatedRawConnectionConf> for ConnectionConf {
    fn from_config_filtered(
        raw: DeprecatedRawConnectionConf,
        cwp: &ConfigPath,
        _filter: (),
    ) -> ConfigResult<Self> {
        use ConnectionConfError::*;

        let url: Url = raw
            .url
            .as_ref()
            .ok_or(MissingConnectionUrl)
            .into_config_result(|| cwp.join("url"))?
            .parse()
            .map_err(|e| InvalidConnectionUrl(raw.url.unwrap().clone(), e))
            .into_config_result(|| cwp.join("url"))?;

        let network_id = raw
            .network_id
            .ok_or(MissingNetworkId)
            .into_config_result(|| cwp.join("network_id"))?;
            
        let chain_id: u8 = raw
            .chain_id
            .as_ref()
            .ok_or(MissingChainId)
            .into_config_result(|| cwp.join("chain_id"))?
            .parse()
            .map_err(|e| InvalidChainId(raw.chain_id.unwrap().clone(), e))
            .into_config_result(|| cwp.join("chain_id"))?;
     
        Ok(Self {
            url,
            network_id,
            chain_id,
        })
    }
}
