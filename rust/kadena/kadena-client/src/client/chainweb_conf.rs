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
        let base = self.url.to_string();

        format!(
            "{}/chainweb/0.0/{}/chain/{}/pact",
            base.trim_end_matches('/'),
            self.network_id,
            self.chain_id
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostapi() {
        let conf = ChainwebConf::default();
        assert_eq!(
            conf.hostapi(),
            "https://127.0.0.1:8080/chainweb/0.0/development/chain/0/pact"
        );

        let conf = ChainwebConf::new(
            Url::parse("https://api.tatum.io/v3/blockchain/node/kadena-mainnet/t-6749e3a82f9eb24437e59245-5eb4be6f9845425cb1a88875").unwrap(),
            "mainnet01".to_string(),
            2,
        );
        assert_eq!(
            conf.hostapi(),
            "https://api.tatum.io/v3/blockchain/node/kadena-mainnet/t-6749e3a82f9eb24437e59245-5eb4be6f9845425cb1a88875/chainweb/0.0/mainnet01/chain/2/pact"
        );
    }
}
