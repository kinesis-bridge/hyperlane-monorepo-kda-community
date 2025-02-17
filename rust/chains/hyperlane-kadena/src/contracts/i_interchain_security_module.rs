use std::sync::Arc;

use crate::{provider, KadenaProvider};
use async_trait::async_trait;
use kadena_client::{
    contract::{Contract, KadenaProxyProvider},
    contract_call::ContractCall,
    error::KadenaClientError,
    models::CommandDto,
    pact::PactValue,
};

use serde::Deserialize;

pub struct ModuleTypeCall<'a> {
    contract: &'a IInterchainSecurityModule,
    gas_limit: Option<u64>,
}

impl ModuleTypeCall<'_> {
    const METHOD_NAME: &'static str = "module-type";
    pub fn new(contract: &IInterchainSecurityModule) -> ModuleTypeCall {
        ModuleTypeCall {
            contract,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for ModuleTypeCall<'_> {
    type Output = u8;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{})",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn local_typed_impl(
        &self,
        rewind_depth: Option<u64>,
    ) -> Result<Self::Output, KadenaClientError> {
        let res_value = &PactValue::deserialize(self.local(rewind_depth).await?.result()?)?;
        res_value.try_into()
    }
}

pub struct VerifyCall<'a> {
    contract: &'a IInterchainSecurityModule,
    metadata: String,
    message: String,
    gas_limit: Option<u64>,
}

impl VerifyCall<'_> {
    const METHOD_NAME: &'static str = "verify";
    pub fn new(
        contract: &IInterchainSecurityModule,
        metadata: String,
        message: String,
    ) -> VerifyCall {
        VerifyCall {
            contract,
            metadata,
            message,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for VerifyCall<'_> {
    type Output = bool;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{} \"{}\",\"{}\")",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                    self.metadata,
                    self.message,
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn local_typed_impl(
        &self,
        rewind_depth: Option<u64>,
    ) -> Result<Self::Output, KadenaClientError> {
        Ok(self
            .contract
            .verify(self.metadata.clone(), self.message.clone())
            .local(rewind_depth)
            .await?
            .result()?
            .as_bool()
            .ok_or(KadenaClientError::TypeConversionError(
                "Verify result is not a bool".to_string(),
            ))?)
    }
}

#[derive(Clone, Debug)]
pub struct IInterchainSecurityModule {
    provider: Arc<KadenaProvider>,
}

impl IInterchainSecurityModule {
    const MODULE_NAME: &'static str = "merkle-tree-ism";

    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self { provider }
    }

    pub fn module_type(&self) -> ModuleTypeCall {
        ModuleTypeCall::new(self)
    }

    pub fn verify(&self, metadata: String, message: String) -> VerifyCall {
        VerifyCall::new(self, metadata, message)
    }
}

impl Contract for IInterchainSecurityModule {
    fn module_name(&self) -> &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}
