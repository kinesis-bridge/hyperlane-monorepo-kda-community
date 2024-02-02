use crate::{
    client::KadenaProxyClient,
    error::KadenaClientError,
    models::{
        BuildPactTxDto,
        CommandDto,
    },
    signers::Signer,
};
use async_trait::async_trait;
use std::sync::Arc;

const CONFIRMATION_DEPTH: u64 = 0;
pub const DEFAULT_GAS_LIMIT: u64 = 100_000;

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
        self.proxy_client().height(Some(CONFIRMATION_DEPTH)).await
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
    fn namespace(&self) -> &'static str {
        // TODO: we may want to remove this in a final version
        "free"
    }

    // TODO: consider moving it to a better place. Public key is not a part of the contract.
    /// Returns a string representation of the public key for this contract signer.
    fn pubkey(&self) -> String {
        // TODO: remove this in a final version
        "368820f80c324bbc7c2b0610688a7da43e39f91d118732671cd9c7500ff43cca".to_string()
    }

    // TODO: consider moving it to a better place. Account name is not a part of the contract.
    /// Returns the account name for this contract.
    fn account_name(&self) -> String {
        //format!("k:{}", self.pubkey())
        // TODO: remove this hardcode when the smart contract side supports k accounts for agents
        "sender00".to_string()
    }

    /// Returns the provider for this contract.
    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync>;

    /// Build a transaction using the given expression.
    async fn build_pact_tx_with_expr(
        &self,
        expr: &str,
        gas_limit: Option<u64>,
    ) -> Result<CommandDto, KadenaClientError> {
        let provider = self.provider();
        let client = provider.proxy_client();
        let conf = client.chainweb_conf();
        let build_tx_dto = BuildPactTxDto::new(
            conf.url.to_string(),
            conf.network_id.clone(),
            conf.chain_id as u32,
            expr.to_owned(),
            self.pubkey(),
            self.account_name(),
            gas_limit.unwrap_or(DEFAULT_GAS_LIMIT),
        );
        client.build_tx(build_tx_dto).await
    }
}
