use std::collections::HashMap;

use crate::{
    apis::configuration::{Configuration as ProxyConf, ConnectionConf},
    apis::{
        kadena_proxy_api::{self, LocalError, PollError, SendError},
        Error,
    },
    models::{
        CommandDto, CommandResultDto, LocalRequestBodyDto, PollRequestBodyDto, RequestKeysDto,
        SendRequestBodyDto,
    },
};
use anyhow::Result;

pub struct ContractCall {
    conf: ConnectionConf,
    proxy_conf: ProxyConf,
    cmd: CommandDto,
}

impl ContractCall {
    pub fn new(conf: ConnectionConf, proxy_conf: ProxyConf, cmd: CommandDto) -> Self {
        Self {
            conf,
            proxy_conf,
            cmd,
        }
    }

    pub async fn send(&self) -> Result<RequestKeysDto, Error<SendError>> {
        let send_body = SendRequestBodyDto::new(vec![self.cmd.clone()], self.conf.get_hostapi());
        kadena_proxy_api::send(&self.proxy_conf, send_body).await
    }

    pub async fn local(&self) -> Result<CommandResultDto, Error<LocalError>> {
        let local_body =
            LocalRequestBodyDto::new(self.cmd.clone(), self.conf.get_hostapi(), true, false);
        kadena_proxy_api::local(&self.proxy_conf, local_body).await
    }

    pub async fn poll(
        &self,
        request_keys: RequestKeysDto,
    ) -> Result<HashMap<std::string::String, CommandResultDto>, Error<PollError>> {
        let poll_body = PollRequestBodyDto::new(request_keys.request_keys, self.conf.get_hostapi());
        kadena_proxy_api::poll(&self.proxy_conf, poll_body).await
    }

    pub async fn estimate_gas(&self) -> Result<u64> {
        let cmd = LocalRequestBodyDto::new(self.cmd.clone(), self.conf.get_hostapi(), true, false);
        let local_rsp = kadena_proxy_api::local(&self.proxy_conf, cmd).await?;
        Ok(local_rsp.gas)
    }
}
