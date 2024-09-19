use anchor_lang::prelude::*;
use crate::{state::*, error::EchoIDError as ErrorCode};

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct CreateSuffix<'info> {
    #[account(mut)]
    pub product_owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"product_owner", product_owner.key().as_ref()],
        bump,
        constraint = product_owner_account.address == product_owner.key() @ ErrorCode::Unauthorized,
        constraint = product_owner_account.is_active @ ErrorCode::ProductOwnerNotActive,
        constraint = product_owner_account.suffix.is_empty() @ ErrorCode::SuffixAlreadyExists
    )]
    pub product_owner_account: Account<'info, ProductOwner>,
    #[account(
        init,
        payer = product_owner,
        space = 8 + 4 + suffix.len(),
        seeds = [b"suffix", suffix.as_bytes()],
        bump
    )]
    pub suffix_account: Account<'info, ProductOwner>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSuffix>, suffix: String) -> Result<()> {
    let product_owner_account = &mut ctx.accounts.product_owner_account;
    product_owner_account.suffix = suffix.clone();
    
    let suffix_account = &mut ctx.accounts.suffix_account;
    suffix_account.address = *ctx.accounts.product_owner.key;
    suffix_account.is_active = true;
    suffix_account.suffix = suffix;
    
    Ok(())
}