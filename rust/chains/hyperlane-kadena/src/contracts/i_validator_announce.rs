use std::sync::Arc;
use crate::{provider, KadenaProvider};
use kadena_client::{contract::{Contract, KadenaProxyProvider}, contract_call::ContractCall, models::CommandDto};
use anyhow::Result;

pub struct AnnounceCall<'a> {
    contract: &'a IValidatorAnnounce,
    validator: String,
    storage_location: String,
    signature: String,
    cmd: CommandDto,
}

impl AnnounceCall<'_> {
    const METHOD_NAME: &'static str = "announce";
    pub async fn new(
        contract: &IValidatorAnnounce,
        validator: String,
        storage_location: String,
        signature: String,
    ) -> Result<AnnounceCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} \"{}\",\"{}\",\"{}\")",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
                validator,
                storage_location,
                signature,
            ),
        ).await?;

        Ok(AnnounceCall {
            contract,
            validator,
            storage_location,
            signature,
            cmd,
        })
    }
}

impl ContractCall for AnnounceCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}

pub struct GetAnnouncedStorageLocationsCall<'a> {
    contract: &'a IValidatorAnnounce,
    validators: Vec<String>,
    cmd: CommandDto,
}

impl GetAnnouncedStorageLocationsCall<'_> {
    const METHOD_NAME: &'static str = "get-announced-storage-locations";
    pub async fn new(
        contract: &IValidatorAnnounce,
        validators: Vec<String>,
    ) -> Result<GetAnnouncedStorageLocationsCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} \"{}\")",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
                validators.join(","),
            ),
        ).await?;

        Ok(GetAnnouncedStorageLocationsCall {
            contract,
            validators,
            cmd,
        })
    }
}

impl ContractCall for GetAnnouncedStorageLocationsCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
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

    pub async fn announce(
        &self,
        validator: String,
        storage_location: String,
        signature: String,
    ) -> Result<AnnounceCall> {
        AnnounceCall::new(
            self,
            validator,
            storage_location,
            signature,
        ).await
    }

    pub async fn get_announced_storage_locations(
        &self,
        validators: Vec<String>,
    ) -> Result<GetAnnouncedStorageLocationsCall> {
        GetAnnouncedStorageLocationsCall::new(
            self,
            validators,
        ).await
    }
}

impl Contract for IValidatorAnnounce {
    fn get_module_name(&self) ->  &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) ->  Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}