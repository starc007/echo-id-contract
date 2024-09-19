use ed25519_dalek::{Verifier, Signature, PublicKey};

pub fn verify_signature(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    PublicKey::from_bytes(public_key)
        .and_then(|pk| Signature::from_bytes(signature).map(|sig| (pk, sig)))
        .map_or(false, |(pk, sig)| pk.verify(message, &sig).is_ok())
}