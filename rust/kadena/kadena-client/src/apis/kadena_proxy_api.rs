use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildTxError {
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlockByHashError {
    DefaultResponse(crate::models::BlockPayloadsDto),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlockByHeightError {
    DefaultResponse(crate::models::BlockPayloadsDto),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlocksError {
    DefaultResponse(Vec<crate::models::BlockPayloadsDto>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventByHashError {
    DefaultResponse(Vec<crate::models::EventDataDto>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventByHeightError {
    DefaultResponse(Vec<crate::models::EventDataDto>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventsError {
    DefaultResponse(Vec<crate::models::EventDataDto>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHeaderByHashError {
    DefaultResponse(crate::models::BlockHeaderDto),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHeaderByHeightError {
    DefaultResponse(crate::models::BlockHeaderDto),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHeadersError {
    DefaultResponse(Vec<crate::models::BlockHeaderDto>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHeightError {
    DefaultResponse(u64),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTxByHashError {
    DefaultResponse(Vec<crate::models::TransactionElementDto>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTxByHeightError {
    DefaultResponse(Vec<crate::models::TransactionElementDto>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTxsError {
    DefaultResponse(Vec<crate::models::TransactionElementDto>),
    UnknownValue(serde_json::Value),
}

pub async fn build_tx(
    configuration: &configuration::Configuration,
    build_pact_tx_dto: crate::models::BuildPactTxDto,
) -> Result<(), Error<BuildTxError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/build_pact_tx", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::POST, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&build_pact_tx_dto);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let entity: Option<BuildTxError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_block_by_hash(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    hash: &str,
) -> Result<crate::models::BlockPayloadsDto, Error<GetBlockByHashError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/block_by_hash", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("hash", &hash.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetBlockByHashError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_block_by_height(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    height: u64,
) -> Result<crate::models::BlockPayloadsDto, Error<GetBlockByHeightError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/block_by_height", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("height", &height.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    println!("content: {}", content);

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetBlockByHeightError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_blocks(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    from: u64,
    to: u64,
) -> Result<Vec<crate::models::BlockPayloadsDto>, Error<GetBlocksError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/blocks", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("from", &from.to_string())]);
    req_builder = req_builder.query(&[("to", &to.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetBlocksError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_event_by_hash(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    hash: &str,
) -> Result<Vec<crate::models::EventDataDto>, Error<GetEventByHashError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/event_by_hash", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("hash", &hash.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetEventByHashError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_event_by_height(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    height: u64,
) -> Result<Vec<crate::models::EventDataDto>, Error<GetEventByHeightError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/event_by_height", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("height", &height.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetEventByHeightError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_events(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    from: u64,
    to: u64,
) -> Result<Vec<crate::models::EventDataDto>, Error<GetEventsError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/events", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("from", &from.to_string())]);
    req_builder = req_builder.query(&[("to", &to.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetEventsError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_header_by_hash(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    hash: &str,
) -> Result<crate::models::BlockHeaderDto, Error<GetHeaderByHashError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/header_by_hash", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("hash", &hash.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetHeaderByHashError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_header_by_height(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    height: u64,
) -> Result<crate::models::BlockHeaderDto, Error<GetHeaderByHeightError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/header_by_height", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("height", &height.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetHeaderByHeightError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_headers(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    from: u64,
    to: u64,
) -> Result<Vec<crate::models::BlockHeaderDto>, Error<GetHeadersError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/headers", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("from", &from.to_string())]);
    req_builder = req_builder.query(&[("to", &to.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetHeadersError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_height(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    depth: Option<u64>,
) -> Result<u64, Error<GetHeightError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/height", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    if let Some(ref str) = depth {
        req_builder = req_builder.query(&[("depth", &str.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetHeightError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_tx_by_hash(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    hash: &str,
) -> Result<Vec<crate::models::TransactionElementDto>, Error<GetTxByHashError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/tx_by_hash", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("hash", &hash.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetTxByHashError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_tx_by_height(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    height: u64,
) -> Result<Vec<crate::models::TransactionElementDto>, Error<GetTxByHeightError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/tx_by_height", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("height", &height.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetTxByHeightError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn get_txs(
    configuration: &configuration::Configuration,
    host: &str,
    network: &str,
    chain_id: u32,
    from: u64,
    to: u64,
) -> Result<Vec<crate::models::TransactionElementDto>, Error<GetTxsError>> {
    let configuration = configuration;

    let client = &configuration.client;

    let uri_str = format!("{}/txs", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    req_builder = req_builder.query(&[("host", &host.to_string())]);
    req_builder = req_builder.query(&[("network", &network.to_string())]);
    req_builder = req_builder.query(&[("chain_id", &chain_id.to_string())]);
    req_builder = req_builder.query(&[("from", &from.to_string())]);
    req_builder = req_builder.query(&[("to", &to.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetTxsError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}
