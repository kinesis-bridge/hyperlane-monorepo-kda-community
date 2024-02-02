#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PactExecDto {
    #[serde(rename = "pactId")]
    pub pact_id: String,
    #[serde(rename = "step")]
    pub step: f32,
    #[serde(rename = "stepCount")]
    pub step_count: f32,
    #[serde(rename = "executed", deserialize_with = "Option::deserialize")]
    pub executed: Option<bool>,
    #[serde(rename = "stepHasRollback")]
    pub step_has_rollback: bool,
    #[serde(rename = "continuation")]
    pub continuation: serde_json::Value,
    #[serde(rename = "yield", deserialize_with = "Option::deserialize")]
    pub r#yield: Option<serde_json::Value>,
}

impl PactExecDto {
    pub fn new(
        pact_id: String,
        step: f32,
        step_count: f32,
        executed: Option<bool>,
        step_has_rollback: bool,
        continuation: serde_json::Value,
        r#yield: Option<serde_json::Value>,
    ) -> PactExecDto {
        PactExecDto {
            pact_id,
            step,
            step_count,
            executed,
            step_has_rollback,
            continuation,
            r#yield,
        }
    }
}
