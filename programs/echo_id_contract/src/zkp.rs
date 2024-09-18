use anchor_lang::prelude::*;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
use rand_core::OsRng;
use sha2::{Sha256, Digest};

pub use curve25519_dalek::ristretto::CompressedRistretto as CompressedRistrettoPoint;

pub struct PublicKey(pub RistrettoPoint);
pub struct SecretKey(pub Scalar);

pub struct Proof {
    pub r: CompressedRistrettoPoint,
    pub s: Scalar,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SerializableProof {
    pub r: [u8; 32],
    pub s: [u8; 32],
}

impl SerializableProof {
    pub fn into_proof(&self) -> Option<Proof> {
        CompressedRistretto::from_slice(&self.r)
            .decompress()
            .map(|_r_point| Proof {
                r: CompressedRistretto::from_slice(&self.r),
                s: Scalar::from_bytes_mod_order(self.s),
            })
    }
}

impl From<Proof> for SerializableProof {
    fn from(proof: Proof) -> Self {
        SerializableProof {
            r: proof.r.to_bytes(),
            s: proof.s.to_bytes(),
        }
    }
}

impl PublicKey {
    pub fn new(secret_key: &SecretKey) -> Self {
        PublicKey(RISTRETTO_BASEPOINT_POINT * secret_key.0)
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Option<Self> {
        CompressedRistretto::from_slice(bytes)
            .decompress()
            .map(PublicKey)
    }
}

pub fn generate_keypair() -> (PublicKey, SecretKey) {
    let mut csprng = OsRng;
    let secret = SecretKey(Scalar::random(&mut csprng));
    let public = PublicKey::new(&secret);
    (public, secret)
}

pub fn prove(secret_key: &SecretKey, message: &[u8]) -> Proof {
    let mut csprng = OsRng;
    let k = Scalar::random(&mut csprng);
    let r = RISTRETTO_BASEPOINT_POINT * k;
    
    let mut hasher = Sha256::new();
    hasher.update(r.compress().as_bytes());
    hasher.update(message);
    let hash = hasher.finalize();
    let e = Scalar::from_bytes_mod_order(hash.into());
    
    let s = k - e * secret_key.0;
    
    Proof { r: r.compress(), s }
}

pub fn verify(public_key: &PublicKey, message: &[u8], proof: &Proof) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(proof.r.as_bytes());
    hasher.update(message);
    let hash = hasher.finalize();
    let e = Scalar::from_bytes_mod_order(hash.into());
    
    let rv = RISTRETTO_BASEPOINT_POINT * proof.s + public_key.0 * e;
    
    rv.compress() == proof.r
}


// Helper function to convert bytes to Scalar
pub fn bytes_to_scalar(bytes: &[u8; 32]) -> Scalar {
    Scalar::from_bytes_mod_order(*bytes)
}

// Helper function to convert Scalar to bytes
pub fn scalar_to_bytes(scalar: &Scalar) -> [u8; 32] {
    scalar.to_bytes()
}