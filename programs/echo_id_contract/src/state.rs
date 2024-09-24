use anchor_lang::prelude::*;


pub const MAX_CHAIN_MAPPINGS: usize = 32; 
#[account]
pub struct AdminConfig {
    pub admin: Pubkey,
}

#[account]
pub struct SuffixAccount {
    pub owner: Pubkey,
    pub is_active: bool,
    pub suffix: String,
}

#[account]
pub struct AliasAccount {
    pub owner: Pubkey,
    pub username: String,
    pub product_suffix: String,
    pub chain_info: ChainInfo,
    pub reputation: i64,
    pub reputation_updated_at: i64,
    pub metadata: AliasMetadata,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct ChainInfo {
    pub name: String,
    pub address: String, 
    pub chain_id: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct AliasMetadata {
    pub name: String,
    pub image_url: String,
}