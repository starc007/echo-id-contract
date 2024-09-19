use anchor_lang::prelude::*;

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
    pub chain_mappings_root: [u8; 32],
    pub chain_mapping_count: u32,
    pub reputation: i64,
    pub reputation_updated_at: i64,
    pub zk_public_key: [u8; 32],
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

impl ChainType {
    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            ChainType::SVM => [0, 0, 0, 0],
            ChainType::EVM => [0, 0, 0, 1],
        }
    }
}
