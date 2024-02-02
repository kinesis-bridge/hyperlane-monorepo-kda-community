#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct TransactionElementDto {
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(rename = "transaction")]
    pub transaction: Box<crate::models::TransactionPayloadDto>,
    #[serde(rename = "output")]
    pub output: Box<crate::models::CoinbaseDto>,
}

impl TransactionElementDto {
    pub fn new(
        transaction: crate::models::TransactionPayloadDto,
        output: crate::models::CoinbaseDto,
    ) -> TransactionElementDto {
        TransactionElementDto {
            height: None,
            transaction: Box::new(transaction),
            output: Box::new(output),
        }
    }
}
