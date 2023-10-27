#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockPayloadDto {
    #[serde(rename = "minerData")]
    pub miner_data: Box<crate::models::MinerDataDto>,
    #[serde(rename = "coinbase")]
    pub coinbase: Box<crate::models::CoinbaseDto>,
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::TransactionElementDto>,
    #[serde(rename = "payloadHash")]
    pub payload_hash: String,
    #[serde(rename = "transactionsHash")]
    pub transactions_hash: String,
    #[serde(rename = "outputsHash")]
    pub outputs_hash: String,
}

impl BlockPayloadDto {
    pub fn new(
        miner_data: crate::models::MinerDataDto,
        coinbase: crate::models::CoinbaseDto,
        transactions: Vec<crate::models::TransactionElementDto>,
        payload_hash: String,
        transactions_hash: String,
        outputs_hash: String,
    ) -> BlockPayloadDto {
        BlockPayloadDto {
            miner_data: Box::new(miner_data),
            coinbase: Box::new(coinbase),
            transactions,
            payload_hash,
            transactions_hash,
            outputs_hash,
        }
    }
}
