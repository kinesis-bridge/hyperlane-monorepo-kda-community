#[derive(Debug, Clone)]
pub struct ContractsConf {
    namespace: String,
}

impl ContractsConf {
    /// Creates a new configuration with the given namespace.
    pub fn new(namespace: String) -> Self {
        ContractsConf { namespace }
    }

    /// Returns the namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
}
