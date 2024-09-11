use anchor_lang::prelude::*;
use crate::state::{AliasAccount, ChainType};

#[derive(Accounts)]
#[instruction(alias: String, chain_type: String, chain_id: u32)]
pub struct RegisterAlias<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + alias.len() + 1 + 4, // 8 (discriminator) + 32 (pubkey) + 4 (string length) + alias length + 1 (chain_type) + 4 (chain_id)
        seeds = [alias.as_bytes()],
        bump
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterAlias>, alias: String, chain_type: String, chain_id: u32) -> Result<()> {
    let alias_account = &mut ctx.accounts.alias_account;
    alias_account.owner = *ctx.accounts.owner.key;
    alias_account.alias = alias;
    alias_account.chain_type = match chain_type.to_lowercase().as_str() {
        "svm" => ChainType::SVM,
        "evm" => ChainType::EVM,
        _ => return Err(ProgramError::InvalidArgument.into()),
    };
    alias_account.chain_id = chain_id;
    Ok(())
}