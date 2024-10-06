use anchor_lang::prelude::*;
use crate::state::AdminConfig;
use light_sdk::{
    compressed_account::LightAccount, light_accounts, merkle_context::PackedAddressMerkleContext
};

use crate::ParamsInitialize;

#[light_accounts]
pub struct Initialize<'info> {
    #[account(mut)]
    #[fee_payer]
    #[authority]
    pub admin: Signer<'info>,
    #[light_account(
        init,
        seeds = [b"admin"],
    )]
    pub admin_config: LightAccount<AdminConfig>,
     #[self_program]
    pub self_program: Program<'info, crate::program::EchoIdContract>,
}
