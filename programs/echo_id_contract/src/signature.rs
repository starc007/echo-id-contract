use anchor_lang::prelude::*;
use anchor_lang::solana_program::ed25519_program::ID as ED25519_PROGRAM_ID;
use anchor_lang::solana_program::instruction::Instruction;
use crate::error::EchoIDError as ErrorCode;

pub fn verify_signature(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool> {
    let pubkey = Pubkey::try_from(public_key).map_err(|_| ErrorCode::InvalidPublicKey)?;
    
    let instruction = Instruction::new_with_bytes(
        ED25519_PROGRAM_ID,
        &[
            &[signature.len() as u8][..],
            signature,
            &[public_key.len() as u8][..],
            public_key,
            &[message.len() as u8][..],
            message,
        ]
        .concat(),
        vec![],
    );

    let account_infos = vec![];

    match anchor_lang::solana_program::program::invoke(&instruction, &account_infos) {
        Ok(_) => Ok(true),
        Err(ProgramError::InvalidAccountData) => Ok(false),
        Err(e) => Err(e.into()),
    }
}