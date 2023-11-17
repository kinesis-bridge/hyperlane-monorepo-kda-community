use std::collections::HashMap;

use crate::apis::configuration::{Configuration, ConnectionConf};
use crate::apis::kadena_proxy_api;
use crate::models::{CommandDto, CommandResultDto, LocalRequestBodyDto};
use crate::models::{PollRequestBodyDto, SendRequestBodyDto};
use anyhow::Result;
use hyperlane_core::ChainCommunicationError;
use tracing::{error, info};

/// Number of times to retry polling
const POLL_RETRY_COUNT: u8 = 10;

/// Polling interval in seconds
const POLL_RETRY_INTERVAL: u8 = 30;

pub async fn report_tx(
    conf: &ConnectionConf,
    cmds: Vec<CommandDto>,
) -> Result<HashMap<String, CommandResultDto>> {
    let config = Configuration::new();
    let send_request_body_dto = SendRequestBodyDto::new(cmds, conf.get_hostapi());

    let send_rsp = kadena_proxy_api::send(&config, send_request_body_dto).await?;

    let poll_req_body_dto =
        PollRequestBodyDto::new(send_rsp.request_keys.clone(), conf.get_hostapi());
    let mut interval =
        tokio::time::interval(std::time::Duration::from_secs(POLL_RETRY_INTERVAL as u64));

    for retry in 0..POLL_RETRY_COUNT {
        let poll_rsp = kadena_proxy_api::poll(&config, poll_req_body_dto.clone()).await?;
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

pub async fn estimate_gas(conf: &ConnectionConf, cmd: CommandDto) -> Result<u64> {
    let cmd = LocalRequestBodyDto::new(cmd, conf.get_hostapi(), true, false);
    let local_rsp = kadena_proxy_api::local(&Configuration::new(), cmd).await?;
    Ok(local_rsp.gas)
}
