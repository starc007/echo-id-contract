use anchor_lang::prelude::*;
use crate::state::{AliasAccount, AdminConfig};
use crate::error::EchoIDError as ErrorCode;

#[derive(Accounts)]
#[instruction(username: String, project_suffix: String, reputation_change: i64)]
pub struct UpdateReputation<'info> {
    #[account(
        seeds = [b"admin"],
        bump,
        constraint = admin_config.admin == admin.key() @ ErrorCode::Unauthorized
    )]
    pub admin_config: Account<'info, AdminConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [username.as_bytes(), b"@", project_suffix.as_bytes()],
        bump,
        constraint = alias_account.username == username @ ErrorCode::InvalidAlias,
        constraint = alias_account.product_suffix == project_suffix @ ErrorCode::InvalidProjectSuffix
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateReputation>, username: String, project_suffix: String, reputation_change: i64) -> Result<()> {
    let alias_account = &mut ctx.accounts.alias_account;
    
    // Verify that the provided username and project_suffix match the account
    require!(alias_account.username == username, ErrorCode::InvalidAlias);
    require!(alias_account.product_suffix == project_suffix, ErrorCode::InvalidProjectSuffix);

    // Update reputation
    alias_account.reputation = alias_account.reputation.saturating_add(reputation_change);
    alias_account.reputation_updated_at = Clock::get()?.unix_timestamp;

    msg!("Updated reputation for {}@{} by {}", username, project_suffix, reputation_change);
    Ok(())
}