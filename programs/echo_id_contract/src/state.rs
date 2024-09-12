use anchor_lang::prelude::*;


#[account]
pub struct AdminConfig {
    pub admin: Pubkey,
}

#[account]
pub struct ProjectSuffix {
    pub suffix: String,
}

#[account]
pub struct AliasAccount {
    pub owner: Pubkey,
    pub username: String,
    pub project_suffix: String,
    pub chain_mappings: Vec<ChainMapping>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct ChainMapping {
    pub chain_type: ChainType,
    pub address: String, 
    pub chain_id: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ChainType {
    SVM,
    EVM,
}
#[account]
pub struct ReputationAccount {
    pub alias: String,
    pub score: i64,
    pub last_update: i64,
}
