use anchor_lang::prelude::*;
use crate::{
    error::EchoIDError as ErrorCode,
    state::{AliasAccount, ProductOwner},
    zkp::{self, SerializableProof},
};

#[derive(Accounts)]
pub struct VerifyAliasOwnership<'info> {
    #[account(mut)]
    pub product_owner: Signer<'info>,
    #[account(
        seeds = [b"product_owner", product_owner.key().as_ref()],
        bump,
        constraint = product_owner_account.address == product_owner.key() @ ErrorCode::Unauthorized,
        constraint = product_owner_account.is_active @ ErrorCode::ProductOwnerNotActive
    )]
    pub product_owner_account: Account<'info, ProductOwner>,
    #[account(
        seeds = [alias_account.username.as_bytes(), b"@", alias_account.product_suffix.as_bytes()],
        bump,
        constraint = alias_account.product_suffix == product_owner_account.suffix @ ErrorCode::Unauthorized
    )]
    pub alias_account: Account<'info, AliasAccount>,
}

pub fn handler(ctx: Context<VerifyAliasOwnership>, proof: SerializableProof) -> Result<()> {
    let alias_account = &ctx.accounts.alias_account;
    
    // Convert the stored public key bytes to a PublicKey
    let public_key = zkp::PublicKey::from_bytes(&alias_account.zk_public_key)
        .ok_or(ErrorCode::InvalidPublicKey)?;

    // Convert SerializableProof to Proof
    let zkp_proof = proof.into_proof().ok_or(ErrorCode::InvalidProof)?;

    // The message to be verified is the concatenation of the username and product suffix
    let message = [
        alias_account.username.as_bytes(),
        b"@",
        alias_account.product_suffix.as_bytes()
    ].concat();

    // Verify the ZK proof
    require!(
        zkp::verify(&public_key, &message, &zkp_proof),
        ErrorCode::InvalidProof
    );

    // If we've reached this point, the proof is valid
    msg!("Alias ownership verified successfully");

    Ok(())
}