use anchor_lang::prelude::*;
use crate::{
    error::EchoIDError as ErrorCode,
    state::{AliasAccount, ChainMapping, ProductOwner},
    signature,
    merkle,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddChainMappingParams {
    pub new_mapping: ChainMapping,
    pub merkle_proof: Vec<[u8; 32]>,
    pub signature: [u8; 64],
}

#[derive(Accounts)]
#[instruction(params: AddChainMappingParams)]
pub struct AddChainMapping<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"product_owner", signer.key().as_ref()],
        bump,
        constraint = product_owner_account.address == signer.key() @ ErrorCode::Unauthorized,
        constraint = product_owner_account.is_active @ ErrorCode::ProductOwnerNotActive
    )]
    pub product_owner_account: Account<'info, ProductOwner>,
    #[account(
        mut,
        seeds = [alias_account.username.as_bytes(), b"@", alias_account.product_suffix.as_bytes()],
        bump,
        constraint = alias_account.owner == signer.key() @ ErrorCode::Unauthorized
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddChainMapping>, params: AddChainMappingParams) -> Result<()> {
    let alias_account = &mut ctx.accounts.alias_account;
    
    // Verify the signature
    let message = merkle::hash_chain_mapping(&params.new_mapping);
    require!(
        signature::verify_signature(&alias_account.public_key, &message, &params.signature),
        ErrorCode::InvalidSignature
    );
    
    // Verify the Merkle proof and compute new root
    let (is_valid, new_root) = merkle::verify_and_update(
        alias_account.chain_mappings_root,
        merkle::hash_chain_mapping(&params.new_mapping),
        &params.merkle_proof
    );
    require!(is_valid, ErrorCode::InvalidMerkleProof);

    // Update the alias account
    alias_account.chain_mappings_root = new_root;
    alias_account.chain_mapping_count += 1;

    Ok(())
}