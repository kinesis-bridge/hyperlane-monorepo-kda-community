use std::{str::FromStr, sync::Arc};

use crate::{provider, KadenaProvider};

use async_trait::async_trait;
use hyperlane_core::{HyperlaneMessage, LogMeta, RawHyperlaneMessage, H160, H256, U256};
use kadena_client::{
    contract::{Contract, KadenaProxyProvider},
    contract_call::ContractCall,
    error::KadenaClientError,
    event::{Event, EventData},
    models::{CommandDto, EventDataDto, EventParamMonoType, EventParamType, VerifierDto},
    pact::IntObject,
};

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};

use serde::ser::SerializeStruct;
use serde::{
    de::{self, Error},
    Deserialize, Serialize, Serializer,
};
use serde_json::{json, Value};

use tracing::info;

use super::LogMetaProxy;

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

pub struct VerifierHyperlaneMessage(HyperlaneMessage);

impl Serialize for VerifierHyperlaneMessage {
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

        let version = IntObject {
            int: *version as u64,
        };
        let nonce = IntObject { int: *nonce as u64 };
        let origin = IntObject {
            int: *origin as u64,
        };
        let destination = IntObject {
            int: *destination as u64,
        };

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
impl From<HyperlaneMessage> for VerifierHyperlaneMessage {
    fn from(msg: HyperlaneMessage) -> Self {
        VerifierHyperlaneMessage(msg)
    }
}

#[derive(Debug, Clone)]
pub struct DispatchEventData {
    pub sender: H256,
    pub destination: u32,
    pub recipient: H256,
    pub message: HyperlaneMessage,
    pub log: LogMetaProxy,
}

impl TryFrom<EventDataDto> for DispatchEventData {
    type Error = KadenaClientError;
    fn try_from(mut event_data_dto: EventDataDto) -> Result<Self, KadenaClientError> {
        let args = Self::check_params(std::mem::take(&mut event_data_dto.params), Self::params())?;

        // Assembling message from event data

        let version = (&args[0]).try_into()?;
        let nonce = (&args[1]).try_into()?;

        // We should store origin in the event, but it's not available yet
        let origin = 626u32;

        // As of now, sender is base64 url encoded string, it should be just bypassed as H256
        let sender_vec = BASE64_URL_SAFE_NO_PAD
            .decode(&args[2].to_string())
            .map_err(|e| KadenaClientError::OtherError(Box::new(e)))?;

        if sender_vec.len() > 32 {
            return Err(KadenaClientError::DeserializationError(de::Error::custom(
                "Sender is too long",
            )));
        }

        let mut sender = [0u8; 32];
        let start_index = sender.len().saturating_sub(sender_vec.len());
        sender[start_index..].copy_from_slice(&sender_vec);
        let sender = H256::from(sender);

        let destination = (&args[3]).try_into()?;

        let recipient_vec = BASE64_URL_SAFE_NO_PAD
            .decode(&args[4].to_string())
            .map_err(|e| KadenaClientError::OtherError(Box::new(e)))?;

        if recipient_vec.len() > 32 {
            return Err(KadenaClientError::DeserializationError(de::Error::custom(
                "Recipient is too long",
            )));
        }

        let mut recipient_arr = [0u8; 32];
        let start_index = recipient_arr.len().saturating_sub(recipient_vec.len());
        recipient_arr[start_index..].copy_from_slice(&recipient_vec);

        let recipient = H256::from(recipient_arr);

        let message_body = BASE64_URL_SAFE_NO_PAD
            .decode(&args[5].to_string())
            .map_err(|e| KadenaClientError::OtherError(Box::new(e)))?;

        let message = HyperlaneMessage {
            version,
            nonce,
            origin,
            sender,
            destination,
            recipient,
            body: message_body.to_vec(),
        };

        Ok(Self {
            sender,
            destination,
            recipient,
            message,
            log: event_data_dto.into(),
        })
    }
}

impl EventData for DispatchEventData {
    fn params() -> &'static [EventParamType] {
        &[
            EventParamType::MonoType(EventParamMonoType::IntObject), // version
            EventParamType::MonoType(EventParamMonoType::IntObject), // nonce
            EventParamType::MonoType(EventParamMonoType::String),    // sender
            EventParamType::MonoType(EventParamMonoType::String),    // destination
            EventParamType::MonoType(EventParamMonoType::String),    // recipient
            EventParamType::MonoType(EventParamMonoType::String),    // message-body
        ]
    }
}

pub struct DispatchEvent<'a> {
    contract: &'a IMailbox,
}

impl<'a> DispatchEvent<'a> {
    const EVENT_NAME: &'static str = "DISPATCH";

    pub fn new(contract: &'a IMailbox) -> Self {
        Self { contract }
    }
}

impl Event for DispatchEvent<'_> {
    type DataType = DispatchEventData;
    type Error = KadenaClientError;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

#[derive(Debug, Clone)]
pub struct DispatchIdEventData {
    pub id: H256,
    pub log: LogMeta,
}

impl TryFrom<EventDataDto> for DispatchIdEventData {
    type Error = KadenaClientError;
    fn try_from(mut event_data_dto: EventDataDto) -> Result<Self, Self::Error> {
        let args = Self::check_params(std::mem::take(&mut event_data_dto.params), Self::params())?;

        let id = H256::from_slice(&BASE64_URL_SAFE_NO_PAD.decode(args[0].to_string())?);

        Ok(Self {
            id,
            log: LogMetaProxy::from(event_data_dto).into(),
        })
    }
}

impl EventData for DispatchIdEventData {
    fn params() -> &'static [EventParamType] {
        &[EventParamType::MonoType(EventParamMonoType::String)] // id
    }
}

pub struct DispatchIdEvent<'a> {
    contract: &'a IMailbox,
}

impl<'a> DispatchIdEvent<'a> {
    const EVENT_NAME: &'static str = "DISPATCH-ID";

    pub fn new(contract: &'a IMailbox) -> Self {
        Self { contract }
    }
}

impl Event for DispatchIdEvent<'_> {
    type DataType = DispatchIdEventData;
    type Error = KadenaClientError;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

#[derive(Debug, Clone)]
pub struct ProcessEventData {
    pub origin: String,
    pub sender: String,
    pub recipient: String,
    pub log: LogMeta,
}

impl TryFrom<EventDataDto> for ProcessEventData {
    type Error = KadenaClientError;
    fn try_from(mut event_data_dto: EventDataDto) -> Result<Self, KadenaClientError> {
        let args = Self::check_params(std::mem::take(&mut event_data_dto.params), Self::params())?;
        let origin = args[0].to_string();
        let sender = args[1].to_string();
        let recipient = args[2].to_string();

        Ok(Self {
            origin,
            sender,
            recipient,
            log: LogMetaProxy::from(event_data_dto).into(),
        })
    }
}

impl EventData for ProcessEventData {
    fn params() -> &'static [EventParamType] {
        &[
            EventParamType::MonoType(EventParamMonoType::String),
            EventParamType::MonoType(EventParamMonoType::String),
            EventParamType::MonoType(EventParamMonoType::String),
        ]
    }
}

pub struct ProcessEvent<'a> {
    contract: &'a IMailbox,
}

impl<'a> ProcessEvent<'a> {
    const EVENT_NAME: &'static str = "PROCESS";

    pub fn new(contract: &'a IMailbox) -> Self {
        Self { contract }
    }
}

impl Event for ProcessEvent<'_> {
    type DataType = ProcessEventData;
    type Error = KadenaClientError;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

#[derive(Debug, Clone)]
pub struct ProcessIdEventData {
    pub id: H256,
    pub log: LogMeta,
}

impl TryFrom<EventDataDto> for ProcessIdEventData {
    type Error = KadenaClientError;
    fn try_from(mut event_data_dto: EventDataDto) -> Result<Self, KadenaClientError> {
        let args = Self::check_params(std::mem::take(&mut event_data_dto.params), Self::params())?;
        let id = H256::from_slice(&BASE64_URL_SAFE_NO_PAD.decode(args[0].to_string())?);

        Ok(Self {
            id,
            log: LogMetaProxy::from(event_data_dto).into(),
        })
    }
}

impl EventData for ProcessIdEventData {
    fn params() -> &'static [EventParamType] {
        &[EventParamType::MonoType(EventParamMonoType::String)]
    }
}

pub struct ProcessIdEvent<'a> {
    contract: &'a IMailbox,
}

impl<'a> ProcessIdEvent<'a> {
    const EVENT_NAME: &'static str = "PROCESS-ID";

    pub fn new(contract: &'a IMailbox) -> Self {
        Self { contract }
    }
}

impl Event for ProcessIdEvent<'_> {
    type DataType = ProcessIdEventData;
    type Error = KadenaClientError;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

pub struct DecodeTokenMessageCall<'a> {
    contract: &'a IMailbox,
    token_message: Vec<u8>,
    gas_limit: Option<u64>,
}

impl DecodeTokenMessageCall<'_> {
    const METHOD_NAME: &'static str = "decode-token-message";
    pub fn new(contract: &IMailbox, token_message: Vec<u8>) -> DecodeTokenMessageCall {
        DecodeTokenMessageCall {
            contract,
            token_message,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for DecodeTokenMessageCall<'_> {
    type Output = Value;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{} \"{}\")",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                    BASE64_URL_SAFE_NO_PAD.encode(&self.token_message),
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        let mut pact_tm: Value = self.local().await?.result()?;

        // TODO: remove this when the smart contract side returns correct type formats

        // Convert amount from u64 to f64 as it's expected in the smart contract
        match pact_tm["amount"] {
            Value::Number(ref num) => {
                if num.is_u64() {
                    let amount = num.as_u64().unwrap();
                    pact_tm["amount"] = json!(amount as f64);
                } else if num.is_i64() {
                    return Err(KadenaClientError::DeserializationError(
                        serde_json::Error::custom("Amount is negative"),
                    ));
                }
            }
            _ => {
                return Err(KadenaClientError::DeserializationError(
                    serde_json::Error::custom("Failed to parse amount as u64"),
                ));
            }
        };

        // Convert chainId from int Object to int as it's expected in the smart contract
        let chain_id = pact_tm["chainId"]
            .as_object()
            .and_then(|map| map.get("int").and_then(Value::as_u64))
            .ok_or_else(|| {
                KadenaClientError::DeserializationError(serde_json::Error::custom(
                    "Failed to parse chainId as u64",
                ))
            })?;

        pact_tm["chainId"] = json!(chain_id);

        // Convert recipient from Object to String as it's expected in the smart contract
        let recipient_obj = pact_tm["recipient"].clone();
        let pact_tm_recipient = recipient_obj.to_string();
        pact_tm["recipient"] = json!(pact_tm_recipient);

        Ok(pact_tm)
    }
}

pub struct DeliveredCall<'a> {
    contract: &'a IMailbox,
    message_id: [u8; 32],
    gas_limit: Option<u64>,
}

impl DeliveredCall<'_> {
    const METHOD_NAME: &'static str = "delivered";
    pub fn new(contract: &IMailbox, message_id: [u8; 32]) -> DeliveredCall {
        DeliveredCall {
            contract,
            message_id,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for DeliveredCall<'_> {
    type Output = bool;
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{} \"{}\")",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                    BASE64_URL_SAFE_NO_PAD.encode(&self.message_id),
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        Ok(self.local().await?.result()?.as_bool().ok_or_else(|| {
            KadenaClientError::TypeConversionError("Failed to convert result to bool".to_string())
        })?)
    }
}

pub struct NonceCall<'a> {
    contract: &'a IMailbox,
    gas_limit: Option<u64>,
}

impl NonceCall<'_> {
    const METHOD_NAME: &'static str = "nonce";
    pub fn new(contract: &IMailbox) -> NonceCall {
        NonceCall {
            contract,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for NonceCall<'_> {
    type Output = u32;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{})",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        Ok(self.local().await?.result()?.as_u64().ok_or_else(|| {
            KadenaClientError::TypeConversionError("Failed to convert result to u32".to_string())
        })? as u32)
    }
}

pub struct ProcessCall<'a> {
    contract: &'a IMailbox,
    metadata: Vec<u8>,
    message: HyperlaneMessage,
    pact_tm: Value,
    destination_chain_id: Option<u16>,
    gas_limit: Option<u64>,
}

impl ProcessCall<'_> {
    const METHOD_NAME: &'static str = "process";
    pub fn new(
        contract: &IMailbox,
        metadata: Vec<u8>,
        message: HyperlaneMessage,
        pact_tm: Value,
    ) -> ProcessCall {
        ProcessCall {
            contract,
            metadata,
            message,
            pact_tm,
            gas_limit: None,
            destination_chain_id: None,
        }
    }

    pub fn set_destination_chain_id(&mut self, destination_chain_id: Option<u16>) {
        self.destination_chain_id = destination_chain_id;
    }
}

#[async_trait]
impl ContractCall for ProcessCall<'_> {
    type Output = ();

    fn with_transfer_remote(&self) -> Option<u16> {
        self.destination_chain_id
    }

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        // TODO: remove this when the smart contract side get signers on its own
        let signer_address = "0x71239e00ae942b394b3a91ab229e5264ad836f6f";

        let pact_msg = PactHyperlaneMessage::from((self.message.clone()));
        let pact_msg_str = serde_json::to_string(&pact_msg)
            .map_err(|e| KadenaClientError::OtherError(Box::new(e)))?;

        let verifier_msg = VerifierHyperlaneMessage::from((self.message.clone()));
        let verifier_msg_str = serde_json::to_string(&verifier_msg)
            .map_err(|e| KadenaClientError::OtherError(Box::new(e)))?;

        let pact_tm_str = BASE64_URL_SAFE_NO_PAD.encode(&self.message.body);
        let pact_recipient = String::from_utf8_lossy(&self.message.recipient.as_bytes());

        let pact_string = format!(
            "({}.{}.{} \"{}\" {})",
            self.contract.namespace(),
            self.contract.module_name(),
            Self::METHOD_NAME,
            BASE64_URL_SAFE_NO_PAD.encode(self.message.id().as_bytes()),
            pact_msg_str,
        );

        let verifier = VerifierDto {
            name: "hyperlane_v3_message".to_string(),
            proof: serde_json::json!([
                BASE64_URL_SAFE_NO_PAD.encode(RawHyperlaneMessage::from(&self.message).to_vec()),
                BASE64_URL_SAFE_NO_PAD.encode(&self.metadata),
            ]),
            capabilities: vec![serde_json::json!([
                format!(
                    "{}.{}.PROCESS-MLC",
                    self.contract.namespace(),
                    self.contract.module_name()
                ),
                format!(
                    "{}",
                    BASE64_URL_SAFE_NO_PAD.encode(self.message.id().as_bytes())
                ),
                verifier_msg_str,
                vec![signer_address],
                IntObject { int: 1 },
            ])],
        };
        info!("Pact string: {} with verfier {:?}", pact_string, verifier);

        self.contract
            .build_pact_tx_with_expr_and_verifiers(
                pact_string.as_str(),
                self.gas_limit,
                vec![verifier],
            )
            .await
            .map_err(|e| e.into())
    }

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        let res = self.local().await?.result()?;
        Ok(())
    }
}

pub struct RecipientIsmCall<'a> {
    contract: &'a IMailbox,
    gas_limit: Option<u64>,
}

impl RecipientIsmCall<'_> {
    const METHOD_NAME: &'static str = "recipient-ism";
    pub fn new(contract: &IMailbox) -> RecipientIsmCall {
        RecipientIsmCall {
            contract,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for RecipientIsmCall<'_> {
    type Output = H256;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.{}.{})",
                    self.contract.namespace(),
                    self.contract.module_name(),
                    Self::METHOD_NAME,
                ),
                self.gas_limit,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        let ism_obj = self.local().await?.result()?;

        let ism_namespace = ism_obj["refName"]["namespace"].as_str().ok_or_else(|| {
            KadenaClientError::DeserializationError(serde_json::Error::custom(
                "ISM namespace is missing",
            ))
        })?;
        let ism_name = ism_obj["refName"]["name"].as_str().ok_or_else(|| {
            KadenaClientError::DeserializationError(serde_json::Error::custom(
                "ISM name is missing",
            ))
        })?;

        let ism = format!("{}.{}", ism_namespace, ism_name);

        let mut ism_bytes: [u8; 32] = [0; 32];
        let bytes_to_copy = std::cmp::min(ism.as_bytes().len(), 32);
        ism_bytes[..bytes_to_copy].copy_from_slice(&ism.as_bytes()[..bytes_to_copy]);

        Ok(H256::from(ism_bytes))
    }
}

#[derive(Clone, Debug)]
pub struct IMailbox {
    provider: Arc<KadenaProvider>,
}

impl IMailbox {
    const MODULE_NAME: &'static str = "mailbox";

    /// Chain ID of the local chain. TODO: consider moving it to a better place
    const LOCAL_CHAIN_ID: u16 = 0;

    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self { provider }
    }

    pub fn delivered(&self, message_id: [u8; 32]) -> DeliveredCall {
        DeliveredCall::new(self, message_id)
    }

    pub fn nonce(&self) -> NonceCall {
        NonceCall::new(self)
    }

    pub async fn process(
        &self,
        metadata: Vec<u8>,
        message: HyperlaneMessage,
    ) -> Result<ProcessCall, KadenaClientError> {
        let mut pact_tm: Value = self
            .decode_token_message(message.body.clone())
            .local()
            .await?
            .result()?;

        // TODO: remove this when the smart contract side returns correct type formats

        // Convert amount from u64 to f64 as it's expected in the smart contract
        match pact_tm["amount"] {
            Value::Number(ref num) => {
                if num.is_u64() {
                    let amount = num.as_u64().unwrap();
                    pact_tm["amount"] = json!(amount as f64);
                } else if num.is_i64() {
                    return Err(KadenaClientError::DeserializationError(
                        serde_json::Error::custom("Amount is negative"),
                    ));
                }
            }
            _ => {
                return Err(KadenaClientError::DeserializationError(
                    serde_json::Error::custom("Failed to parse amount as u64"),
                ));
            }
        };

        // Convert chainId from int Object to int as it's expected in the smart contract
        let chain_id = pact_tm["chainId"]
            .as_object()
            .and_then(|map| map.get("int").and_then(Value::as_u64))
            .ok_or_else(|| {
                KadenaClientError::DeserializationError(serde_json::Error::custom(
                    "Failed to parse chainId as u64",
                ))
            })?;

        pact_tm["chainId"] = json!(chain_id);

        // Convert recipient from Object to String as it's expected in the smart contract
        let recipient_obj = pact_tm["recipient"].clone();
        let pact_tm_recipient = recipient_obj.to_string();
        pact_tm["recipient"] = json!(pact_tm_recipient);

        let mut call = ProcessCall::new(self, metadata, message, pact_tm);
        if chain_id != Self::LOCAL_CHAIN_ID as u64 {
            call.set_destination_chain_id(Some(chain_id as u16));
        }
        Ok(call)
    }

    pub fn recipient_ism(&self) -> RecipientIsmCall {
        RecipientIsmCall::new(self)
    }

    pub fn decode_token_message(&self, token_message: Vec<u8>) -> DecodeTokenMessageCall {
        DecodeTokenMessageCall::new(self, token_message)
    }

    pub fn dispatch_event(&self) -> DispatchEvent {
        DispatchEvent::new(self)
    }

    pub fn dispatch_id_event(&self) -> DispatchIdEvent {
        DispatchIdEvent::new(self)
    }

    pub fn process_event(&self) -> ProcessEvent {
        ProcessEvent::new(self)
    }

    pub fn process_id_event(&self) -> ProcessIdEvent {
        ProcessIdEvent::new(self)
    }
}

impl Contract for IMailbox {
    fn module_name(&self) -> &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}
