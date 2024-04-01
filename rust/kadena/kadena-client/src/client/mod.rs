use std::collections::HashMap;

use url::Url;

use crate::{
    error::KadenaClientError,
    models::{
        BlockHeaderDto, BlockPayloadsDto, BuildPactTxDto, CommandDto, CommandResultDto,
        ContinueTransferRemoteDto, EventDataDto, LocalRequestBodyDto, PollRequestBodyDto,
        RequestKeysDto, SendRequestBodyDto, TransactionElementDto,
    },
};

mod chainweb_conf;
pub use chainweb_conf::ChainwebConf;

mod proxy_conf;
pub use proxy_conf::ProxyConf;

mod endpoint;
use endpoint::Endpoint;

/// Kadena Proxy Client is a client for Kadena Proxy API.
#[derive(Clone, Debug)]
pub struct KadenaProxyClient {
    reqwest_client: reqwest::Client,
    proxy_conf: ProxyConf,
    chainweb_conf: ChainwebConf,
}

impl KadenaProxyClient {
    /// Creates a new client with the given base url, proxy configuration, and chainweb
    /// configuration.
    pub fn new(proxy_conf: ProxyConf, chainweb_conf: ChainwebConf) -> Self {
        KadenaProxyClient {
            reqwest_client: reqwest::Client::new(),
            proxy_conf,
            chainweb_conf,
        }
    }

    /// Returns the chainweb configuration.
    pub fn chainweb_conf(&self) -> &ChainwebConf {
        &self.chainweb_conf
    }

    /// Returns the proxy configuration.
    pub fn proxy_conf(&self) -> &ProxyConf {
        &self.proxy_conf
    }

    /// Returns the base url.
    pub fn base_url(&self) -> &Url {
        &self.proxy_conf.url
    }

    /// Returns the hostapi url.
    pub fn hostapi(&self) -> String {
        self.chainweb_conf.hostapi()
    }

    /// Generic request endpoint that takes an endpoint and returns a DTO.
    async fn request_endpoint<T: for<'de> serde::Deserialize<'de>>(
        &self,
        endpoint: Endpoint,
    ) -> Result<T, KadenaClientError> {
        let req = endpoint.request(self);
        let resp = self.reqwest_client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(KadenaClientError::from)
        } else {
            Err(KadenaClientError::HttpResponseError(status, content))
        }
    }

    /// Builds a transaction.
    pub async fn build_tx(
        &self,
        build_pact_tx_dto: BuildPactTxDto,
    ) -> Result<CommandDto, KadenaClientError> {
        self.request_endpoint(Endpoint::BuildTx(build_pact_tx_dto))
            .await
    }

    /// Continues a remote transfer.
    pub async fn continue_transfer_remote(
        &self,
        continue_transfer_remote_dto: ContinueTransferRemoteDto,
    ) -> Result<CommandResultDto, KadenaClientError> {
        self.request_endpoint(Endpoint::ContinueTransferRemote(
            continue_transfer_remote_dto,
        ))
        .await
    }

    /// Polls for a transaction.
    pub async fn poll(
        &self,
        poll_request_body_dto: PollRequestBodyDto,
    ) -> Result<HashMap<String, CommandResultDto>, KadenaClientError> {
        self.request_endpoint(Endpoint::Poll(poll_request_body_dto))
            .await
    }

    /// Reads a state (aka dirty read) or performs dry run
    pub async fn local(
        &self,
        local_request_body_dto: LocalRequestBodyDto,
    ) -> Result<CommandResultDto, KadenaClientError> {
        self.request_endpoint(Endpoint::Local(local_request_body_dto))
            .await
    }

    /// Sends a transaction to the blockchain.
    pub async fn send(
        &self,
        send_request_body_dto: SendRequestBodyDto,
    ) -> Result<RequestKeysDto, KadenaClientError> {
        self.request_endpoint(Endpoint::Send(send_request_body_dto))
            .await
    }

    /// Gets a block by hash.
    pub async fn block_by_hash(&self, hash: String) -> Result<BlockPayloadsDto, KadenaClientError> {
        self.request_endpoint(Endpoint::BlockByHash(hash)).await
    }

    /// Gets a block by height.
    pub async fn block_by_height(
        &self,
        height: u64,
    ) -> Result<BlockPayloadsDto, KadenaClientError> {
        self.request_endpoint(Endpoint::BlockByHeight(height)).await
    }

    /// Gets blocks by height range.
    pub async fn blocks(
        &self,
        from: u64,
        to: u64,
    ) -> Result<Vec<BlockPayloadsDto>, KadenaClientError> {
        self.request_endpoint(Endpoint::Blocks { from, to }).await
    }

    /// Gets a transaction by hash.
    pub async fn tx_by_hash(
        &self,
        hash: String,
    ) -> Result<Vec<TransactionElementDto>, KadenaClientError> {
        self.request_endpoint(Endpoint::TxByHash(hash)).await
    }

    /// Gets a transaction by height.
    pub async fn tx_by_height(
        &self,
        height: u64,
    ) -> Result<Vec<TransactionElementDto>, KadenaClientError> {
        self.request_endpoint(Endpoint::TxByHeight(height)).await
    }

    /// Gets transactions by height range.
    pub async fn txs(
        &self,
        from: u64,
        to: u64,
    ) -> Result<Vec<TransactionElementDto>, KadenaClientError> {
        self.request_endpoint(Endpoint::Txs { from, to }).await
    }

    /// Gets an event by hash.
    pub async fn event_by_hash(
        &self,
        hash: String,
    ) -> Result<Vec<EventDataDto>, KadenaClientError> {
        self.request_endpoint(Endpoint::EventByHash(hash)).await
    }

    /// Gets an event by height.
    pub async fn event_by_height(
        &self,
        height: u64,
    ) -> Result<Vec<EventDataDto>, KadenaClientError> {
        self.request_endpoint(Endpoint::EventByHeight(height)).await
    }

    /// Gets events by height range.
    pub async fn events(&self, from: u64, to: u64) -> Result<Vec<EventDataDto>, KadenaClientError> {
        self.request_endpoint(Endpoint::Events { from, to }).await
    }

    /// Gets a block header by hash.
    pub async fn header_by_hash(&self, hash: String) -> Result<BlockHeaderDto, KadenaClientError> {
        self.request_endpoint(Endpoint::HeaderByHash(hash)).await
    }

    /// Gets a block header by height.
    pub async fn header_by_height(&self, height: u64) -> Result<BlockHeaderDto, KadenaClientError> {
        self.request_endpoint(Endpoint::HeaderByHeight(height))
            .await
    }

    /// Gets block headers by height range.
    pub async fn headers(
        &self,
        from: u64,
        to: u64,
    ) -> Result<Vec<BlockHeaderDto>, KadenaClientError> {
        self.request_endpoint(Endpoint::Headers { from, to }).await
    }

    /// Gets the height of the chain with optional depth.
    pub async fn height(&self, depth: Option<u64>) -> Result<u64, KadenaClientError> {
        self.request_endpoint(Endpoint::Height { depth }).await
    }
}

#[cfg(test)]
mod tests;
