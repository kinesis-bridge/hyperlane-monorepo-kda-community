use crate::{error::KadenaClientError, models::CommandDto};
use async_trait::async_trait;
use base64::prelude::{Engine as _, BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, VerifyingKey};
use tracing::instrument;
use vaultrs::{client::VaultClient, transit::data};

pub struct VaultSigner {
    client: VaultClient,
    key_id: String,
    key_version: Option<u64>,
    mount: String,
    pubkey: VerifyingKey,
}

impl VaultSigner {
    const DEFAULT_TRANSIT_MOUNT: &str = "transit";

    #[instrument(err, skip(client, key_id, key_version, mount), fields(key_id = %key_id.as_ref()))]
    pub async fn new<K, M, V>(
        client: VaultClient,
        key_id: K,
        key_version: Option<V>,
        mount: Option<M>,
    ) -> Result<Self, KadenaClientError>
    where
        K: AsRef<str>,
        M: AsRef<str>,
        V: Into<u64>,
    {
        let pubkey = VerifyingKey::default();
        Ok(Self {
            client,
            key_id: key_id.as_ref().to_owned(),
            key_version: key_version.map(|v| v.into()),
            mount: mount
                .map(|m| m.as_ref().to_owned())
                .unwrap_or(Self::DEFAULT_TRANSIT_MOUNT.to_owned()),
            pubkey,
        })
    }
}

impl std::fmt::Debug for VaultSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VaultSigner")
            .field("address", &self.client.settings.address)
            .field("key_id", &self.key_id)
            .field("key_version", &self.key_version)
            .field("mount", &self.mount)
            .field("pubkey", &hex::encode(self.pubkey.to_bytes()))
            .finish()
    }
}

#[async_trait]
impl super::Signer for VaultSigner {
    #[instrument(err, skip(self))]
    async fn sign_transaction(&self, tx: &mut CommandDto) -> Result<Signature, KadenaClientError> {
        // Vault requires the hash to be base64 encoded with padding
        let hash_bin = BASE64_URL_SAFE_NO_PAD.decode(&tx.hash)?;
        let new_hash = BASE64_STANDARD.encode(&hash_bin);

        let resp = data::sign(&self.client, &self.mount, &self.key_id, &new_hash, None)
            .await
            .unwrap();

        // Since the vault response is in the following format: "vault:v1:signature",
        // we need to split the string and get the signature part
        // If the split fails, we just use the whole string
        let sig_str = resp
            .signature
            .rsplit_once(':')
            .map(|(_, v)| v)
            .unwrap_or(&resp.signature);

        let sig_bin = BASE64_STANDARD.decode(sig_str)?;
        let sig = hex::encode(&sig_bin);
        tx.sigs
            .push(crate::models::SignatureJsonDto::new(Some(sig)));

        Ok(Signature::from_slice(&sig_bin)?)
    }

    fn pubkey(&self) -> VerifyingKey {
        self.pubkey
    }
}
