use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyPovider};

const MODULE_NAME: &str = "IMultisigIsm";

#[derive(Clone, Debug)]
pub(crate) struct IMultisigIsm {
    module_name: &'static str,
    provider: Arc<KadenaProvider>,
}

impl IMultisigIsm {
    pub(crate) fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            module_name: &MODULE_NAME,
            provider,
        }
    }
}

impl Contract for IMultisigIsm {
    fn module_name(&self) -> &str {
        &self.module_name
    }
    fn provider(&self) ->  Arc<dyn KadenaProxyPovider + Send + Sync> {
        self.provider.clone()
    }
}