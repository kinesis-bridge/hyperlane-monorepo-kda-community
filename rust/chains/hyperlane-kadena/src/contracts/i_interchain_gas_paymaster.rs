use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyProvider};

#[derive(Clone, Debug)]
pub struct IInterchainGasPaymaster {
    provider: Arc<KadenaProvider>,
}

impl IInterchainGasPaymaster {
    const MODULE_NAME: &'static str = "igp";
    
    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }
}

impl Contract for IInterchainGasPaymaster {
    fn get_module_name(&self) ->  &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) ->  Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}