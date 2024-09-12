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
        space = 8 + // discriminator
                32 + // owner
                4 + params.username.len() + // username
                4 + params.project_suffix.len() + // project_suffix
                4 + (10 * (1 + 4 + 32)) + // space for up to 10 chain mappings
                8 + // reputation_score
                8, // last_reputation_update

        seeds = [params.username.as_bytes(), b"@", params.project_suffix.as_bytes()],
        bump,
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
    
    let chain_type = match params.chain_type.as_str() {
        "svm" => ChainType::SVM,
        "evm" => ChainType::EVM,
        _ => return Err(EchoIDError::InvalidChainType.into()),
    };
    
    alias_account.chain_mappings = vec![ChainMapping {
        chain_type,
        address: params.address,
        chain_id: params.chain_id,
    }];

    //initialize reputation
    alias_account.reputation = 10;
    alias_account.reputation_updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}