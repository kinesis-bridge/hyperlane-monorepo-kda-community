use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyPovider};

const MODULE_NAME: &str = "IMailbox";

#[derive(Clone, Debug)]
pub(crate) struct IMailbox {
    module_name: &'static str,
    provider: Arc<KadenaProvider>,
}

impl IMailbox {
    pub(crate) fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            module_name: &MODULE_NAME,
            provider,
        }
    }
}

impl Contract for IMailbox {
    fn module_name(&self) -> &str {
        &self.module_name
    }
    fn provider(&self) ->  Arc<dyn KadenaProxyPovider + Send + Sync> {
        self.provider.clone()
    }
}
