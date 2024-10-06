use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::EchoIDError as ErrorCode;
use light_sdk::{
    compressed_account::LightAccount, light_accounts, merkle_context::PackedAddressMerkleContext
};

use crate::ParamsUpdateReputation;

#[light_accounts]
#[instruction(username: String, project_suffix: String,chain_name: String, change: i64)]
pub struct UpdateReputation<'info> {
    #[light_account(
        mut,
        seeds = [b"admin"],
        constraint = admin_config.admin == admin.key() @ ErrorCode::Unauthorized
    )]
    pub admin_config: LightAccount<AdminConfig>,
    #[account(mut)]
    #[fee_payer]
    #[authority]
    pub admin: Signer<'info>,
    #[light_account(
        mut,
        seeds = [username.as_bytes(), b"@", project_suffix.as_bytes(),chain_name.as_bytes()],
        constraint = alias_account.username == username @ ErrorCode::InvalidAlias,
        constraint = alias_account.product_suffix == project_suffix @ ErrorCode::InvalidProjectSuffix,
        constraint = alias_account.chain_info.name == *chain_name @ ErrorCode::InvalidChainName
    )]
    pub alias_account: LightAccount<AliasAccount>,
    #[self_program]
    pub self_program: Program<'info, crate::program::EchoIdContract>,
}
