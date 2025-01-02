#![allow(clippy::enum_variant_names)]
#![allow(missing_docs)]

use std::num::NonZeroU64;
use std::ops::RangeInclusive;
use std::sync::Arc;

use crate::contracts::i_multisig_ism::IMultisigIsm;
use crate::ConnectionConf;
use async_trait::async_trait;
use hyperlane_core::H512;
use kadena_client::event::Event;
use kadena_client::signers::Signer;
use tracing::{info, instrument, warn};

use hyperlane_core::{
    utils::bytes_to_hex, ChainCommunicationError, ChainResult, HyperlaneChain, HyperlaneContract,
    HyperlaneDomain, HyperlaneMessage, HyperlaneProvider, Indexer, LogMeta, Mailbox,
    SequenceIndexer, TxCostEstimate, TxOutcome, H256, U256,
};

use crate::contracts::i_mailbox::{IMailbox, ProcessCall};
use crate::KadenaProvider;
use kadena_client::contract::{self, Contract, KadenaProxyProvider};
use kadena_client::contract_call::ContractCall;
use kadena_client::tx::{fill_tx_gas_params, report_tx};

#[derive(Debug, Clone)]
/// Struct that retrieves event data for an Kadena mailbox
pub struct KadenaMailboxIndexer {
    contract: Arc<IMailbox>,
    provider: Arc<KadenaProvider>,
    reorg_period: u32,
}

impl KadenaMailboxIndexer {
    /// Create new KadenaMailboxIndexer
    pub fn new(
        conf: &ConnectionConf,
        domain: &HyperlaneDomain,
        signer: Arc<dyn Signer>,
        reorg_period: u32,
    ) -> Self {
        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(conf.into()),
            signer.clone(),
        ));

        let contract = Arc::new(IMailbox::new(provider.clone()));
        Self {
            contract,
            provider,
            reorg_period,
        }
    }

    #[instrument(level = "debug", err, ret, skip(self))]
    async fn get_finalized_block_number(&self) -> ChainResult<u32> {
        Ok((self
            .provider
            .get_block_number()
            .await
            .map_err(ChainCommunicationError::from_other)? as u32)
            .saturating_sub(self.reorg_period))
    }
}

#[async_trait]
impl Indexer<HyperlaneMessage> for KadenaMailboxIndexer {
    async fn get_finalized_block_number(&self) -> ChainResult<u32> {
        self.get_finalized_block_number().await
    }

    #[instrument(err, skip(self))]
    async fn fetch_logs(
        &self,
        range: RangeInclusive<u32>,
    ) -> ChainResult<Vec<(HyperlaneMessage, LogMeta)>> {
        let mut events: Vec<(HyperlaneMessage, LogMeta)> = self
            .contract
            .dispatch_event()
            .query_events_range(range)
            .await
            .map_err(ChainCommunicationError::from_other)?
            .into_iter()
            .map(|event| (event.message, event.log.into()))
            .collect();

        events.sort_by(|a, b| a.0.nonce.cmp(&b.0.nonce));
        Ok(events)
    }
}

#[async_trait]
impl SequenceIndexer<HyperlaneMessage> for KadenaMailboxIndexer {
    #[instrument(err, skip(self))]
    async fn sequence_and_tip(&self) -> ChainResult<(Option<u32>, u32)> {
        let tip = Indexer::<HyperlaneMessage>::get_finalized_block_number(self).await?;

        let rewind_depth = if self.reorg_period > 0 {
            Some(self.reorg_period.into())
        } else {
            None
        };

        let sequence = self
            .contract
            .nonce()
            .local_typed_with_rewind_depth(rewind_depth)
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok((Some(sequence), tip))
    }
}

#[async_trait]
impl Indexer<H256> for KadenaMailboxIndexer {
    async fn get_finalized_block_number(&self) -> ChainResult<u32> {
        self.get_finalized_block_number().await
    }

    #[instrument(err, skip(self))]
    async fn fetch_logs(&self, range: RangeInclusive<u32>) -> ChainResult<Vec<(H256, LogMeta)>> {
        Ok(self
            .contract
            .process_id_event()
            .query_events_range(range)
            .await
            .map_err(ChainCommunicationError::from_other)?
            .into_iter()
            .map(|event| (H256::from(event.id), event.log))
            .collect())
    }
}

#[async_trait]
impl SequenceIndexer<H256> for KadenaMailboxIndexer {
    async fn sequence_and_tip(&self) -> ChainResult<(Option<u32>, u32)> {
        // A blanket implementation for this trait is fine for the EVM.
        // TODO: Consider removing `Indexer` as a supertrait of `SequenceIndexer`
        let tip = Indexer::<H256>::get_finalized_block_number(self).await?;
        Ok((None, tip))
    }
}

/// A reference to a Mailbox contract on some Kadena chain
#[derive(Debug)]
pub struct KadenaMailbox {
    contract: Arc<IMailbox>,
    domain: HyperlaneDomain,
    provider: Arc<KadenaProvider>,
}

impl KadenaMailbox {
    /// Create a reference to a Kadena mailbox
    pub fn new(conf: &ConnectionConf, domain: &HyperlaneDomain, signer: Arc<dyn Signer>) -> Self {
        let provider = Arc::new(KadenaProvider::new(
            domain.clone(),
            Arc::new(conf.into()),
            signer.clone(),
        ));
        Self {
            contract: Arc::new(IMailbox::new(provider.clone())),
            domain: domain.clone(),
            provider,
        }
    }

    /// Returns a ContractCall that processes the provided message.
    /// If the provided tx_gas_limit is None, gas estimation occurs.
    async fn process_contract_call(
        &self,
        message: &HyperlaneMessage,
        metadata: &[u8],
        tx_gas_limit: Option<U256>,
    ) -> ChainResult<ProcessCall> {
        // Get the validators and threshold from the multisig ISM contract.
        // It's a necessary workaround required by Kadena limitations (SPI and Verifiers).
        let multisig_ism_contract = IMultisigIsm::new(self.provider.clone());

        let validators_and_threshold = multisig_ism_contract
            .validators_and_threshold(message)
            .local_typed()
            .await
            .map_err(ChainCommunicationError::from_other)?;

        let tx = self
            .contract
            .process(metadata.to_vec(), message.clone(), validators_and_threshold)
            .await
            .map_err(ChainCommunicationError::from_other)?;
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
            .map_err(ChainCommunicationError::from_other)
    }
}

impl HyperlaneChain for KadenaMailbox {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }

    fn provider(&self) -> Box<dyn HyperlaneProvider> {
        Box::new(KadenaProvider::new(
            self.domain.clone(),
            self.contract.provider().proxy_client().clone(),
            self.contract.provider().signer().clone(),
        ))
    }
}

impl HyperlaneContract for KadenaMailbox {
    fn address(&self) -> H256 {
        self.contract.address().into()
    }
}

#[async_trait]
impl Mailbox for KadenaMailbox {
    #[instrument(skip(self))]
    async fn count(&self, maybe_lag: Option<NonZeroU64>) -> ChainResult<u32> {
        let rewind_depth = maybe_lag.map(|lag| lag.get());

        let nonce = self
            .contract
            .nonce()
            .local_typed_with_rewind_depth(rewind_depth)
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok(nonce)
    }

    #[instrument(skip(self))]
    async fn delivered(&self, id: H256) -> ChainResult<bool> {
        let delivered = self
            .contract
            .delivered(id.into())
            .local_typed()
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok(delivered)
    }

    #[instrument(skip(self))]
    async fn default_ism(&self) -> ChainResult<H256> {
        self.recipient_ism(H256::zero()).await
    }

    #[instrument(skip(self))]
    async fn recipient_ism(&self, _recipient: H256) -> ChainResult<H256> {
        let recipient_ism = self
            .contract
            .recipient_ism()
            .local_typed()
            .await
            .map_err(ChainCommunicationError::from_other)?;

        Ok(recipient_ism)
    }

    #[instrument(skip(self), fields(metadata=%bytes_to_hex(metadata)))]
    async fn process(
        &self,
        message: &HyperlaneMessage,
        metadata: &[u8],
        tx_gas_limit: Option<U256>,
    ) -> ChainResult<TxOutcome> {
        let contract_call = self
            .process_contract_call(message, metadata, tx_gas_limit)
            .await?;
        let receipt = report_tx(&contract_call)
            .await
            .map_err(ChainCommunicationError::from_other)?;

        info!("Got receipt: {:?}", receipt);

        let (_req_key, res) =
            receipt
                .into_iter()
                .next()
                .ok_or(ChainCommunicationError::from_other_str(
                    "Error in getting receipt",
                ))?;

        // Check if the transaction was successful.
        res.result().map_err(ChainCommunicationError::from_other)?;

        // Check if the transaction was cross-chain (between two Kadena chains).
        if let Some(dst_chain_id) = contract_call.with_transfer_remote() {
            if let Some(continuation) = res.continuation() {
                // If the transaction was cross-chain, we need to continue it.
                let pact_id = continuation.pact_id.clone();
                let step = continuation.step.saturating_add(1);
                let rollback = continuation.step_has_rollback;
                let cc_tx_res = self
                    .contract
                    .continue_transfer_remote(&pact_id, dst_chain_id, step as u8, rollback)
                    .await
                    .map_err(ChainCommunicationError::from_other)?;

                // Check if the continuation was successful.
                cc_tx_res
                    .result()
                    .map_err(ChainCommunicationError::from_other)?;
            } else {
                return Err(ChainCommunicationError::from_other_str(
                    "Cross-chain transaction did not return a continuation",
                ));
            }
        } else {
            if let Some(_) = res.continuation() {
                return Err(ChainCommunicationError::from_other_str(
                    "Local transaction returned a continuation",
                ));
            }
        }

        let tx_outcome = TxOutcome {
            transaction_id: H512::from_low_u64_be(
                res.tx_id
                    .ok_or(ChainCommunicationError::from_other_str("Tx id is missing"))?,
            ),
            executed: true,
            gas_used: U256::from(res.gas),
            gas_price: (res
                .meta_data
                .and_then(|meta| meta.public_meta.map(|public_meta| public_meta.gas_price))
                .unwrap_or_default() as u128)
                .into(),
        };

        Ok(tx_outcome)
    }

    #[instrument(skip(self), fields(msg=%message, metadata=%bytes_to_hex(metadata)))]
    async fn process_estimate_costs(
        &self,
        message: &HyperlaneMessage,
        metadata: &[u8],
    ) -> ChainResult<TxCostEstimate> {
        let contract_call = self.process_contract_call(message, metadata, None).await?;
        let gas_limit = contract_call
            .gas_limit()
            .unwrap_or(contract::DEFAULT_GAS_LIMIT);

        let gas_price = self.provider.get_gas_price();

        Ok(TxCostEstimate {
            gas_limit: gas_limit.into(),
            gas_price: gas_price.into(),
            l2_gas_limit: None,
        })
    }

    fn process_calldata(&self, _message: &HyperlaneMessage, _metadata: &[u8]) -> Vec<u8> {
        unimplemented!("Not required")
    }
}
