use anchor_lang::prelude::*;
use crate::{
    error::EchoIDError as ErrorCode,
    state::*,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterAliasParams {
    pub username: String,
    pub suffix: String,
    pub chain_info: ChainInfo,
    pub metadata: AliasMetadata,
    pub user_address: Pubkey,
}

#[derive(Accounts)]
#[instruction(params: RegisterAliasParams)]
pub struct RegisterAlias<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This account is not written to or read from. It's just used for authorization.
    #[account(
        constraint = user.key() == params.user_address @ ErrorCode::InvalidUser
    )]
    pub user: AccountInfo<'info>,
    #[account(
        seeds = [b"suffix", params.suffix.as_bytes()],
        bump,
        constraint = suffix_account.is_active @ ErrorCode::InvalidProjectSuffix
    )]
    pub suffix_account: Account<'info, SuffixAccount>,
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + params.username.len() + 4 + params.suffix.len() + 8 + 4 + params.chain_info.name.len() + 4 + params.chain_info.address.len() + 8 + 8 + 4 + params.metadata.name.len() + 4 + params.metadata.image_url.len(),

        seeds = [params.username.as_bytes(), b"@", params.suffix.as_bytes(),params.chain_info.name.as_bytes()],
        bump,
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterAlias>, params: RegisterAliasParams) -> Result<()> {
    require!(!params.username.contains('@'), ErrorCode::InvalidUsername);
    
    let alias_account = &mut ctx.accounts.alias_account;
    
    alias_account.owner = params.user_address;
    alias_account.username = params.username;
    alias_account.product_suffix = params.suffix;

    
    alias_account.chain_info = params.chain_info;
    alias_account.metadata = params.metadata;
    
    alias_account.reputation = 10; // Initial reputation
    alias_account.reputation_updated_at = Clock::get()?.unix_timestamp;

    emit!(AliasRegistered {
        username: alias_account.username.clone(),
        suffix: alias_account.product_suffix.clone(),
        chain_info: alias_account.chain_info.clone(),
        metadata: alias_account.metadata.clone(),
        user_address: alias_account.owner,
    });

    Ok(())
}