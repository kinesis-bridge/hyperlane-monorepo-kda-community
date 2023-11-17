use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;
use hex;
use base64::{Engine as _, engine::general_purpose};
use anyhow::Result;

use crate::models::CommandDto;

pub fn sign_tx(tx: &mut CommandDto, key: &SigningKey) -> Result<Signature> {
    let hash_str = &tx.hash[..];
    let hash_bin = general_purpose::URL_SAFE_NO_PAD.decode(hash_str)?;
    let signature = key.sign(&hash_bin);
    let sig_str = hex::encode(signature.to_bytes());

    tx.sigs.push(crate::models::SignatureJsonDto::new(
        Some(sig_str),
    ));
    Ok(signature)
}
