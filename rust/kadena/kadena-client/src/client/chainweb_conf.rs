use url::Url;

/// Chainweb connection configuration.
#[derive(Debug, Clone)]
pub struct ChainwebConf {
    /// Fully qualified string to connect to.
    pub url: Url,
    /// Network ID.
    pub network_id: String,
    /// Chain ID.
    pub chain_id: u16,
}

impl ChainwebConf {
    /// Creates a new configuration with the given url, network id and chain id.
    pub fn new(url: Url, network_id: String, chain_id: u16) -> Self {
        ChainwebConf {
            url,
            network_id,
            chain_id,
        }
    }

    /// Returns the hostapi url for this configuration.
    pub(crate) fn hostapi(&self) -> String {
        format!(
            "{}chainweb/0.0/{}/chain/{}/pact",
            self.url, self.network_id, self.chain_id
        )
    }
}

impl Default for ChainwebConf {
    /// Creates a new configuration with default values (devnet).
    fn default() -> Self {
        ChainwebConf {
            url: Url::parse("https://127.0.0.1:8080").unwrap(),
            network_id: "development".to_string(),
            chain_id: 0,
        }
    }
}
