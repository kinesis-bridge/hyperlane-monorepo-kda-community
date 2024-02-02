use url::Url;

/// Configuration for the proxy client.
#[derive(Debug, Clone)]
pub struct ProxyConf {
    pub url: Url,
    pub user_agent: Option<String>,
}

impl ProxyConf {
    /// Creates a new configuration with the given url and user agent.
    pub fn new(url: Url, user_agent: Option<String>) -> ProxyConf {
        ProxyConf { url, user_agent }
    }

    /// Creates a new configuration with the given url. Initialize other fields with default values.
    pub fn new_with_url(url: Url) -> ProxyConf {
        ProxyConf {
            url,
            ..Default::default()
        }
    }
}

impl Default for ProxyConf {
    /// Creates a new configuration with default values.
    fn default() -> Self {
        ProxyConf {
            url: Url::parse("http://localhost:3000").unwrap(),
            user_agent: Some("kadena-proxy-client/0.1.0/rust".to_owned()),
        }
    }
}
