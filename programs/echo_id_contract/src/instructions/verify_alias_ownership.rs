use anchor_lang::prelude::*;
use crate::{
    error::EchoIDError as ErrorCode,
    state::{AliasAccount, ProjectSuffix, ChainMapping},
    zkp,
    merkle,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterAliasParams {
    pub username: String,
    pub project_suffix: String,
    pub zk_public_key: [u8; 32],
    pub initial_chain_mapping: ChainMapping,
}

#[derive(Accounts)]
#[instruction(params: RegisterAliasParams)]
pub struct RegisterAlias<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + // discriminator
                32 + // owner
                4 + params.username.len() + // username
                4 + params.project_suffix.len() + // project_suffix
                32 + // chain_mappings_root
                4 + // chain_mapping_count
                8 + // reputation
                8 + // reputation_updated_at
                32, // zk_public_key
        seeds = [params.username.as_bytes(), b"@", params.project_suffix.as_bytes()],
        bump,
    )]
    pub alias_account: Account<'info, AliasAccount>,
    #[account(
        seeds = [b"project_suffix", params.project_suffix.as_bytes()],
        bump,
        constraint = project_suffix_account.suffix == params.project_suffix @ ErrorCode::InvalidProjectSuffix
    )]
    pub project_suffix_account: Account<'info, ProjectSuffix>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterAlias>, params: RegisterAliasParams) -> Result<()> {
    require!(!params.username.contains('@'), ErrorCode::InvalidUsername);
    
    let alias_account = &mut ctx.accounts.alias_account;
    alias_account.owner = *ctx.accounts.owner.key;
    alias_account.username = params.username;
    alias_account.project_suffix = params.project_suffix;
    
    // Verify that the provided public key is valid
    zkp::PublicKey::from_bytes(&params.zk_public_key)
        .ok_or(ErrorCode::InvalidPublicKey)?;
    
    alias_account.zk_public_key = params.zk_public_key;
    
    // Hash the initial chain mapping to create the root
    let leaf = merkle::hash_chain_mapping(&params.initial_chain_mapping);
    alias_account.chain_mappings_root = merkle::compute_merkle_root(&[leaf]);
    alias_account.chain_mapping_count = 1;
    
    // Initialize reputation
    alias_account.reputation = 10;
    alias_account.reputation_updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}