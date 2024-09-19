use anchor_lang::prelude::*;
use crate::{state::*, error::EchoIDError};

#[derive(Accounts)]
pub struct AddProductOwner<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin"],
        bump,
        constraint = admin_config.admin == admin.key() @ EchoIDError::Unauthorized
    )]
    pub admin_config: Account<'info, AdminConfig>,
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1 + 4,
        seeds = [b"product_owner", new_product_owner.key().as_ref()],
        bump
    )]
    pub product_owner: Account<'info, ProductOwner>,
    /// CHECK: This is the new product owner being added
    pub new_product_owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddProductOwner>) -> Result<()> {
    let product_owner = &mut ctx.accounts.product_owner;
    product_owner.address = *ctx.accounts.new_product_owner.key;
    product_owner.is_active = true;
    product_owner.suffix = String::new();
    Ok(())
}