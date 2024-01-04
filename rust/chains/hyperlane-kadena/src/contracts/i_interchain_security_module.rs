use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::{contract::{Contract, KadenaProxyProvider}, models::CommandDto, contract_call::ContractCall};
use anyhow::Result;

pub struct ModuleTypeCall<'a> {
    contract: &'a IInterchainSecurityModule,
    cmd: CommandDto,
}

impl ModuleTypeCall<'_> {
    const METHOD_NAME: &'static str = "module-type";
    pub async fn new(
        contract: &IInterchainSecurityModule,
    ) -> Result<ModuleTypeCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{})",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
            ),
        ).await?;

        Ok(ModuleTypeCall {
            contract,
            cmd,
        })
    }
}

impl ContractCall for ModuleTypeCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}

pub struct VerifyCall<'a> {
    contract: &'a IInterchainSecurityModule,
    metadata: String,
    message: String,
    cmd: CommandDto,
}

impl VerifyCall<'_> {
    const METHOD_NAME: &'static str = "verify";
    pub async fn new(
        contract: &IInterchainSecurityModule,
        metadata: String,
        message: String,
    ) -> Result<VerifyCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} \"{}\",\"{}\")",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
                metadata,
                message,
            ),
        ).await?;

        Ok(VerifyCall {
            contract,
            metadata,
            message,
            cmd,
        })
    }
}

impl ContractCall for VerifyCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}

#[derive(Clone, Debug)]
pub struct IInterchainSecurityModule {
    provider: Arc<KadenaProvider>,
}

impl IInterchainSecurityModule {
    const MODULE_NAME: &'static str = "isp";
    
    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }

    pub async fn module_type(&self) -> Result<ModuleTypeCall> {
        ModuleTypeCall::new(
            self,
        ).await
    }

    pub async fn verify(&self, metadata: String, message: String) -> Result<VerifyCall> {
        VerifyCall::new(
            self,
            metadata,
            message,
        ).await
    }
}

impl Contract for IInterchainSecurityModule {
    fn get_module_name(&self) ->  &'static str {
        Self::MODULE_NAME
    }
    
    fn provider(&self) ->  Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}