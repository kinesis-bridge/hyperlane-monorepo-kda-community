use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyPovider};

const MODULE_NAME: &str = "IInterchainSecurityModule";

#[derive(Clone, Debug)]
pub(crate) struct IInterchainSecurityModule {
    module_name: &'static str,
    provider: Arc<KadenaProvider>,
}

impl IInterchainSecurityModule {
    pub(crate) fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            module_name: &MODULE_NAME,
            provider,
        }
    }
}

impl Contract for IInterchainSecurityModule {
    fn module_name(&self) -> &str {
        &self.module_name
    }
    fn provider(&self) ->  Arc<dyn KadenaProxyPovider + Send + Sync> {
        self.provider.clone()
    }
}