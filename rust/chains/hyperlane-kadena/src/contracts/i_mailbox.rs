use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::contract::{Contract, KadenaProxyPovider};

#[derive(Clone, Debug)]
pub(crate) struct IMailbox {
    provider: Arc<KadenaProvider>,
}

impl IMailbox {
    pub(crate) fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }
}

impl Contract for IMailbox {
    const MODULE_NAME: &'static str = "mailbox";

    fn provider(&self) ->  Arc<dyn KadenaProxyPovider + Send + Sync> {
        self.provider.clone()
    }
}
