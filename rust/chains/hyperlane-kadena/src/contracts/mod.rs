#![allow(warnings)]
#![allow(clippy::all)]
#![allow(missing_docs)]

pub mod i_mailbox;
pub mod i_validator_announce;
pub mod i_interchain_gas_paymaster;
pub mod i_interchain_security_module;
pub mod i_multisig_ism;
pub mod i_merkle_tree_hook;

use hyperlane_core::{LogMeta, H256, H512, U256};
use kadena_client::models::EventDataDto;
use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};

#[derive(Debug, Clone)]
pub struct LogMetaProxy(LogMeta);

impl From<LogMetaProxy> for LogMeta {
    fn from(proxy: LogMetaProxy) -> Self {
        proxy.0
    }
}

impl From<EventDataDto> for LogMetaProxy {
    fn from(event: EventDataDto) -> Self {
        let address_bin = BASE64_URL_SAFE_NO_PAD.decode(event.module_hash).unwrap_or(b"Unknown module".to_vec());
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