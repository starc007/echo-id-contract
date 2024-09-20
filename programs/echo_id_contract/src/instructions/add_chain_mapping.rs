use anchor_lang::prelude::*;
use crate::{
    error::EchoIDError as ErrorCode,
    state::{AliasAccount, ChainMapping, ProductOwner},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddChainMappingParams {
    pub new_mapping: ChainMapping,
}

#[derive(Accounts)]
#[instruction(params: AddChainMappingParams)]
pub struct AddChainMapping<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"product_owner", alias_account.owner.as_ref()],
        bump,
        constraint = product_owner_account.address == alias_account.owner @ ErrorCode::Unauthorized,
        constraint = product_owner_account.is_active @ ErrorCode::ProductOwnerNotActive
    )]
    pub product_owner_account: Account<'info, ProductOwner>,
    #[account(
        mut,
        seeds = [alias_account.username.as_bytes(), b"@", alias_account.product_suffix.as_bytes()],
        bump,
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