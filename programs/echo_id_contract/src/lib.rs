use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod instructions;

use instructions::*;

declare_id!("32o6oPNakFqnr53DseUrv8Bg3ircxzcRq1QCuy4Gqqu9");

#[program]
pub mod echo_id_contract {
    use super::*;

    pub fn register_alias(ctx: Context<RegisterAlias>, alias: String, chain_type: String, chain_id: u32) -> Result<()> {
        instructions::register_alias::handler(ctx, alias, chain_type, chain_id)
    }

    pub fn update_reputation(ctx: Context<UpdateReputation>, change: i64) -> Result<()> {
        instructions::update_reputation::handler(ctx, change)
    }

}
