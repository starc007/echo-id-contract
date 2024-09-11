use anchor_lang::prelude::*;
use crate::state::AdminConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + 32,
        seeds = [b"admin"],
        bump
    )]
    pub admin_config: Account<'info, AdminConfig>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let admin_config = &mut ctx.accounts.admin_config;
    admin_config.admin = ctx.accounts.admin.key();
    Ok(())
}