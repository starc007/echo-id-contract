use anchor_lang::prelude::*;


pub const MAX_CHAIN_MAPPINGS: usize = 32; 
#[account]
pub struct AdminConfig {
    pub admin: Pubkey,
}

#[account]
pub struct ProductOwner {
    pub address: Pubkey,
    pub is_active: bool,
    pub suffix: String,
}

#[account]
pub struct AliasAccount {
    pub owner: Pubkey,
    pub username: String,
    pub product_suffix: String,
    pub chain_mappings: Vec<ChainMapping>,
    pub reputation: i64,
    pub reputation_updated_at: i64,
    pub public_key: [u8; 32],
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
