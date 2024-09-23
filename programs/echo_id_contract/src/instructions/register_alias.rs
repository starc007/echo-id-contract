use anchor_lang::prelude::*;
use crate::{
    error::EchoIDError as ErrorCode,
    state::*,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterAliasParams {
    pub username: String,
    pub suffix: String,
    pub initial_chain_mapping: ChainMapping,
}

#[derive(Accounts)]
#[instruction(params: RegisterAliasParams)]
pub struct RegisterAlias<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"suffix", params.suffix.as_bytes()],
        bump,
        constraint = suffix_account.is_active @ ErrorCode::InvalidProjectSuffix
    )]
    pub suffix_account: Account<'info, ProductOwner>,
    #[account(
        init,
        payer = user,
        space = 8 + // discriminator
                32 + // owner
                4 + params.username.len() + // username
                4 + params.suffix.len() + // product_suffix
                4 + // vec length for chain_mappings
                (1 + // chain type
                 4 + params.initial_chain_mapping.address.len() + // address
                 4) * 10 + // chain_id, assuming max 10 mappings
                8 + // reputation
                8, // reputation_updated_at
        seeds = [params.username.as_bytes(), b"@", params.suffix.as_bytes()],
        bump,
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterAlias>, params: RegisterAliasParams) -> Result<()> {
    require!(!params.username.contains('@'), ErrorCode::InvalidUsername);
    
    let alias_account = &mut ctx.accounts.alias_account;
    
    alias_account.owner = ctx.accounts.user.key();
    alias_account.username = params.username;
    alias_account.product_suffix = params.suffix;

    // Initialize with the first chain mapping
    alias_account.chain_mappings = vec![params.initial_chain_mapping];
    
    alias_account.reputation = 10; // Initial reputation
    alias_account.reputation_updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}