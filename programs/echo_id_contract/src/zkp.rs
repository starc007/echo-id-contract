use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
use rand::rngs::OsRng;
use rand::Rng;
use sha2::{Sha256, Digest};
use std::boxed::Box;

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
           .ok()
           .and_then(|compressed| compressed.decompress())
           .map(|_r_point| Proof {
               r: CompressedRistretto::from_slice(&self.r).unwrap(),
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
       sol_log(&format!("Attempting to create PublicKey from bytes: {:?}", bytes));
       CompressedRistretto::from_slice(bytes)
           .ok()
           .and_then(|compressed| compressed.decompress())
           .map(|point| {
               sol_log("Successfully decompressed public key");
               PublicKey(point)
           })
   }
}

pub fn generate_keypair() -> (PublicKey, SecretKey) {
   let mut csprng = OsRng;
   let scalar_bytes: [u8; 32] = csprng.gen();
   let secret = SecretKey(Scalar::from_bytes_mod_order(scalar_bytes));
   let public = PublicKey::new(&secret);
   (public, secret)
}

pub fn prove(secret_key: &SecretKey, message: &[u8]) -> Proof {
   let computation = Box::new(|| {
       let mut csprng = OsRng;
       let scalar_bytes: [u8; 32] = csprng.gen();
       let k = Scalar::from_bytes_mod_order(scalar_bytes);
       let r = RISTRETTO_BASEPOINT_POINT * k;
       
       let mut hasher = Sha256::new();
       hasher.update(r.compress().as_bytes());
       hasher.update(message);
       let hash = hasher.finalize();
       let e = Scalar::from_bytes_mod_order(hash.into());
       
       let s = k - e * secret_key.0;
       
       Proof { r: r.compress(), s }
   });
   
   computation()
}

pub fn verify(public_key: &PublicKey, message: &[u8], proof: &Proof) -> bool {
   let computation = Box::new(|| {
       let mut hasher = Sha256::new();
       hasher.update(proof.r.as_bytes());
       hasher.update(message);
       let hash = hasher.finalize();
       let e = Scalar::from_bytes_mod_order(hash.into());
       
       let rv = RISTRETTO_BASEPOINT_POINT * proof.s + public_key.0 * e;
       
       rv.compress() == proof.r
   });
   
   computation()
}

pub fn bytes_to_scalar(bytes: &[u8; 32]) -> Scalar {
   Scalar::from_bytes_mod_order(*bytes)
}

pub fn scalar_to_bytes(scalar: &Scalar) -> [u8; 32] {
   scalar.to_bytes()
}