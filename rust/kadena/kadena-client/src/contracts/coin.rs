use std::sync::Arc;

use async_trait::async_trait;

use crate::contract::{Contract, KadenaProxyProvider};
use crate::contract_call::ContractCall;
use crate::error::KadenaClientError;
use crate::models::CommandDto;
use crate::pact::PicoKda;

pub struct GetBalanceCall<'a> {
    contract: &'a CoinContract,
    account: String,
    gas_limit: Option<u64>,
}

impl GetBalanceCall<'_> {
    pub async fn new(contract: &CoinContract, account: String) -> GetBalanceCall {
        GetBalanceCall {
            contract,
            account,
            gas_limit: None,
        }
    }
}

#[async_trait]
impl ContractCall for GetBalanceCall<'_> {
    type Output = PicoKda;

    fn contract(&self) -> &dyn Contract {
        self.contract
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = Some(gas_limit);
    }

    async fn cmd(&self) -> Result<CommandDto, KadenaClientError> {
        self.contract
            .build_pact_tx_with_expr(
                &format!(
                    "({}.get-balance \"{}\")",
                    self.contract.module_name(),
                    self.account
                ),
                None,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn local_typed(&self) -> Result<Self::Output, KadenaClientError> {
        let rep_res = self.local().await?.result()?;
        serde_json::from_value(rep_res).map_err(|e| e.into())
    }
}

pub struct CoinContract {
    provider: Arc<dyn KadenaProxyProvider + Sync + Send>,
}

impl CoinContract {
    const MODULE_NAME: &'static str = "coin";

    pub fn new(provider: Arc<dyn KadenaProxyProvider + Sync + Send>) -> Self {
        Self { provider }
    }

    pub async fn get_balance(&self, account: String) -> Result<PicoKda, KadenaClientError> {
        let get_balanace_call = GetBalanceCall::new(self, account).await;
        let rep_res = get_balanace_call.local().await?.result()?;
        serde_json::from_value(rep_res).map_err(|e| e.into())
    }
}

impl Contract for CoinContract {
    fn module_name(&self) -> &'static str {
        Self::MODULE_NAME
    }

    fn provider(&self) -> Arc<dyn KadenaProxyProvider + Send + Sync> {
        self.provider.clone()
    }
}
