use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyPovider};

const MODULE_NAME: &str = "IValidatorAnnounce";

#[derive(Clone, Debug)]
pub(crate) struct IValidatorAnnounce {
    module_name: &'static str,
    provider: Arc<KadenaProvider>,
}

impl IValidatorAnnounce {
    pub(crate) fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            module_name: &MODULE_NAME,
            provider,
        }
    }
}

impl Contract for IValidatorAnnounce {
    fn module_name(&self) -> &str {
        &self.module_name
    }
    fn provider(&self) ->  Arc<dyn KadenaProxyPovider + Send + Sync> {
        self.provider.clone()
    }
}