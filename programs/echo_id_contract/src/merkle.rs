use sha2::{Sha256, Digest};
use crate::state::ChainMapping;

pub fn hash_chain_mapping(mapping: &ChainMapping) -> [u8; 32] {
     let mut hasher = Sha256::new();
    hasher.update(mapping.chain_type.to_bytes());
    hasher.update(mapping.address.as_bytes());
    hasher.update(mapping.chain_id.to_le_bytes());
    hasher.finalize().into()
}

pub fn compute_merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return [0u8; 32];
    }
    if leaves.len() == 1 {
        return leaves[0];
    }
    let mut next_level = Vec::new();
    for chunk in leaves.chunks(2) {
        let mut hasher = Sha256::new();
        hasher.update(chunk[0]);
        if chunk.len() > 1 {
            hasher.update(chunk[1]);
        } else {
            hasher.update([0u8; 32]);
        }
        next_level.push(hasher.finalize().into());
    }
    compute_merkle_root(&next_level)
}

pub fn create_merkle_proof(leaves: &[[u8; 32]], index: usize) -> Vec<[u8; 32]> {
    let mut proof = Vec::new();
    let mut current_index = index;
    let mut current_level = leaves.to_vec();

    while current_level.len() > 1 {
        let sibling_index = if current_index % 2 == 0 { current_index + 1 } else { current_index - 1 };
        if sibling_index < current_level.len() {
            proof.push(current_level[sibling_index]);
        } else {
            proof.push([0u8; 32]);
        }

        let mut next_level = Vec::new();
        for chunk in current_level.chunks(2) {
            let mut hasher = Sha256::new();
            hasher.update(chunk[0]);
            if chunk.len() > 1 {
                hasher.update(chunk[1]);
            } else {
                hasher.update([0u8; 32]);
            }
            next_level.push(hasher.finalize().into());
        }

        current_index /= 2;
        current_level = next_level;
    }

    proof
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