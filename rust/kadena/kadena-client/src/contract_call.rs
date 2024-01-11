use std::{collections::HashMap, sync::Arc};

use crate::{
    apis::configuration::{Configuration as ProxyConf, ConnectionConf},
    apis::{
        kadena_proxy_api::{self, PollError},
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
    fn contract(&self) -> &dyn Contract;

    fn conf(&self) -> Arc<ConnectionConf> {
        self.contract().provider().connection_conf().clone()
    }
    fn proxy_conf(&self) -> Arc<ProxyConf> {
        self.contract().provider().kadena_proxy_config().clone()
    }

    // TODO: use contract call context in case of multiple fields required by contract call
    fn set_gas_limit(&mut self, gas_limit: u64);
    fn gas_limit(&self) -> Option<u64>;

    async fn cmd(&self) -> Result<CommandDto>;

    async fn send(&self) -> Result<RequestKeysDto> {
        let mut cmd = self.cmd().await?;
        let _ = self.contract().provider().signer().sign_transaction(&mut cmd).await?;
        let send_body = SendRequestBodyDto::new(vec![cmd], self.conf().hostapi());
        kadena_proxy_api::send(&self.proxy_conf(), send_body).await.map_err(|e| e.into())
    }

    async fn local(&self) -> Result<CommandResultDto> {
        let local_body =
            LocalRequestBodyDto::new(self.cmd().await?, self.conf().hostapi(), true, false);
        kadena_proxy_api::local(&self.proxy_conf(), local_body).await.map_err(|e| e.into())
    }

    async fn poll(
        &self,
        request_keys: RequestKeysDto,
    ) -> Result<HashMap<std::string::String, CommandResultDto>, Error<PollError>> {
        let poll_body = PollRequestBodyDto::new(request_keys.request_keys, self.conf().hostapi());
        kadena_proxy_api::poll(&self.proxy_conf(), poll_body).await
    }

    async fn estimate_gas(&self) -> Result<u64> {
        let cmd = LocalRequestBodyDto::new(self.cmd().await?, self.conf().hostapi(), true, false);
        let local_rsp = kadena_proxy_api::local(&self.proxy_conf(), cmd).await?;
        Ok(local_rsp.gas)
    }
}
