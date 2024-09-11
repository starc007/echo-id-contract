use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod instructions;

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

    pub fn register_alias(ctx: Context<RegisterAlias>, params: RegisterAliasParams) -> Result<()> {
        instructions::register_alias::handler(ctx, params)
    }


    pub fn initialize_reputation(ctx: Context<InitializeReputation>, username: String, project_suffix: String) -> Result<()> {
        instructions::initialize_reputation::handler(ctx, username, project_suffix)
    }

    pub fn update_reputation(ctx: Context<UpdateReputation>, username: String, project_suffix: String, change: i64) -> Result<()> {
        instructions::update_reputation::handler(ctx, username, project_suffix, change)
    }

}
