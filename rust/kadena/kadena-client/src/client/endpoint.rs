use super::KadenaProxyClient;
use crate::models::{
    BuildPactTxDto, ContinueTransferRemoteDto, LocalRequestBodyDto, PollRequestBodyDto,
    SendRequestBodyDto,
};
use reqwest::{Method, Request};

/// Endpoint enum that represents all possible endpoints.
pub(crate) enum Endpoint {
    BuildTx(BuildPactTxDto),
    BlockByHash(String),
    BlockByHeight(u64),
    Blocks { from: u64, to: u64 },
    ContinueTransferRemote(ContinueTransferRemoteDto),
    EventByHash(String),
    EventByHeight(u64),
    Events { from: u64, to: u64 },
    HeaderByHash(String),
    HeaderByHeight(u64),
    Headers { from: u64, to: u64 },
    TxByHash(String),
    TxByHeight(u64),
    Txs { from: u64, to: u64 },
    Height { depth: Option<u64> },
    Local(LocalRequestBodyDto),
    Send(SendRequestBodyDto),
    Poll(PollRequestBodyDto),
}

impl Endpoint {
    pub(crate) const BUILD_TX_PATH: &'static str = "/build_pact_tx";
    pub(crate) const BLOCK_BY_HASH_PATH: &'static str = "/block_by_hash";
    pub(crate) const BLOCK_BY_HEIGHT_PATH: &'static str = "/block_by_height";
    pub(crate) const BLOCKS_PATH: &'static str = "/blocks";
    pub(crate) const CONTINUE_TRANSFER_REMOTE_PATH: &'static str = "/continue_transfer_remote";
    pub(crate) const EVENT_BY_HASH_PATH: &'static str = "/event_by_hash";
    pub(crate) const EVENT_BY_HEIGHT_PATH: &'static str = "/event_by_height";
    pub(crate) const EVENTS_PATH: &'static str = "/events";
    pub(crate) const HEADER_BY_HASH_PATH: &'static str = "/header_by_hash";
    pub(crate) const HEADER_BY_HEIGHT_PATH: &'static str = "/header_by_height";
    pub(crate) const HEADERS_PATH: &'static str = "/headers";
    pub(crate) const TX_BY_HASH_PATH: &'static str = "/tx_by_hash";
    pub(crate) const TX_BY_HEIGHT_PATH: &'static str = "/tx_by_height";
    pub(crate) const TXS_PATH: &'static str = "/txs";
    pub(crate) const HEIGHT_PATH: &'static str = "/height";
    pub(crate) const LOCAL_PATH: &'static str = "/local";
    pub(crate) const SEND_PATH: &'static str = "/send";
    pub(crate) const POLL_PATH: &'static str = "/poll";

    /// Returns a `Request` for the given `Endpoint`.
    pub(crate) fn request(&self, client: &KadenaProxyClient) -> Request {
        match self {
            Endpoint::BuildTx(build_pact_tx_dto) => {
                let url = client.base_url().join(Self::BUILD_TX_PATH).unwrap();
                client
                    .reqwest_client
                    .post(url.as_ref())
                    .json(build_pact_tx_dto)
                    .build()
                    .unwrap()
            }
            Endpoint::BlockByHash(hash) => {
                let url = client.base_url().join(Self::BLOCK_BY_HASH_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("hash", hash),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::BlockByHeight(height) => {
                let url = client.base_url().join(Self::BLOCK_BY_HEIGHT_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("height", &height.to_string()),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::Blocks { from, to } => {
                let url = client.base_url().join(Self::BLOCKS_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("from", &from.to_string()),
                    ("to", &to.to_string()),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }

            Endpoint::ContinueTransferRemote(continue_transfer_remote_dto) => {
                let url = client
                    .base_url()
                    .join(Self::CONTINUE_TRANSFER_REMOTE_PATH)
                    .unwrap();
                client
                    .reqwest_client
                    .post(url.as_ref())
                    .json(continue_transfer_remote_dto)
                    .build()
                    .unwrap()
            }

            Endpoint::EventByHash(hash) => {
                let url = client.base_url().join(Self::EVENT_BY_HASH_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("hash", hash),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::EventByHeight(height) => {
                let url = client.base_url().join(Self::EVENT_BY_HEIGHT_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("height", &height.to_string()),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::Events { from, to } => {
                let url = client.base_url().join(Self::EVENTS_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("from", &from.to_string()),
                    ("to", &to.to_string()),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::HeaderByHash(hash) => {
                let url = client.base_url().join(Self::HEADER_BY_HASH_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("hash", hash),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::HeaderByHeight(height) => {
                let url = client.base_url().join(Self::HEADER_BY_HEIGHT_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("height", &height.to_string()),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::Headers { from, to } => {
                let url = client.base_url().join(Self::HEADERS_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("from", &from.to_string()),
                    ("to", &to.to_string()),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::TxByHash(hash) => {
                let url = client.base_url().join(Self::TX_BY_HASH_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("hash", hash),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::TxByHeight(height) => {
                let url = client.base_url().join(Self::TX_BY_HEIGHT_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("height", &height.to_string()),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::Txs { from, to } => {
                let url = client.base_url().join(Self::TXS_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = [
                    ("host", conf.url.as_ref()),
                    ("network", &conf.network_id),
                    ("chain_id", &conf.chain_id.to_string()),
                    ("from", &from.to_string()),
                    ("to", &to.to_string()),
                ];

                client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params)
                    .build()
                    .unwrap()
            }
            Endpoint::Height { depth } => {
                let url = client.base_url().join(Self::HEIGHT_PATH).unwrap();
                let conf = client.chainweb_conf();

                let params = vec![("host", conf.url.as_ref()), ("network", &conf.network_id)];

                let mut req_builder = client
                    .reqwest_client
                    .request(Method::GET, url.as_ref())
                    .query(&params);

                if let Some(value) = depth {
                    req_builder = req_builder.query(&vec![("depth", &value.to_string())]);
                }

                req_builder.build().unwrap()
            }
            Endpoint::Local(local_request_body_dto) => {
                let url = client.base_url().join(Self::LOCAL_PATH).unwrap();
                client
                    .reqwest_client
                    .post(url.as_ref())
                    .json(local_request_body_dto)
                    .build()
                    .unwrap()
            }
            Endpoint::Send(send_request_body_dto) => {
                let url = client.base_url().join(Self::SEND_PATH).unwrap();
                client
                    .reqwest_client
                    .post(url.as_ref())
                    .json(send_request_body_dto)
                    .build()
                    .unwrap()
            }
            Endpoint::Poll(poll_request_body_dto) => {
                let url = client.base_url().join(Self::POLL_PATH).unwrap();
                client
                    .reqwest_client
                    .post(url.as_ref())
                    .json(poll_request_body_dto)
                    .build()
                    .unwrap()
            }
        }
    }
}
