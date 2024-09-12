use anchor_lang::prelude::*;
use crate::state::{AliasAccount, AdminConfig};

#[derive(Accounts)]
#[instruction(username: String, project_suffix: String, reputation_change: i64)]
pub struct UpdateReputation<'info> {
    #[account(
        seeds = [b"admin"],
        bump,
        constraint = admin_config.admin == admin.key()
    )]
    pub admin_config: Account<'info, AdminConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [username.as_bytes(), b"@", project_suffix.as_bytes()],
        bump,
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateReputation>, username: String, project_suffix: String, reputation_change: i64) -> Result<()> {
    let alias_account = &mut ctx.accounts.alias_account;
    let clock = Clock::get()?;
    alias_account.reputation = alias_account.reputation.saturating_add(reputation_change);
    alias_account.reputation_updated_at = clock.unix_timestamp;
    Ok(())
}