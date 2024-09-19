use anchor_lang::prelude::*;
use crate::{
    error::EchoIDError as ErrorCode,
    state::{AliasAccount, ChainMapping, ProductOwner},
    zkp,
    merkle,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddChainMappingParams {
    pub new_mapping: ChainMapping,
    pub merkle_proof: Vec<[u8; 32]>,
    pub zk_proof: zkp::SerializableProof,
}

#[derive(Accounts)]
#[instruction(params: AddChainMappingParams)]
pub struct AddChainMapping<'info> {
    #[account(mut)]
    pub product_owner: Signer<'info>,
    #[account(
        seeds = [b"product_owner", product_owner.key().as_ref()],
        bump,
        constraint = product_owner_account.address == product_owner.key() @ ErrorCode::Unauthorized,
        constraint = product_owner_account.is_active @ ErrorCode::ProductOwnerNotActive
    )]
    pub product_owner_account: Account<'info, ProductOwner>,
    #[account(
        mut,
        seeds = [alias_account.username.as_bytes(), b"@", alias_account.product_suffix.as_bytes()],
        bump,
        constraint = alias_account.owner == product_owner.key() @ ErrorCode::Unauthorized
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddChainMapping>, params: AddChainMappingParams) -> Result<()> {
    let alias_account = &mut ctx.accounts.alias_account;
    
    // Verify the ZK proof
    let public_key = zkp::PublicKey::from_bytes(&alias_account.zk_public_key)
        .ok_or(ErrorCode::InvalidPublicKey)?;
    
    let message = merkle::hash_chain_mapping(&params.new_mapping);
    let proof = params.zk_proof.into_proof().ok_or(ErrorCode::InvalidProof)?;
    
    require!(
        zkp::verify(&public_key, &message, &proof),
        ErrorCode::InvalidProof
    );
    
    // Verify the Merkle proof
    let new_leaf = merkle::hash_chain_mapping(&params.new_mapping);
    let old_root = alias_account.chain_mappings_root;
    require!(
        merkle::verify_merkle_proof(old_root, new_leaf, &params.merkle_proof),
        ErrorCode::InvalidMerkleProof
    );
    
    // Compute the new Merkle root
    let mut leaves = vec![new_leaf];
    for &proof_element in params.merkle_proof.iter().rev() {
        leaves.push(proof_element);
    }
    let new_root = merkle::compute_merkle_root(&leaves);
    
    // Update the alias account
    alias_account.chain_mappings_root = new_root;
    alias_account.chain_mapping_count += 1;

    Ok(())
}