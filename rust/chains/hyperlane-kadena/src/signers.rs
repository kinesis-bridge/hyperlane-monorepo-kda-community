#![allow(missing_docs)]

use async_trait::async_trait;
use kadena_client::signers::{Signer, LocalWallet, VaultSigner};
use anyhow::Result;

#[derive(Debug)]
pub enum Signers {
    /// A wallet instantiated with a locally stored private key
    Local(LocalWallet),
    /// A signer using a key stored in hashicorp vault
    Vault(VaultSigner),
}

impl From<LocalWallet> for Signers {
    fn from(s: LocalWallet) -> Self {
        Signers::Local(s)
    }
}

impl From<VaultSigner> for Signers {
    fn from(s: VaultSigner) -> Self {
        Signers::Vault(s)
    }
}

#[async_trait]
impl Signer for Signers {
    async fn sign_transaction(&self, tx: &mut kadena_client::models::CommandDto) -> Result<ed25519_dalek::Signature> {
        match self {
            Signers::Local(s) => s.sign_transaction(tx).await,
            Signers::Vault(s) => s.sign_transaction(tx).await,
        }
    }

    fn pubkey(&self) -> ed25519_dalek::VerifyingKey {
        match self {
            Signers::Local(s) => s.pubkey(),
            Signers::Vault(s) => s.pubkey(),
        }
    }
}