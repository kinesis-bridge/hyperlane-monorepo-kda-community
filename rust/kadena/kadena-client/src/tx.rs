use std::collections::HashMap;

use crate::contract_call::ContractCall;
use crate::models::CommandResultDto;
use anyhow::Result;
use hyperlane_core::ChainCommunicationError;
use tracing::{error, info};

/// Number of times to retry polling
const POLL_RETRY_COUNT: u8 = 10;

/// Polling interval in seconds
const POLL_RETRY_INTERVAL: u8 = 30;

pub async fn report_tx(tx: ContractCall) -> Result<HashMap<String, CommandResultDto>> {
    info!("Dispatching transaction");
    let send_rsp = tx.send().await?;

    let mut interval =
        tokio::time::interval(std::time::Duration::from_secs(POLL_RETRY_INTERVAL as u64));

    for retry in 0..POLL_RETRY_COUNT {
        let poll_rsp = tx.poll(send_rsp.clone()).await?;
        if poll_rsp.len() > 0
            && poll_rsp
                .keys()
                .all(|key| send_rsp.request_keys.contains(key))
        {
            info!(?send_rsp.request_keys, "receipt received");
            return Ok(poll_rsp);
        }
        if retry < POLL_RETRY_COUNT - 1 {
            interval.tick().await;
            info!(?send_rsp.request_keys, ?retry, "waiting for receipt");
        }
    }
    error!(?send_rsp.request_keys, "waiting for receipt timed out");
    Err(anyhow::Error::from(
        ChainCommunicationError::TransactionTimeout(),
    ))
}
