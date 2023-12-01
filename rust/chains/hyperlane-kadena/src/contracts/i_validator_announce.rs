use std::sync::Arc;
use crate::{provider, KadenaProvider};
use kadena_client::{contract::{Contract, KadenaProxyPovider}, contract_call::ContractCall, models::CommandDto};
use anyhow::Result;

#[derive(Clone, Debug)]
pub(crate) struct IValidatorAnnounce {
    provider: Arc<KadenaProvider>,
}

impl IValidatorAnnounce {
    pub(crate) fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }

    pub fn announce(
        &self,
        validator: ethers::core::types::Address,
        storage_location: String,
        signature: ethers::core::types::Bytes,
    ) -> Result<ContractCall> {
        let cmd = CommandDto::new();
        let call = ContractCall::new(
            self.provider().connection_conf(),
            self.provider().kadena_proxy_config(),
            cmd,
        );
    }
}

impl Contract for IValidatorAnnounce {
    const MODULE_NAME: &str = "validator-announce";

    fn provider(&self) ->  Arc<dyn KadenaProxyPovider + Send + Sync> {
        self.provider.clone()
    }
}