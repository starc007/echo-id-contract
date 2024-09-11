use anchor_lang::prelude::*;
use crate::{error::EchoIDError, state::{AliasAccount, ReputationAccount}};

#[derive(Accounts)]
#[instruction(username: String, project_suffix: String, change: i64)]
pub struct UpdateReputation<'info> {
    #[account(mut)]
    pub updater: Signer<'info>,
    #[account(
        seeds = [username.as_bytes(), b"@", project_suffix.as_bytes()],
        bump,
        constraint = alias_account.username == username && alias_account.project_suffix == project_suffix @ EchoIDError::InvalidAlias
    )]
    pub alias_account: Account<'info, AliasAccount>,
    #[account(
        mut,
        seeds = [b"reputation", username.as_bytes(), b"@", project_suffix.as_bytes()],
        bump
    )]
    pub reputation_account: Account<'info, ReputationAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateReputation>, username: String, project_suffix: String, change: i64) -> Result<()> {
    let reputation_account = &mut ctx.accounts.reputation_account;
    let alias = format!("{}@{}", username, project_suffix);

    require!(reputation_account.alias == alias, EchoIDError::InvalidAlias);

    reputation_account.score = reputation_account.score.saturating_add(change);
    reputation_account.last_update = Clock::get()?.unix_timestamp;

    Ok(())
}