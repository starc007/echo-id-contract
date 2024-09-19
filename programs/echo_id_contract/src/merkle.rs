use sha2::{Sha256, Digest};
use crate::state::ChainMapping;

pub fn hash_chain_mapping(mapping: &ChainMapping) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(mapping.chain_type.to_bytes());
    hasher.update(mapping.address.as_bytes());
    hasher.update(mapping.chain_id.to_le_bytes());
    hasher.finalize().into()
}

pub fn verify_and_update(root: [u8; 32], leaf: [u8; 32], proof: &[[u8; 32]]) -> (bool, [u8; 32]) {
    let mut current = leaf;
    for &sibling in proof {
        let mut hasher = Sha256::new();
        if current <= sibling {
            hasher.update(current);
            hasher.update(sibling);
        } else {
            hasher.update(sibling);
            hasher.update(current);
        }
        current = hasher.finalize().into();
    }
    (current == root, current)
}

pub fn compute_merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return [0u8; 32];
    }
    if leaves.len() == 1 {
        return leaves[0];
    }
    let mut hasher = Sha256::new();
    hasher.update(leaves[0]);
    hasher.update(leaves[1]);
    hasher.finalize().into()
}

pub fn verify_merkle_proof(root: [u8; 32], leaf: [u8; 32], proof: &[[u8; 32]]) -> bool {
    let mut current = leaf;
    for &sibling in proof {
        let mut hasher = Sha256::new();
        if current <= sibling {
            hasher.update(current);
            hasher.update(sibling);
        } else {
            hasher.update(sibling);
            hasher.update(current);
        }
        current = hasher.finalize().into();
    }
    current == root
}