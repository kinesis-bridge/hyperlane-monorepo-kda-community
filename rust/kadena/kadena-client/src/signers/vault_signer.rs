use crate::{error::KadenaClientError, models::CommandDto};
use async_trait::async_trait;
use base64::prelude::{Engine as _, BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, VerifyingKey};
use tracing::instrument;
use vaultrs::{client::VaultClient, transit::data};

pub struct VaultSigner {
    client: VaultClient,
    key_id: String,
    mount: String,
    pubkey: VerifyingKey,
}

impl VaultSigner {
    const DEFAULT_TRANSIT_MOUNT: &str = "transit";

    #[instrument(err, skip(client, key_id, mount), fields(key_id = %key_id.as_ref()))]
    pub async fn new<K, M>(
        client: VaultClient,
        key_id: K,
        mount: Option<M>,
    ) -> Result<Self, KadenaClientError>
    where
        K: AsRef<str>,
        M: AsRef<str>,
    {
        let mount = mount
            .map(|m| m.as_ref().to_owned())
            .unwrap_or(Self::DEFAULT_TRANSIT_MOUNT.to_owned());
        let key_id = key_id.as_ref().to_owned();

        // Get the public key from the vault
        let read_key_rsp = vaultrs::transit::key::read(&client, &mount, &key_id)
            .await
            .map_err(KadenaClientError::from)?;

        let key_data = read_key_rsp.keys;

        let pubkey = match key_data {
            vaultrs::api::transit::responses::ReadKeyData::Asymmetric(keys) => {
                let (_, key_entry) = keys.iter().next().ok_or_else(|| {
                    KadenaClientError::VaultCustomError("No key found".to_string())
                })?;

                VerifyingKey::from_bytes(
                    BASE64_STANDARD
                        .decode(&key_entry.public_key)?
                        .as_slice()
                        .try_into()
                        .unwrap(),
                )?
            }
            _ => {
                return Err(KadenaClientError::VaultCustomError(
                    "Key is not an asymmetric key".to_string(),
                ))
            }
        };

        Ok(Self {
            client,
            key_id,
            mount,
            pubkey,
        })
    }
}

impl std::fmt::Debug for VaultSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VaultSigner")
            .field("address", &self.client.settings.address)
            .field("key_id", &self.key_id)
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
