use ed25519_dalek::{
    Signature,
    VerifyingKey,
};

use crate::{
    error::KadenaClientError,
    models::CommandDto,
};
use async_trait::async_trait;

#[async_trait]
pub trait Signer: std::fmt::Debug + Send + Sync {
    /// Sign a transaction and return the signature
    async fn sign_transaction(&self, tx: &mut CommandDto) -> Result<Signature, KadenaClientError>;

    /// Get the public key of the signer
    fn pubkey(&self) -> VerifyingKey;
}
