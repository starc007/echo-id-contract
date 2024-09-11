use anchor_lang::prelude::*;
use crate::{error::EchoIDError, state::{AliasAccount, ReputationAccount}};

#[derive(Accounts)]
#[instruction(username: String, project_suffix: String)]
pub struct InitializeReputation<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        seeds = [username.as_bytes(), b"@", project_suffix.as_bytes()],
        bump,
        constraint = alias_account.username == username && alias_account.project_suffix == project_suffix @ EchoIDError::InvalidAlias
    )]
    pub alias_account: Account<'info, AliasAccount>,
    #[account(
        init,
        payer = initializer,
        space = 8 + 4 + (username.len() + project_suffix.len() + 1) + 8 + 8,
        seeds = [b"reputation", username.as_bytes(), b"@", project_suffix.as_bytes()],
        bump
    )]
    pub reputation_account: Account<'info, ReputationAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeReputation>, username: String, project_suffix: String) -> Result<()> {
    let reputation_account = &mut ctx.accounts.reputation_account;
    let alias = format!("{}@{}", username, project_suffix);

    reputation_account.alias = alias;
    reputation_account.score = 0;
    reputation_account.last_update = Clock::get()?.unix_timestamp;

    Ok(())
}