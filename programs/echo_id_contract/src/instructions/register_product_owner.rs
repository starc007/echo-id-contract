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
        seeds = [b"suffix", suffix.as_bytes()],
        bump
    )]
    pub suffix_account: Account<'info, SuffixAccount>,
    /// CHECK: This is the new product owner being added
    pub new_product_owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterProductOwner>, suffix: String) -> Result<()> {

    let suffix_account = &mut ctx.accounts.suffix_account;
    suffix_account.owner = *ctx.accounts.new_product_owner.key;
    suffix_account.is_active = true;
    suffix_account.suffix = suffix;

    emit!(SuffixUpdated {
        suffix: suffix_account.suffix.clone(),
        owner: suffix_account.owner,
    });

    Ok(())
}