use anchor_lang::prelude::*;
use crate::{error::EchoIDError, state::{AdminConfig, ProjectSuffix}};

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct RegisterProjectSuffix<'info> {
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
        space = 8 + 4 + suffix.len(),
        seeds = [b"project_suffix", suffix.as_bytes()],
        bump
    )]
    pub project_suffix_account: Account<'info, ProjectSuffix>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterProjectSuffix>, suffix: String) -> Result<()> {
    let project_suffix_account = &mut ctx.accounts.project_suffix_account;
    project_suffix_account.suffix = suffix;
    Ok(())
}