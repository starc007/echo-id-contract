use anchor_lang::prelude::*;
use light_sdk::light_program;
use crate::{
    error::EchoIDError as ErrorCode,
    state::*,
};

pub mod state;
pub mod error;
pub mod instructions;

use instructions::*;

declare_id!("5zFfHUucGgrkRuCAvEE2QSr5aoXrZmeARULQZ5k3YKy");

#[light_program]
#[program]
pub mod echo_id_contract {

    use super::*;

    pub fn initialize<'info>(ctx: LightContext<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        let admin_key = ctx.accounts.admin.key();
        let admin_config = &mut ctx.light_accounts.admin_config;
        admin_config.admin = admin_key;
        Ok(())
    }

    pub fn register_product_owner<'info>(ctx: LightContext<'_, '_, '_, 'info, RegisterProductOwner<'info>>, suffix: String) -> Result<()> {

        let suffix_account = &mut ctx.light_accounts.suffix_account;
        suffix_account.owner = *ctx.anchor_context.accounts.new_product_owner.key;
        suffix_account.is_active = true;
        suffix_account.suffix = suffix;

        emit!(SuffixUpdated {
            suffix: suffix_account.suffix.clone(),
            owner: suffix_account.owner,
        });

        Ok(())
    }

    pub fn register_alias<'info>(ctx: LightContext<'_, '_, '_, 'info, RegisterAlias<'info>>, params: register_alias::RegisterAliasParams) -> Result<()> {
        // instructions::register_alias::handler(ctx, params)
        require!(!params.username.contains('@'), ErrorCode::InvalidUsername);
    
        let alias_account = &mut ctx.light_accounts.alias_account;
    
        alias_account.owner = params.user_address;
        alias_account.username = params.username;
        alias_account.product_suffix = params.suffix;

        
        alias_account.chain_info = params.chain_info;
        alias_account.metadata = params.metadata;
        
        alias_account.reputation = 10; // Initial reputation
        alias_account.reputation_updated_at = Clock::get()?.unix_timestamp;

        emit!(AliasRegistered {
            username: alias_account.username.clone(),
            suffix: alias_account.product_suffix.clone(),
            chain_info: alias_account.chain_info.clone(),
            metadata: alias_account.metadata.clone(),
            user_address: alias_account.owner,
        });

        Ok(())
    }


    pub fn update_reputation<'info>(ctx: LightContext<'_, '_, '_, 'info, UpdateReputation<'info>>, username: String, project_suffix: String, chain_name:String,  change: i64) -> Result<()> {
         let alias_account = &mut ctx.light_accounts.alias_account;
    
    // Verify that the provided username and project_suffix match the account
        require!(alias_account.username == username, ErrorCode::InvalidAlias);
        require!(alias_account.product_suffix == project_suffix, ErrorCode::InvalidProjectSuffix);
        require!(alias_account.chain_info.name == chain_name, ErrorCode::InvalidChainName);
        // Update reputation
        alias_account.reputation = alias_account.reputation.saturating_add(change);
        alias_account.reputation_updated_at = Clock::get()?.unix_timestamp;

        
        emit!(AliasReputationUpdated {
            username: alias_account.username.clone(),
            suffix: alias_account.product_suffix.clone(),
            chain_info: alias_account.chain_info.clone(),
            metadata: alias_account.metadata.clone(),
            user_address: alias_account.owner,
            });
        Ok(())
    }

}
