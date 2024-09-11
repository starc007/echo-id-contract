use anchor_lang::prelude::*;

#[account]
pub struct AliasAccount {
    pub owner: Pubkey,
    pub alias: String,
    pub chain_type: ChainType,
    pub chain_id: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ChainType {
    EVM,
    SVM,
}

#[account]
pub struct ReputationAccount {
    pub alias: String,
    pub score: i64,
    pub last_update: i64,
}

#[account]
pub struct CrossChainAccount {
    pub alias: String,
    pub chain_mappings: Vec<ChainMapping>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ChainMapping {
    pub chain_id: u8,
    pub address: [u8; 32],
}
