use std::sync::Arc;

use crate::{provider, KadenaProvider};

use kadena_client::{contract::{Contract, KadenaProxyProvider}, models::{CommandDto, EventDataDto}, contract_call::ContractCall, event::{Event, EventData}};
use anyhow::Result;
use async_trait::async_trait;

pub struct DispatchEventData {
    pub sender: [u8; 32],
    pub destination: u32,
    pub recipient: [u8; 32],
    pub message: Vec<u8>,
}

impl TryFrom<EventDataDto> for DispatchEventData {
    type Error = anyhow::Error;
    fn try_from(event_data_dto: EventDataDto) -> Result<Self> {
        let params = event_data_dto.params;

        // Assembling message since it can't be stored in Event due to verify-spv

        let version = params.get(0).ok_or(anyhow::anyhow!("Version is missing"))?;
        let version = TryInto::<u64>::try_into(version.clone())? as u8;
        
        let nonce = params.get(1).ok_or(anyhow::anyhow!("Nonce is missing"))?;
        let nonce = TryInto::<u64>::try_into(nonce.clone())? as u32;

        let origin = params.get(2).ok_or(anyhow::anyhow!("Origin is missing"))?;
        let origin = TryInto::<u64>::try_into(origin.clone())? as u32;

        let sender = params.get(3).ok_or(anyhow::anyhow!("Sender is missing"))?;
        let sender_vec = hex::decode(sender.to_string()).map_err(|_| anyhow::anyhow!("Invalid hex string"))?;
        let mut sender = [0u8; 32];
        sender.copy_from_slice(&sender_vec);
        
        let destination = params.get(4).ok_or(anyhow::anyhow!("Destination is missing"))?;
        let destination = TryInto::<u64>::try_into(destination.clone())? as u32;

        let recipient = params.get(5).ok_or(anyhow::anyhow!("Recipient is missing"))?;
        let recipient_vec = hex::decode(recipient.to_string()).map_err(|_| anyhow::anyhow!("Invalid hex string"))?;
        let mut recipient = [0u8; 32];
        recipient.copy_from_slice(&recipient_vec);

        let recipient_tm = params.get(6).ok_or(anyhow::anyhow!("Recipient TM is missing"))?;
        let recipient_tm_vec = hex::decode(recipient_tm.to_string()).map_err(|_| anyhow::anyhow!("Invalid hex string"))?;
        let mut recipient_tm = [0u8; 32];
        recipient_tm.copy_from_slice(&recipient_tm_vec);

        let amount = params.get(7).ok_or(anyhow::anyhow!("Amount is missing"))?;
        let amount = TryInto::<u64>::try_into(nonce.clone())?;

        let mut message = Vec::new();

        message.extend_from_slice(&version.to_be_bytes());
        message.extend_from_slice(&nonce.to_be_bytes());
        message.extend_from_slice(&origin.to_be_bytes());
        message.extend_from_slice(&sender);
        message.extend_from_slice(&destination.to_be_bytes());
        message.extend_from_slice(&recipient);
        message.extend_from_slice(&recipient_tm);
        message.extend_from_slice(&amount.to_be_bytes());

        Ok(Self {
            sender,
            destination,
            recipient,
            message,
        })
    }
}

impl EventData for DispatchEventData {}

pub struct DispatchEvent<'a> {
    contract: &'a IMailbox,
}

impl<'a> DispatchEvent<'a> {
    const EVENT_NAME: &'static str = "DISPATCH";

    pub fn new(
        contract: &'a IMailbox,
    ) -> Self {
        Self {
            contract,
        }
    }
}

impl Event for DispatchEvent<'_> {
    type DataType = DispatchEventData;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

pub struct DispatchIdEventData {
    id: String,
}

impl TryFrom<EventDataDto> for DispatchIdEventData {
    type Error = anyhow::Error;
    fn try_from(event_data_dto: EventDataDto) -> Result<Self> {
        let params = event_data_dto.params;
        let id = params.get(0).ok_or(anyhow::anyhow!("ID is missing"))?;

        Ok(Self {
            id: id.to_string(),
        })
    }
}

impl EventData for DispatchIdEventData {}

pub struct DispatchIdEvent<'a> {
    contract: &'a IMailbox,
}

impl<'a> DispatchIdEvent<'a> {
    const EVENT_NAME: &'static str = "DISPATCH-ID";

    pub fn new(
        contract: &'a IMailbox,
    ) -> Self {
        Self {
            contract,
        }
    }
}

impl Event for DispatchIdEvent<'_> {
    type DataType = DispatchIdEventData;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

pub struct ProcessEventData {
    origin: String,
    sender: String,
    recipient: String,
}

impl TryFrom<EventDataDto> for ProcessEventData {
    type Error = anyhow::Error;
    fn try_from(event_data_dto: EventDataDto) -> Result<Self> {
        let params = event_data_dto.params;
        let origin = params.get(0).ok_or(anyhow::anyhow!("Origin is missing"))?;
        let sender = params.get(1).ok_or(anyhow::anyhow!("Sender is missing"))?;
        let recipient = params.get(2).ok_or(anyhow::anyhow!("Recipient is missing"))?;

        Ok(Self {
            origin: origin.to_string(),
            sender: sender.to_string(),
            recipient: recipient.to_string(),
        })
    }
}

impl EventData for ProcessEventData {}

pub struct ProcessEvent<'a> {
    contract: &'a IMailbox,
}

impl<'a> ProcessEvent<'a> {
    const EVENT_NAME: &'static str = "PROCESS";

    pub fn new(
        contract: &'a IMailbox,
    ) -> Self {
        Self {
            contract,
        }
    }
}

impl Event for ProcessEvent<'_> {
    type DataType = ProcessEventData;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

pub struct ProcessIdEventData {
    pub id: [u8; 32],
}

impl TryFrom<EventDataDto> for ProcessIdEventData {
    type Error = anyhow::Error;
    fn try_from(event_data_dto: EventDataDto) -> Result<Self> {
        let params = event_data_dto.params;
        let id_str = params.get(0).ok_or(anyhow::anyhow!("ID is missing"))?;
        let id_vec = hex::decode(id_str.to_string()).map_err(|_| anyhow::anyhow!("Invalid hex string"))?;
        let mut id = [0; 32];
        id.copy_from_slice(&id_vec);

        Ok(Self {
            id,
        })
    }
}

impl EventData for ProcessIdEventData {}

pub struct ProcessIdEvent<'a> {
    contract: &'a IMailbox,
}

impl<'a> ProcessIdEvent<'a> {
    const EVENT_NAME: &'static str = "PROCESS-ID";

    pub fn new(
        contract: &'a IMailbox,
    ) -> Self {
        Self {
            contract,
        }
    }
}

impl Event for ProcessIdEvent<'_> {
    type DataType = ProcessIdEventData;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

pub struct DeliveredCall<'a> {
    contract: &'a IMailbox,
    message_id: [u8; 32],
    gas_limit: Option<u64>,
}

impl DeliveredCall<'_> {
    const METHOD_NAME: &'static str = "delivered";
    pub fn new(
        contract: &IMailbox,
        message_id: [u8; 32],
    ) -> DeliveredCall {
        DeliveredCall {
            contract,
            message_id,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for DeliveredCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto> {
        self.contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} \"{}\")",
                self.contract.namespace(),
                self.contract.module_name(),
                Self::METHOD_NAME,
                hex::encode(self.message_id),
            ),
            self.gas_limit,
        ).await.map_err(|e| e.into())
    }
}


pub struct NonceCall<'a> {
    contract: &'a IMailbox,
    gas_limit: Option<u64>,
}

impl NonceCall<'_> {
    const METHOD_NAME: &'static str = "nonce";
    pub fn new(
        contract: &IMailbox,
    ) -> NonceCall {
        NonceCall {
            contract,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for NonceCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto> {
        self.contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{})",
                self.contract.namespace(),
                self.contract.module_name(),
                Self::METHOD_NAME,
            ),
            self.gas_limit,
        ).await.map_err(|e| e.into())
    }
}


pub struct ProcessCall<'a> {
    contract: &'a IMailbox,
    metadata: Vec<u8>,
    message: Vec<u8>,
    gas_limit: Option<u64>,
}

impl ProcessCall<'_> {
    const METHOD_NAME: &'static str = "process";
    pub fn new(
        contract: &IMailbox,
        metadata: Vec<u8>,
        message: Vec<u8>,
    ) -> ProcessCall {
        ProcessCall {
            contract,
            metadata,
            message,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for ProcessCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto> {
        self.contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} \"{}\",\"{}\")",
                self.contract.namespace(),
                self.contract.module_name(),
                Self::METHOD_NAME,
                hex::encode(&self.metadata),
                hex::encode(&self.message),
            ),
            self.gas_limit,
        ).await.map_err(|e| e.into())
    }
}


pub struct RecipientIsmCall<'a> {
    contract: &'a IMailbox,
    recipient: String,
    gas_limit: Option<u64>,
}

impl RecipientIsmCall<'_> {
    const METHOD_NAME: &'static str = "recipient-ism";
    pub fn new(
        contract: &IMailbox,
        recipient: String,
    ) -> RecipientIsmCall {
        RecipientIsmCall {
            contract,
            recipient,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for RecipientIsmCall<'_> {
    fn contract(&self) -> &dyn Contract {
        self.contract
    }
    
    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    async fn cmd(&self) -> Result<CommandDto> {
        self.contract.build_pact_tx_with_expr(
            &format!(
                "({}.{}.{} \"{}\")",
                self.contract.namespace(),
                self.contract.module_name(),
                Self::METHOD_NAME,
                self.recipient,
            ),
            self.gas_limit,
        ).await.map_err(|e| e.into())
    }
}

#[derive(Clone, Debug)]
pub struct IMailbox {
    provider: Arc<KadenaProvider>,
}

impl IMailbox {
    const MODULE_NAME: &'static str = "mailbox";

    pub fn new(provider: Arc<KadenaProvider>) -> Self {
        Self {
            provider,
        }
    }

    pub fn delivered(&self, message_id: [u8; 32]) -> DeliveredCall {
        DeliveredCall::new(
            self,
            message_id,
        )
    }

    pub fn nonce(&self) -> NonceCall {
        NonceCall::new(
            self,
        )
    }

    pub fn process(&self, metadata: Vec<u8>, message: Vec<u8>) -> ProcessCall {
        ProcessCall::new(
            self,
            metadata,
            message,
        )
    }

    pub fn recipient_ism(&self, recipient: String) -> RecipientIsmCall {
        RecipientIsmCall::new(
            self,
            recipient,
        )
    }

    pub fn dispatch_event(&self) -> DispatchEvent {
        DispatchEvent::new(
            self,
        )
    }

    pub fn dispatch_id_event(&self) -> DispatchIdEvent {
        DispatchIdEvent::new(
            self,
        )
    }

    pub fn process_event(&self) -> ProcessEvent {
        ProcessEvent::new(
            self,
        )
    }

    pub fn process_id_event(&self) -> ProcessIdEvent {
        ProcessIdEvent::new(
            self,
        )
    }
}

impl Contract for IMailbox {
    fn module_name(&self) ->  &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) ->  Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}
