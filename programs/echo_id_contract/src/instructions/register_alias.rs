use anchor_lang::prelude::*;
use crate::{error::EchoIDError, state::{AliasAccount, ChainType, ProjectSuffix,ChainMapping}};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterAliasParams {
    pub username: String,
    pub project_suffix: String,
    pub chain_id: u32,
    pub chain_type: String,
    pub address: String,
}


#[derive(Accounts)]
#[instruction(params: RegisterAliasParams)]
pub struct RegisterAlias<'info> {
   #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + params.username.len() + 4 + params.project_suffix.len() + 32 + 4 + 32 + 1 + 32,
        seeds = [params.username.as_bytes(), b"@", params.project_suffix.as_bytes()],
        bump
    )]
    pub alias_account: Account<'info, AliasAccount>,
    #[account(
        seeds = [b"project_suffix", params.project_suffix.as_bytes()],
        bump,
        constraint = project_suffix_account.suffix == params.project_suffix @ EchoIDError::InvalidProjectSuffix
    )]
    pub project_suffix_account: Account<'info, ProjectSuffix>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterAlias>, params:RegisterAliasParams) -> Result<()> {
    require!(!params.username.contains('@'), EchoIDError::InvalidUsername);
    require!(!params.address.is_empty(), EchoIDError::EmptyAddress);
    
    let alias_account = &mut ctx.accounts.alias_account;
    alias_account.username = params.username;
    alias_account.project_suffix = params.project_suffix;
    alias_account.owner = *ctx.accounts.owner.key;
    alias_account.chain_id = params.chain_id;
    
    let chain_type = match params.chain_type.as_str() {
        "svm" => ChainType::SVM,
        "evm" => ChainType::EVM,
        _ => return Err(EchoIDError::InvalidChainType.into()),
    };
    
    alias_account.chain_mappings = vec![ChainMapping {
        chain_type,
        address: params.address,
    }];

    Ok(())
}