#![allow(clippy::enum_variant_names)]
#![allow(missing_docs)]

use std::sync::Arc;

use async_trait::async_trait;

use hyperlane_core::{
    Announcement, ChainResult, HyperlaneChain, HyperlaneContract,
    HyperlaneDomain, HyperlaneProvider, SignedType, TxOutcome, ValidatorAnnounce, H160, H256, U256, ChainCommunicationError, H512,
};
use tracing::warn;
use tracing::instrument;

use kadena_client::signers::Signer;
use crate::ConnectionConf;
use crate::contracts::i_validator_announce::AnnounceCall;
use crate::{
    contracts::i_validator_announce::IValidatorAnnounce,
    KadenaProvider,
};

use kadena_client::contract::Contract;
use kadena_client::contract_call::ContractCall;
use kadena_client::tx::{fill_tx_gas_params, report_tx};

/// A reference to a ValidatorAnnounce contract on some Kadena chain
#[derive(Debug)]
pub struct KadenaValidatorAnnounce {
    contract: Arc<IValidatorAnnounce>,
    domain: HyperlaneDomain,
}

impl KadenaValidatorAnnounce {
    /// Create a reference to a ValidatoAnnounce contract
    pub fn new(conf: &ConnectionConf, domain: &HyperlaneDomain, signer: Arc<dyn Signer>) -> Self {
        let (api_conf, proxy_conf) = conf.into();

        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(api_conf),
            Arc::new(proxy_conf),
            signer.clone(),
        ));

        Self {
            contract: Arc::new(IValidatorAnnounce::new(
                provider.clone(),
            )),
            domain: domain.clone(),
        }
    }

    /// Returns a ContractCall that processes the provided message.
    /// If the provided tx_gas_limit is None, gas estimation occurs.
    async fn announce_contract_call(
        &self,
        announcement: SignedType<Announcement>,
        tx_gas_limit: Option<U256>,
    ) -> ChainResult<AnnounceCall> {
        let serialized_signature: [u8; 65] = announcement.signature.into();
        let tx = self.contract.announce(
            announcement.value.validator.into(),
            announcement.value.storage_location,
            serialized_signature.into(),
        );

        let tx_gas_limit_u64_op: Option<u64> = tx_gas_limit.and_then(|value| {
            if value > U256::from(u64::MAX) {
                warn!(%value, "tx_gas_limit is too large to fit into a u64");
                None
            } else {
                Some(value.low_u64())
            }
        });

        fill_tx_gas_params(tx, tx_gas_limit_u64_op)
            .await
            .map_err(|_| ChainCommunicationError::from_other_str("Error while filling tx gas params"))
    }
}

impl HyperlaneChain for KadenaValidatorAnnounce {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }

    fn provider(&self) -> Box<dyn HyperlaneProvider> {
        Box::new(KadenaProvider::new(
            self.domain.clone(),
            self.contract.provider().connection_conf().clone(),
            self.contract.provider().kadena_proxy_config().clone(),
            self.contract.provider().signer().clone()
        ))
    }
}

impl HyperlaneContract for KadenaValidatorAnnounce {
    fn address(&self) -> H256 {
        self.contract.address().into()
    }
}

#[async_trait]
impl ValidatorAnnounce for KadenaValidatorAnnounce {
    async fn get_announced_storage_locations(
        &self,
        validators: &[H256],
    ) -> ChainResult<Vec<Vec<String>>> {
        #[derive(serde::Deserialize, Debug, Default)]
        struct StorageLocationsJson {
            storage_locations: Vec<Vec<String>>,
        }

        let storage_locations = self
            .contract
            .get_announced_storage_locations(
                validators.iter().map(|v| H160::from(*v).into()).collect(),
            )
            .local()
            .await
            .map_err(|_| ChainCommunicationError::from_other_str("Error returned while doing local"))?
            .result()
            .unwrap_or_default(); // TODO: should be removed when the smart contract side is fixed
            // TODO: handle error when the smart contract side is fixed
            //.map_err(|_| ChainCommunicationError::from_other_str("Error returned while calling get_announced_storage_locations"))?;

        let locations: StorageLocationsJson = serde_json::from_value(storage_locations).unwrap_or_default();
            // TODO: handle error when the smart contract side is fixed
            //.map_err(|_| ChainCommunicationError::from_other_str("Error returned while parsing storage_locations"))?;

        Ok(locations.storage_locations)
    }

    #[instrument(ret, skip(self))]
    async fn announce_tokens_needed(&self, _announcement: SignedType<Announcement>) -> Option<U256> {
        // TODO: implement when we have a way to query balance validator on Kadena
        // as of now, we assume there are enough tokens
        Some(U256::zero())

        /* 
        let validator = announcement.value.validator;
        let eth_h160: ethers::types::H160 = validator.into();

        let Ok(contract_call) = self.announce_contract_call(announcement, None).await else {
            trace!("Unable to get announce contract call");
            return None;
        };

        let Ok(balance) = self.provider.get_balance(eth_h160, None).await else {
            trace!("Unable to query balance");
            return None;
        };

        let Some(max_cost) = contract_call.tx.max_cost() else {
            trace!("Unable to get announce max cost");
            return None;
        };
        Some(max_cost.saturating_sub(balance).into())
        */
    }

    #[instrument(err, ret, skip(self))]
    async fn announce(
        &self,
        announcement: SignedType<Announcement>,
        tx_gas_limit: Option<U256>,
    ) -> ChainResult<TxOutcome> {
        let contract_call = self
            .announce_contract_call(announcement, tx_gas_limit)
            .await?;

        let receipt = report_tx(contract_call)
            .await
            .map_err(|_| ChainCommunicationError::from_other_str("Error returned while calling process"))?;

        let (_req_key, res) = receipt.into_iter().next().ok_or(ChainCommunicationError::from_other_str("Error in getting receipt"))?;
        let tx_outcome = TxOutcome {
            transaction_id: H512::from_low_u64_be(res.tx_id.ok_or(ChainCommunicationError::from_other_str("Tx id is missing"))?),
            executed: true,
            gas_used: U256::from(res.gas),
            gas_price: U256::from(res.meta_data.and_then(|meta| meta.public_meta.map(|public_meta| public_meta.gas_price)).unwrap_or_default() as u128),
        }; 

        Ok(tx_outcome)
    }
}
