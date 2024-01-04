use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::{contract::{Contract, KadenaProxyProvider}, models::CommandDto, contract_call::ContractCall};
use anyhow::Result;

pub struct DeliveredCall<'a> {
    contract: &'a IMailbox,
    message_id: String,
    cmd: CommandDto,
}

impl DeliveredCall<'_> {
    const METHOD_NAME: &'static str = "delivered";
    pub async fn new(
        contract: &IMailbox,
        message_id: String,
    ) -> Result<DeliveredCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} \"{}\")",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
                message_id,
            ),
        ).await?;

        Ok(DeliveredCall {
            contract,
            message_id,
            cmd,
        })
    }
}

impl ContractCall for DeliveredCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}


pub struct NonceCall<'a> {
    contract: &'a IMailbox,
    cmd: CommandDto,
}

impl NonceCall<'_> {
    const METHOD_NAME: &'static str = "nonce";
    pub async fn new(
        contract: &IMailbox,
    ) -> Result<NonceCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{})",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
            ),
        ).await?;

        Ok(NonceCall {
            contract,
            cmd,
        })
    }
}

impl ContractCall for NonceCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}


pub struct ProcessCall<'a> {
    contract: &'a IMailbox,
    metadata: String,
    message: String,
    cmd: CommandDto,
}

impl ProcessCall<'_> {
    const METHOD_NAME: &'static str = "process";
    pub async fn new(
        contract: &IMailbox,
        metadata: String,
        message: String,
    ) -> Result<ProcessCall> {
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

        Ok(ProcessCall {
            contract,
            metadata,
            message,
            cmd,
        })
    }
}

impl ContractCall for ProcessCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}


pub struct RecipientIsmCall<'a> {
    contract: &'a IMailbox,
    recipient: String,
    cmd: CommandDto,
}

impl RecipientIsmCall<'_> {
    const METHOD_NAME: &'static str = "recipient-ism";
    pub async fn new(
        contract: &IMailbox,
        recipient: String,
    ) -> Result<RecipientIsmCall> {
        let cmd = contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} \"{}\")",
                contract.get_namespace(),
                contract.get_module_name(),
                Self::METHOD_NAME,
                recipient,
            ),
        ).await?;

        Ok(RecipientIsmCall {
            contract,
            recipient,
            cmd,
        })
    }
}

impl ContractCall for RecipientIsmCall<'_> {
    fn get_contract(&self) -> &dyn Contract {
        self.contract
    }

    fn get_cmd(&self) -> &CommandDto {
        &self.cmd
    }
}

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

    pub async fn delivered(&self, message_id: String) -> Result<DeliveredCall> {
        DeliveredCall::new(
            self,
            message_id,
        ).await
    }

    pub async fn nonce(&self) -> Result<NonceCall> {
        NonceCall::new(
            self,
        ).await
    }

    pub async fn process(&self, metadata: String, message: String) -> Result<ProcessCall> {
        ProcessCall::new(
            self,
            metadata,
            message,
        ).await
    }

    pub async fn recipient_ism(&self, recipient: String) -> Result<RecipientIsmCall> {
        RecipientIsmCall::new(
            self,
            recipient,
        ).await
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
