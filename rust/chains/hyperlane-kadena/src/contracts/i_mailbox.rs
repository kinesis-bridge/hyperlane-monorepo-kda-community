use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyProvider};

#[derive(Clone, Debug)]
pub struct IMailbox {
    provider: Arc<KadenaProvider>,
}

impl IMailbox {
    const MODULE_NAME: &'static str = "mailbox";

    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }
}

impl Contract for IMailbox {
    fn get_module_name(&self) ->  &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) ->  Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}
