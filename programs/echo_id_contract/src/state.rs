use anchor_lang::prelude::*;
use light_sdk::light_account;
use light_hasher::bytes::AsByteVec;


pub const MAX_CHAIN_MAPPINGS: usize = 32; 
#[light_account]
#[derive(Clone, Debug, Default)]
pub struct AdminConfig {
    pub admin: Pubkey,
}

#[light_account]
#[derive(Clone, Debug, Default)]
pub struct SuffixAccount {
    pub owner: Pubkey,
    pub is_active: bool,
    pub suffix: String,
}

#[light_account]
#[derive(Clone, Debug, Default)]
pub struct AliasAccount {
    pub owner: Pubkey,
    pub username: String,
    pub product_suffix: String,
    pub chain_info: ChainInfo,
    pub reputation: i64,
    pub reputation_updated_at: i64,
    pub metadata: AliasMetadata,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug,Default)]
pub struct ChainInfo {
    pub name: String,
    pub address: String, 
    pub chain_id: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug,Default)]
pub struct AliasMetadata {
    pub name: String,
    pub image_url: String,
}

#[event]
pub struct AliasRegistered {
    pub username: String,
    pub suffix: String,
    pub chain_info: ChainInfo,
    pub metadata: AliasMetadata,
    pub user_address: Pubkey,
}

#[event]
pub struct AliasReputationUpdated {
    pub username: String,
    pub suffix: String,
    pub chain_info: ChainInfo,
    pub metadata: AliasMetadata,
    pub user_address: Pubkey,
}

#[event]
pub struct SuffixUpdated {
    pub suffix: String,
    pub owner: Pubkey,
}

impl AsByteVec for AliasMetadata {
    fn as_byte_vec(&self) -> Vec<Vec<u8>> {
        vec![
            self.name.as_bytes().to_vec(),
            self.image_url.as_bytes().to_vec(),
        ]
    }
}

impl AsByteVec for ChainInfo {
    fn as_byte_vec(&self) -> Vec<Vec<u8>> {
        vec![
            self.name.as_bytes().to_vec(),
            self.address.as_bytes().to_vec(),
            self.chain_id.to_be_bytes().to_vec(),
        ]
    }
}