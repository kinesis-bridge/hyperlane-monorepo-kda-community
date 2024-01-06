use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::{contract::{Contract, KadenaProxyProvider}, models::CommandDto, contract_call::ContractCall};
use anyhow::Result;

pub struct CountCall<'a> {
    contract: &'a IMerlkeTreeHook,
    cmd: CommandDto,
}

impl CountCall<'_> {
    const METHOD_NAME: &'static str = "count";
    pub async fn new(
        contract: &IMerlkeTreeHook,
    ) -> Result<CountCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{})",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
            ),
        ).await?;

        Ok(CountCall {
            contract,
            cmd,
        })
    }
}

impl ContractCall for CountCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}

pub struct LatestCheckpointCall<'a> {
    contract: &'a IMerlkeTreeHook,
    cmd: CommandDto,
}

impl LatestCheckpointCall<'_> {
    const METHOD_NAME: &'static str = "latest-checkpoint";
    pub async fn new(
        contract: &IMerlkeTreeHook,
    ) -> Result<LatestCheckpointCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{})",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
            ),
        ).await?;

        Ok(LatestCheckpointCall {
            contract,
            cmd,
        })
    }
}

impl ContractCall for LatestCheckpointCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}

pub struct TreeCall<'a> {
    contract: &'a IMerlkeTreeHook,
    cmd: CommandDto,
}

impl TreeCall<'_> {
    const METHOD_NAME: &'static str = "tree";
    pub async fn new(
        contract: &IMerlkeTreeHook,
    ) -> Result<TreeCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{})",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
            ),
        ).await?;

        Ok(TreeCall {
            contract,
            cmd,
        })
    }
}

impl ContractCall for TreeCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}

#[derive(Clone, Debug)]
pub struct IMerlkeTreeHook {
    provider: Arc<KadenaProvider>,
}

impl IMerlkeTreeHook {
    // refers to the mailbox module unless the merkle tree hook is not implemented on the pact side
    const MODULE_NAME: &'static str = "mailbox";
    
    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }

    pub async fn count(&self) -> Result<CountCall> {
        CountCall::new(
            self,
        ).await
    }

    pub async fn latest_checkpoint(&self) -> Result<LatestCheckpointCall> {
        LatestCheckpointCall::new(
            self,
        ).await
    }

    pub async fn tree(&self) -> Result<TreeCall> {
        TreeCall::new(
            self,
        ).await
    }
}

impl Contract for IMerlkeTreeHook {
    fn get_module_name(&self) ->  &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) ->  Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}