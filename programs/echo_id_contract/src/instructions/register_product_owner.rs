use anchor_lang::prelude::*;
use crate::{state::*, error::EchoIDError as ErrorCode};
use light_sdk::{
    compressed_account::LightAccount, light_accounts, merkle_context::PackedAddressMerkleContext
};
use crate::ParamsRegisterProductOwner;

#[light_accounts]
#[instruction(suffix: String)]
pub struct RegisterProductOwner<'info> {
    #[account(mut)]
    #[fee_payer]
    pub admin: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::EchoIdContract>,
    #[light_account(
        mut,
        seeds = [b"admin"],
        constraint = admin_config.admin == admin.key() @ ErrorCode::Unauthorized
    )]
    pub admin_config: LightAccount<AdminConfig>,
    #[light_account(
        init,
        seeds = [b"suffix", suffix.as_bytes()],
    )]
    pub suffix_account: LightAccount<SuffixAccount>,
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub new_product_owner: AccountInfo<'info>,
}


