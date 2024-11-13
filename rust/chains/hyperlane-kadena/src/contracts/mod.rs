#![allow(warnings)]
#![allow(clippy::all)]
#![allow(missing_docs)]

pub mod i_interchain_gas_paymaster;
pub mod i_interchain_security_module;
pub mod i_mailbox;
pub mod i_merkle_tree_hook;
pub mod i_multisig_ism;
pub mod i_validator_announce;

use serde::ser::SerializeStruct;
use serde::{
    de::{self, Error},
    Deserialize, Serialize, Serializer,
};
use std::primitive;

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use hyperlane_core::{HyperlaneMessage, LogMeta, H256, H512, U256};
use kadena_client::{
    error::KadenaClientError,
    models::{EventDataDto, EventParam},
};

#[derive(Debug, Clone)]
pub struct LogMetaProxy(LogMeta);

impl From<LogMetaProxy> for LogMeta {
    fn from(proxy: LogMetaProxy) -> Self {
        proxy.0
    }
}

impl From<EventDataDto> for LogMetaProxy {
    fn from(event: EventDataDto) -> Self {
        let address_bin = BASE64_URL_SAFE_NO_PAD
            .decode(event.module_hash)
            .unwrap_or(b"Unknown module".to_vec());
        let mut address: [u8; 32] = [0; 32];
        let len = std::cmp::min(address_bin.len(), address.len());
        address[..len].copy_from_slice(&address_bin[..len]);

        LogMetaProxy(LogMeta {
            address: address.into(),
            block_number: event.height.unwrap_or_default(),
            // TODO: implement these when adding Scraper support
            block_hash: H256::zero(),
            transaction_id: H512::zero(),
            transaction_index: 0,
            log_index: U256::zero(),
        })
    }
}

pub struct U256Proxy(U256);

impl From<U256Proxy> for U256 {
    fn from(proxy: U256Proxy) -> Self {
        proxy.0
    }
}

impl TryFrom<&EventParam> for U256Proxy {
    type Error = KadenaClientError;
    fn try_from(param: &EventParam) -> Result<Self, KadenaClientError> {
        let primitive_u256: primitive_types::U256 = param.try_into()?;
        Ok(U256Proxy(U256(primitive_u256.0)))
    }
}

pub struct PactHyperlaneMessage(HyperlaneMessage);

impl Serialize for PactHyperlaneMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let HyperlaneMessage {
            version,
            nonce,
            origin,
            sender,
            destination,
            recipient,
            body,
        } = &self.0;

        let sender = BASE64_URL_SAFE_NO_PAD.encode(sender);
        let recipient = BASE64_URL_SAFE_NO_PAD.encode(recipient);
        let body = BASE64_URL_SAFE_NO_PAD.encode(body);

        let mut state = serializer.serialize_struct("HyperlaneMessage", 7)?;
        state.serialize_field("version", &version)?;
        state.serialize_field("nonce", &nonce)?;
        state.serialize_field("originDomain", &origin)?;
        state.serialize_field("sender", &sender)?;
        state.serialize_field("destinationDomain", &destination)?;
        state.serialize_field("recipient", &recipient)?;
        state.serialize_field("messageBody", &body)?;
        state.end()
    }
}

// This is a conversion from a tuple of HyperlaneMessage and decode token message (as serde_json::Value) to PactHyperlaneMessage
impl From<HyperlaneMessage> for PactHyperlaneMessage {
    fn from(msg: HyperlaneMessage) -> Self {
        PactHyperlaneMessage(msg)
    }
}
