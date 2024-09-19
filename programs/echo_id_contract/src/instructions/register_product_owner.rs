use anchor_lang::prelude::*;
use crate::{state::*, error::EchoIDError as ErrorCode};

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct RegisterProductOwner<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin"],
        bump,
        constraint = admin_config.admin == admin.key() @ ErrorCode::Unauthorized
    )]
    pub admin_config: Account<'info, AdminConfig>,
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1 + 4 + suffix.len(),
        seeds = [b"product_owner", new_product_owner.key().as_ref()],
        bump
    )]
    pub product_owner: Account<'info, ProductOwner>,
    /// CHECK: This is the new product owner being added
    pub new_product_owner: AccountInfo<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1 + 4 + suffix.len(),
        seeds = [b"suffix", suffix.as_bytes()],
        bump
    )]
    pub suffix_account: Account<'info, ProductOwner>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterProductOwner>, suffix: String) -> Result<()> {
    let product_owner = &mut ctx.accounts.product_owner;
    product_owner.address = *ctx.accounts.new_product_owner.key;
    product_owner.is_active = true;
    product_owner.suffix = suffix.clone();

    let suffix_account = &mut ctx.accounts.suffix_account;
    suffix_account.address = *ctx.accounts.new_product_owner.key;
    suffix_account.is_active = true;
    suffix_account.suffix = suffix;

    Ok(())
}