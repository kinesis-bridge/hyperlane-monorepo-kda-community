#[derive(Debug, Clone)]
pub struct ContractsConf {
    namespace: String,
    account_name: Option<String>,
}

impl ContractsConf {
    /// Creates a new configuration with the given namespace.
    pub fn new(namespace: String, account_name: Option<String>) -> Self {
        ContractsConf {
            namespace,
            account_name,
        }
    }

    /// Returns the namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the account name.
    pub fn account_name(&self) -> Option<String> {
        self.account_name.clone()
    }
}
