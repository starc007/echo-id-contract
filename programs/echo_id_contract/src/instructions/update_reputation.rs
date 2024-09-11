use anchor_lang::prelude::*;
use crate::state::{AliasAccount, ReputationAccount};

#[derive(Accounts)]
pub struct UpdateReputation<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        has_one = owner,
    )]
    pub alias_account: Account<'info, AliasAccount>,
    #[account(
        init,
        payer = owner,
        space = 8 + 4 + 32 + 8 + 8,
        seeds = [b"reputation", alias_account.alias.as_bytes()],
        bump
    )]
    pub reputation_account: Account<'info, ReputationAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateReputation>, change: i64) -> Result<()> {
    let reputation_account = &mut ctx.accounts.reputation_account;
    reputation_account.alias = ctx.accounts.alias_account.alias.clone();
    reputation_account.score = reputation_account.score.saturating_add(change);
    reputation_account.last_update = Clock::get()?.unix_timestamp;
    Ok(())
}