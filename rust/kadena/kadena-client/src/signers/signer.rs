use ed25519_dalek::{VerifyingKey, Signature};

use crate::models::CommandDto;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Signer: std::fmt::Debug + Send + Sync {
    /// Sign a transaction and return the signature
    async fn sign_transaction(&self, tx: &mut CommandDto) -> Result<Signature>;

    /// Get the public key of the signer
    fn pubkey(&self) -> VerifyingKey;
}