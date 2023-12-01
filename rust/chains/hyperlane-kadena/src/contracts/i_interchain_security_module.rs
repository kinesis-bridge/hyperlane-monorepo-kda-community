use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyPovider};

#[derive(Clone, Debug)]
pub(crate) struct IInterchainSecurityModule {
    provider: Arc<KadenaProvider>,
}

impl IInterchainSecurityModule {
    pub(crate) fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }
}

impl Contract for IInterchainSecurityModule {
    const MODULE_NAME: &str = "isp";
    
    fn provider(&self) ->  Arc<dyn KadenaProxyPovider + Send + Sync> {
        self.provider.clone()
    }
}