use crate::{
    client::KadenaProxyClient,
    error::KadenaClientError,
    models::{
        BuildPactTxDtoBuilder, CommandDto, CommandResultDto, ContinueTransferRemoteDto, VerifierDto,
    },
    signers::Signer,
};
use async_trait::async_trait;
use std::sync::Arc;

/* Since the proxy returns the height of chain 0, and not the considered chain, it's safe
   a couple of confirmation here to be sure the indexer doesn't miss blocks. Moreover, height can jump
   in case of a reorg.. So take a little bit of headroom */
const HEIGHT_CONFIRMATION_DEPTH: u64 = 3;

pub const DEFAULT_GAS_LIMIT: u64 = 150_000;

#[async_trait]
pub trait KadenaProxyProvider {
    /// Returns the proxy client for this provider.
    fn proxy_client(&self) -> Arc<KadenaProxyClient>;

    /// Returns the signer for this provider.
    fn signer(&self) -> Arc<dyn Signer>;

    // TODO: calculate gas price based on the current network conditions
    /// Returns the gas price for this provider.
    fn get_gas_price(&self) -> u64 {
        // 1e-12 is the smallest unit of KDA
        1_000_000 // 1e-6 KDA
    }

    /// Returns the block number for this provider with confirmation depth.
    async fn get_block_number(&self) -> Result<u64, KadenaClientError> {
        self.proxy_client().height(Some(HEIGHT_CONFIRMATION_DEPTH)).await
    }
}

#[async_trait]
pub trait Contract: Send + Sync {
    /// Returns the module name for this contract.
    fn module_name(&self) -> &'static str;

    // Since Pact smart contract don't have address, we use module name as address for Hyperlane
    // compatibility We may use hashed module name as address in the future
    /// Returns the address for this contract.
    fn address(&self) -> [u8; 32] {
        let module_name = self.module_name();
        let module_name_bytes = module_name.as_bytes();
        let mut address = [0; 32];
        address[..module_name_bytes.len()].copy_from_slice(module_name_bytes);
        address
    }

    /// Returns the namespace for this contract.
    fn namespace(&self) -> String {
        self.provider()
            .proxy_client()
            .contracts_conf()
            .namespace()
            .to_owned()
    }

    /// Returns the public key for this contract signer.
    fn pubkey(&self) -> String {
        self.provider().signer().pubkey_str()
    }

    /// Returns the account name of the signer.
    fn account_name(&self) -> String {
        let account_name_from_conf = self
            .provider()
            .proxy_client()
            .contracts_conf()
            .account_name();
        account_name_from_conf.unwrap_or(self.provider().signer().k_account())
    }

    /// Returns the provider for this contract.
    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync>;

    /// Build a transaction using the given expression.
    async fn build_pact_tx_with_expr(
        &self,
        expr: &str,
        gas_limit: Option<u64>,
    ) -> Result<CommandDto, KadenaClientError> {
        //build_pact_tx_with_expr_internal(self, expr, gas_limit, None).await
        build_pact_tx_with_expr_internal(self, expr, gas_limit, None).await
    }

    /// Build a transaction using the given expression.
    async fn build_pact_tx_with_expr_and_verifiers(
        &self,
        expr: &str,
        gas_limit: Option<u64>,
        verifiers: Vec<VerifierDto>,
    ) -> Result<CommandDto, KadenaClientError> {
        build_pact_tx_with_expr_internal(self, expr, gas_limit, Some(verifiers)).await
    }

    async fn continue_transfer_remote(
        &self,
        pact_id: &str,
        dst_chain_id: u16,
        step: u8,
        rollback: bool,
    ) -> Result<CommandResultDto, KadenaClientError> {
        let client = self.provider().proxy_client();
        let conf = client.chainweb_conf();
        let continue_body =
            ContinueTransferRemoteDto::new(conf, pact_id.to_owned(), dst_chain_id, step, rollback);
        client.continue_transfer_remote(continue_body).await
    }
}

// This is the helper function
async fn build_pact_tx_with_expr_internal<T: Contract + ?Sized + Send + Sync>(
    instance: &T,
    expr: &str,
    gas_limit: Option<u64>,
    verifiers: Option<Vec<VerifierDto>>,
) -> Result<CommandDto, KadenaClientError> {
    let provider = instance.provider();
    let client = provider.proxy_client();
    let conf = client.chainweb_conf();

    let mut tx_dto_builder = BuildPactTxDtoBuilder::new(
        &conf,
        expr.to_owned(),
        instance.pubkey(),
        instance.account_name(),
        gas_limit.unwrap_or(DEFAULT_GAS_LIMIT),
    );
    if let Some(verifiers) = verifiers {
        tx_dto_builder = tx_dto_builder.with_verifiers(verifiers);
    };
    client.build_tx(tx_dto_builder.build()).await
}
