#![allow(warnings)]
#![allow(clippy::all)]
#![allow(missing_docs)]

pub mod i_interchain_gas_paymaster;
pub mod i_interchain_security_module;
pub mod i_mailbox;
pub mod i_merkle_tree_hook;
pub mod i_multisig_ism;
pub mod i_validator_announce;

use std::primitive;

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use hyperlane_core::{LogMeta, H256, H512, U256};
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
