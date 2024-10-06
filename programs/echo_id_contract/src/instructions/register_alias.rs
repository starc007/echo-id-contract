use anchor_lang::prelude::*;
use crate::{
    error::EchoIDError as ErrorCode,
    state::*,
};

use light_sdk::{
    compressed_account::LightAccount, light_accounts, merkle_context::PackedAddressMerkleContext
};

use crate::ParamsRegisterAlias;
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterAliasParams {
    pub username: String,
    pub suffix: String,
    pub chain_info: ChainInfo,
    pub metadata: AliasMetadata,
    pub user_address: Pubkey,
}


#[light_accounts]
#[instruction(params: RegisterAliasParams)]
pub struct RegisterAlias<'info> {
    #[account(mut)]
    #[fee_payer]
    pub payer: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::EchoIdContract>,
    /// CHECK: This account is not written to or read from. It's just used for authorization.
    #[account(
        constraint = user.key() == params.user_address @ ErrorCode::InvalidUser
    )]
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub user: AccountInfo<'info>,
    #[light_account(
        mut,
        seeds = [b"suffix", params.suffix.as_bytes()],
        constraint = suffix_account.is_active @ ErrorCode::InvalidProjectSuffix
    )]
    pub suffix_account: LightAccount<SuffixAccount>,
    #[light_account(
        init,
        seeds = [params.username.as_bytes(), b"@", params.suffix.as_bytes(),params.chain_info.name.as_bytes()],
    )]
    pub alias_account: LightAccount<AliasAccount>,
}
