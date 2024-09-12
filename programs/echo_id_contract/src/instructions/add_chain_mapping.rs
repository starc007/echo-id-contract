use anchor_lang::prelude::*;
use crate::{error::EchoIDError, state::{AliasAccount, ChainMapping, ChainType}};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddChainMappingParams {
    pub chain_type: String,
    pub chain_id: u32,
    pub address: String,
}

#[derive(Accounts)]
#[instruction(params: AddChainMappingParams)]
pub struct AddChainMapping<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [alias_account.username.as_bytes(), b"@", alias_account.project_suffix.as_bytes()],
        bump,
        has_one = owner,
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddChainMapping>, params: AddChainMappingParams) -> Result<()> {
    require!(!params.address.is_empty(), EchoIDError::EmptyAddress);
    
    let alias_account = &mut ctx.accounts.alias_account;
    
    let chain_type = match params.chain_type.as_str() {
        "svm" => ChainType::SVM,
        "evm" => ChainType::EVM,
        _ => return Err(EchoIDError::InvalidChainType.into()),
    };
    
    // Check if this chain type already exists
    if alias_account.chain_mappings.iter().any(|mapping| mapping.chain_type == chain_type) {
        return Err(EchoIDError::ChainMappingAlreadyExists.into());
    }
    
    alias_account.chain_mappings.push(ChainMapping {
        chain_type,
        chain_id: params.chain_id,
        address: params.address,
    });

    Ok(())
}