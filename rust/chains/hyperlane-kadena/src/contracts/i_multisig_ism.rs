use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyPovider};

#[derive(Clone, Debug)]
pub(crate) struct IMultisigIsm {
    provider: Arc<KadenaProvider>,
}

impl IMultisigIsm {
    pub(crate) fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }
}

impl Contract for IMultisigIsm {
    const MODULE_NAME: &str = "multisig-ism";

    fn provider(&self) ->  Arc<dyn KadenaProxyPovider + Send + Sync> {
        self.provider.clone()
    }
}