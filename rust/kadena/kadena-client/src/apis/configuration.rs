use url::Url;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }

    pub fn new_with_base_path(base_path: String) -> Result<Configuration> {
        Url::parse(&base_path)?; // Ensure base_path is a valid URL
        Ok(Configuration {
            base_path,
            ..Configuration::default()
        })
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost:3000".to_owned(),
            user_agent: Some("kadena-proxy-client/0.1.0/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}

/// Kadena connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConf {
    /// Fully qualified string to connect to
    pub url: Url,
    /// Network ID
    pub network_id: String,
    /// Chain ID
    pub chain_id: u8,
}

impl ConnectionConf {
    pub fn new(url: Url, network_id: String, chain_id: u8) -> Self {
        ConnectionConf {
            url,
            network_id,
            chain_id,
        }
    }

    pub fn get_hostapi(&self) -> String {
        format!("{}chainweb/0.0/{}/chain/{}/pact", self.url, self.network_id, self.chain_id)
    }    
}

impl Default for ConnectionConf {
    fn default() -> Self {
        ConnectionConf {
            url: Url::parse("https://api.testnet.chainweb.com").unwrap(),
            network_id: "testnet04".to_string(),
            chain_id: 1,
        }
    }
}
