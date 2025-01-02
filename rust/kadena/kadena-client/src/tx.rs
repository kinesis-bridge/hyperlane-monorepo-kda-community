use std::collections::HashMap;

use crate::{contract_call::ContractCall, error::KadenaClientError, models::CommandResultDto};
use tracing::{error, info};

/// Buffer to add to gas estimate
const GAS_ESTIMATE_BUFFER: u64 = 50000;

/// Number of times to retry polling
const POLL_RETRY_COUNT: u8 = 10;

/// Polling interval in seconds
const POLL_RETRY_INTERVAL: u8 = 30;

pub async fn report_tx<C: ContractCall>(
    tx: &C,
) -> Result<HashMap<String, CommandResultDto>, KadenaClientError> {
    info!("Dispatching transaction");
    let send_rsp = tx.send().await?;

    let mut interval =
        tokio::time::interval(std::time::Duration::from_secs(POLL_RETRY_INTERVAL as u64));

    for retry in 0..POLL_RETRY_COUNT {
        let poll_rsp = tx
            .poll(send_rsp.clone())
            .await
            .map_err(|e| KadenaClientError::OtherError(Box::new(e)))?;
        if !poll_rsp.is_empty()
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
    Err(KadenaClientError::PollTimeoutError(send_rsp.request_keys))
}

pub async fn fill_tx_gas_params<C: ContractCall>(
    tx: C,
    tx_gas_limit: Option<u64>,
) -> Result<C, KadenaClientError> {
    let gas_limit = if let Some(gas_limit) = tx_gas_limit {
        gas_limit
    } else {
        tx.estimate_gas().await?.saturating_add(GAS_ESTIMATE_BUFFER)
    };
    let mut tx = tx;
    tx.set_gas_limit(gas_limit);
    Ok(tx)
}
