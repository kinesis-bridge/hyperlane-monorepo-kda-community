use std::sync::Arc;
use crate::{provider, KadenaProvider};
use kadena_client::{contract::{Contract, KadenaProxyProvider}, contract_call::ContractCall, models::CommandDto};
use anyhow::Result;
use async_trait::async_trait;

pub struct AnnounceCall<'a> {
    contract: &'a IValidatorAnnounce,
    validator: [u8; 20],
    storage_location: String,
    signature: [u8; 65],
    gas_limit: Option<u64>,
}

impl AnnounceCall<'_> {
    const METHOD_NAME: &'static str = "announce";
    pub fn new(
        contract: &IValidatorAnnounce,
        validator: [u8; 20],
        storage_location: String,
        signature: [u8; 65],
    ) -> AnnounceCall {
        AnnounceCall {
            contract,
            validator,
            storage_location,
            signature,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for AnnounceCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto> {
        self.contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} \"{}\",\"{}\",\"{}\")",
                self.contract.namespace(),
                self.contract.module_name(),
                Self::METHOD_NAME,
                hex::encode(self.validator),
                self.storage_location,
                hex::encode(self.signature),
            ),
            self.gas_limit,
        ).await.map_err(|e| e.into())
    }
}

pub struct GetAnnouncedStorageLocationsCall<'a> {
    contract: &'a IValidatorAnnounce,
    validators: Vec<[u8; 20]>,
    gas_limit: Option<u64>,
}

impl GetAnnouncedStorageLocationsCall<'_> {
    const METHOD_NAME: &'static str = "get-announced-storage-locations";
    pub fn new(
        contract: &IValidatorAnnounce,
        validators: Vec<[u8; 20]>,
    ) -> GetAnnouncedStorageLocationsCall {
        GetAnnouncedStorageLocationsCall {
            contract,
            validators,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for GetAnnouncedStorageLocationsCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto> {
        self.contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} [{}])",
                self.contract.namespace(),
                self.contract.module_name(),
                Self::METHOD_NAME,
                self.validators.iter().map(|v| format!("\"{}\"", hex::encode(v))).collect::<Vec<String>>().join(","),
            ),
            self.gas_limit,
        ).await.map_err(|e| e.into())
    }
}


#[derive(Clone, Debug)]
pub struct IValidatorAnnounce {
    provider: Arc<KadenaProvider>,
}

impl IValidatorAnnounce {
    const MODULE_NAME: &'static str = "validator-announce";

    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }

    pub fn announce(
        &self,
        validator: [u8; 20],
        storage_location: String,
        signature: [u8; 65],
    ) -> AnnounceCall {
        AnnounceCall::new(
            self,
            validator,
            storage_location,
            signature,
        )
    }

    pub fn get_announced_storage_locations(
        &self,
        validators: Vec<[u8; 20]>,
    ) -> GetAnnouncedStorageLocationsCall {
        GetAnnouncedStorageLocationsCall::new(
            self,
            validators,
        )
    }
}

impl Contract for IValidatorAnnounce {
    fn module_name(&self) ->  &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) ->  Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}