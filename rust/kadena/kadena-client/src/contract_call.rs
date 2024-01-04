use std::{collections::HashMap, sync::Arc};

use crate::{
    apis::configuration::{Configuration as ProxyConf, ConnectionConf},
    apis::{
        kadena_proxy_api::{self, LocalError, PollError, SendError},
        Error,
    },
    models::{
        CommandDto, CommandResultDto, LocalRequestBodyDto, PollRequestBodyDto, RequestKeysDto,
        SendRequestBodyDto,
    }, contract::Contract,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ContractCall: Send + Sync {
    fn get_contract(&self) -> &dyn Contract;

    fn get_conf(&self) -> Arc<ConnectionConf> {
        self.get_contract().provider().connection_conf().clone()
    }
    fn get_proxy_conf(&self) -> Arc<ProxyConf> {
        self.get_contract().provider().kadena_proxy_config().clone()
    }

    fn get_cmd(&self) -> &CommandDto;

    async fn send(&self) -> Result<RequestKeysDto, Error<SendError>> {
        let mut cmd = self.get_cmd().clone();
        // TODO: handle signing error
        let _ = self.get_contract().provider().signer().sign_transaction(&mut cmd).await;
        let send_body = SendRequestBodyDto::new(vec![cmd], self.get_conf().get_hostapi());
        kadena_proxy_api::send(&self.get_proxy_conf(), send_body).await
    }

    async fn local(&self) -> Result<CommandResultDto, Error<LocalError>> {
        let local_body =
            LocalRequestBodyDto::new(self.get_cmd().clone(), self.get_conf().get_hostapi(), true, false);
        kadena_proxy_api::local(&self.get_proxy_conf(), local_body).await
    }

    async fn poll(
        &self,
        request_keys: RequestKeysDto,
    ) -> Result<HashMap<std::string::String, CommandResultDto>, Error<PollError>> {
        let poll_body = PollRequestBodyDto::new(request_keys.request_keys, self.get_conf().get_hostapi());
        kadena_proxy_api::poll(&self.get_proxy_conf(), poll_body).await
    }

    async fn estimate_gas(&self) -> Result<u64> {
        let cmd = LocalRequestBodyDto::new(self.get_cmd().clone(), self.get_conf().get_hostapi(), true, false);
        let local_rsp = kadena_proxy_api::local(&self.get_proxy_conf(), cmd).await?;
        Ok(local_rsp.gas)
    }
}
