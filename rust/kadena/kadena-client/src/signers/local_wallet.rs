use anyhow::Result;
use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::VerifyingKey;
use hex;
use tracing::instrument;

use crate::models::CommandDto;
use async_trait::async_trait;

#[derive(Debug)]
pub struct LocalWallet {
    signer: SigningKey,
}

impl LocalWallet {
    pub fn new(signer: SigningKey) -> Self {
        Self { signer }
    }
}

#[async_trait]
impl super::Signer for LocalWallet {
    #[instrument(err, skip(self))]
    async fn sign_transaction(&self, tx: &mut CommandDto) -> Result<Signature> {
        let hash_bin = BASE64_URL_SAFE_NO_PAD.decode(&tx.hash)?;
        let signature = self.signer.sign(&hash_bin);
        let sig_str = hex::encode(signature.to_bytes());

        tx.sigs
            .push(crate::models::SignatureJsonDto::new(Some(sig_str)));
        Ok(signature)
    }
    
    fn pubkey(&self) -> VerifyingKey {
        self.signer.verifying_key()
    }
}
