use std::{
    collections::HashMap,
    sync::Arc,
};

use crate::{
    client::KadenaProxyClient,
    contract::Contract,
    error::KadenaClientError,
    models::{
        CommandDto,
        CommandResultDto,
        LocalRequestBodyDto,
        PollRequestBodyDto,
        RequestKeysDto,
        SendRequestBodyDto,
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait ContractCall: Send + Sync {
    /// Returns the contract that this call is for.
    fn contract(&self) -> &dyn Contract;

    /// Returns the proxy client that this call is for.
    fn proxy_client(&self) -> Arc<KadenaProxyClient> {
        self.contract().provider().proxy_client().clone()
    }

    /// Sets the gas limit for this call.
    fn set_gas_limit(&mut self, gas_limit: u64);

    /// Returns the gas limit for this call.
    fn gas_limit(&self) -> Option<u64>;

    /// Returns the command DTO for this call. This is the main DTO for the call.
    async fn cmd(&self) -> Result<CommandDto, KadenaClientError>;

    /// Sends the transaction to the blockchain via the proxy.
    async fn send(&self) -> Result<RequestKeysDto, KadenaClientError> {
        let mut cmd = self.cmd().await?;
        let _ = self
            .contract()
            .provider()
            .signer()
            .sign_transaction(&mut cmd)
            .await?;
        let client = self.proxy_client();
        let send_body = SendRequestBodyDto::new(vec![cmd], client.hostapi());
        client.send(send_body).await
    }

    /// Preforms a local call to the blockchain via the proxy.
    async fn local(&self) -> Result<CommandResultDto, KadenaClientError> {
        let client = self.proxy_client();
        let local_body = LocalRequestBodyDto::new(self.cmd().await?, client.hostapi(), true, false);
        client.local(local_body).await
    }

    /// Polls the blockchain via the proxy.
    async fn poll(
        &self,
        request_keys: RequestKeysDto,
    ) -> Result<HashMap<String, CommandResultDto>, KadenaClientError> {
        let client = self.proxy_client();
        let poll_body = PollRequestBodyDto::new(request_keys.request_keys, client.hostapi());
        client.poll(poll_body).await
    }

    /// Estimates the gas for this call using local endpoint.
    async fn estimate_gas(&self) -> Result<u64, KadenaClientError> {
        let client = self.proxy_client();
        let cmd = LocalRequestBodyDto::new(self.cmd().await?, client.hostapi(), true, false);
        let local_rsp = client.local(cmd).await?;
        Ok(local_rsp.gas)
    }
}
