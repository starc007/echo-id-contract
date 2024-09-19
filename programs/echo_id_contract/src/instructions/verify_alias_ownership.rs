use anchor_lang::prelude::*;
use crate::signature;
use crate::{
    error::EchoIDError as ErrorCode,
    state::{AliasAccount, ProductOwner},
};

#[derive(Accounts)]
pub struct VerifyAliasOwnership<'info> {
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
        seeds = [alias_account.username.as_bytes(), b"@", alias_account.product_suffix.as_bytes()],
        bump,
        constraint = alias_account.product_suffix == product_owner_account.suffix @ ErrorCode::Unauthorized
    )]
    pub alias_account: Account<'info, AliasAccount>,
}

pub fn handler(ctx: Context<VerifyAliasOwnership>, signature: [u8; 64]) -> Result<()> {
    let alias_account = &ctx.accounts.alias_account;
    

    // The message to be verified is the concatenation of the username and product suffix
    let message = [
        alias_account.username.as_bytes(),
        b"@",
        alias_account.product_suffix.as_bytes()
    ].concat();

    require!(
        signature::verify_signature(&alias_account.public_key, &message, &signature),
        ErrorCode::InvalidSignature
    );

    // If we've reached this point, the proof is valid
    msg!("Alias ownership verified successfully");

    Ok(())
}