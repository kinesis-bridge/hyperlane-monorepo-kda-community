use std::{collections::HashMap, sync::Arc};

use crate::{
    client::KadenaProxyClient,
    contract::Contract,
    error::KadenaClientError,
    models::{
        CommandDto, CommandResultDto, LocalRequestBodyDto, PollRequestBodyDto, RequestKeysDto,
        SendRequestBodyDto,
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait ContractCall: Send + Sync {
    type Output;

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

    /// Performs a local call to the blockchain via the proxy.
    async fn local(
        &self,
        rewind_depth: Option<u64>,
    ) -> Result<CommandResultDto, KadenaClientError> {
        let client = self.proxy_client();
        let local_body = LocalRequestBodyDto::new(
            self.cmd().await?,
            client.hostapi(),
            false,
            false,
            rewind_depth,
        );
        client.local(local_body).await
    }

    /// Implements the local call to the blockchain via the proxy with optional rewind depth
    /// and returns the typed output.
    async fn local_typed_impl(
        &self,
        rewind_depth: Option<u64>,
    ) -> Result<Self::Output, KadenaClientError>;

    /// Performs a local call to the blockchain via the proxy and returns the typed output.
    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        self.local_typed_impl(None).await
    }

    /// Performs a local call to the blockchain via the proxy with rewind depth and returns the typed output.
    async fn local_typed_with_rewind_depth(
        &self,
        rewind_depth: Option<u64>,
    ) -> Result<Self::Output, KadenaClientError> {
        self.local_typed_impl(rewind_depth).await
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
        let cmd = LocalRequestBodyDto::new(self.cmd().await?, client.hostapi(), true, false, None);
        let local_rsp = client.local(cmd).await?;
        Ok(local_rsp.gas)
    }

    /// Returns the destination chain id for this call.
    /// None means transfer is local.   
    fn with_transfer_remote(&self) -> Option<u16> {
        None
    }
}
