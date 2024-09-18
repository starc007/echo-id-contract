use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod instructions;
pub mod zkp;
pub mod merkle;

use instructions::*;

declare_id!("FLWz8YARctJPAAdcx347mSMrxwp9zNsZExQ91Sf2Yos2");

#[program]
pub mod echo_id_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn register_project_suffix(ctx: Context<RegisterProjectSuffix>, suffix: String) -> Result<()> {
        instructions::register_project_suffix::handler(ctx, suffix)
    }

    pub fn register_alias(ctx: Context<RegisterAlias>, params: register_alias::RegisterAliasParams) -> Result<()> {
        instructions::register_alias::handler(ctx, params)
    }

    pub fn add_chain_mapping(ctx: Context<AddChainMapping>, params: add_chain_mapping::AddChainMappingParams) -> Result<()> {
        instructions::add_chain_mapping::handler(ctx, params)
    }

    pub fn update_reputation(ctx: Context<UpdateReputation>, username: String, project_suffix: String, change: i64) -> Result<()> {
        instructions::update_reputation::handler(ctx, username, project_suffix, change)
    }

    pub fn verify_alias_ownership(ctx: Context<VerifyAliasOwnership>, proof_data: verify_alias_ownership::ProofData) -> Result<()> {
        instructions::verify_alias_ownership::handler(ctx, proof_data)
    }
}