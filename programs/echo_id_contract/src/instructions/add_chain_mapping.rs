use anchor_lang::prelude::*;
use crate::{
    error::EchoIDError as ErrorCode,
    state::{AliasAccount, ChainMapping},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddChainMappingParams {
    pub new_mapping: ChainMapping,
}

#[derive(Accounts)]
#[instruction(params: AddChainMappingParams)]
pub struct AddChainMapping<'info> {
    #[account(mut)]
    pub alias_owner: Signer<'info>,
    #[account(
        mut,
        seeds = [alias_account.username.as_bytes(), b"@", alias_account.product_suffix.as_bytes()],
        bump,
        constraint = alias_account.owner == alias_owner.key() @ ErrorCode::Unauthorized
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddChainMapping>, params: AddChainMappingParams) -> Result<()> {
    let alias_account = &mut ctx.accounts.alias_account;
    
     // Check if the chain mapping already exists
    require!(
        !alias_account.chain_mappings.iter().any(|m| m.chain_type == params.new_mapping.chain_type),
        ErrorCode::ChainMappingAlreadyExists
    );

    // Add the new chain mapping
    alias_account.chain_mappings.push(params.new_mapping);

    Ok(())
}