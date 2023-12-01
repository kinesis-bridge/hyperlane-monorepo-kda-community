use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyPovider};

#[derive(Clone, Debug)]
pub(crate) struct IInterchainGasPaymaster {
    provider: Arc<KadenaProvider>,
}

impl IInterchainGasPaymaster {
    pub(crate) fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }
}

impl Contract for IInterchainGasPaymaster {
    const MODULE_NAME: &str = "igp";

    fn provider(&self) ->  Arc<dyn KadenaProxyPovider + Send + Sync> {
        self.provider.clone()
    }
}