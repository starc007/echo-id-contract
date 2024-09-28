use anchor_lang::prelude::*;

#[error_code]
pub enum EchoIDError {
    #[msg("Invalid instruction")]
    InvalidInstruction,

    #[msg("Invalid signer")]
    InvalidSigner,

    #[msg("Invalid derived account")]
    InvalidDerivedAccount,

    #[msg("Alias already exists")]
    AliasAlreadyExists,

    #[msg("Alias does not exist")]
    AliasDoesNotExist,

    #[msg("Invalid owner")]
    InvalidOwner,

    #[msg("Alias is too long")]
    AliasTooLong,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Chain ID already exists for this alias")]
    ChainIDAlreadyExists,

    #[msg("Maximum number of chain mappings reached")]
    MaxChainMappingsReached,

    #[msg("Invalid reputation change")]
    InvalidReputationChange,

    #[msg("Reputation account not initialized")]
    ReputationAccountNotInitialized,

    #[msg("Cross-chain account not initialized")]
    CrossChainAccountNotInitialized,

    #[msg("Invalid username format")]
    InvalidUsername,
    #[msg("Invalid chain type")]
    InvalidChainType,
    #[msg("Unauthorized")]
    Unauthorized,

     #[msg("Invalid project suffix")]
    InvalidProjectSuffix,

    #[msg("Invalid alias")]
    InvalidAlias,

    #[msg("Address cannot be empty")]
    EmptyAddress,
    
    #[msg("Chain mapping already exists for this chain type")]
    ChainMappingAlreadyExists,

    #[msg("Invalid chain mapping")]
    InvalidChainMapping,

    #[msg("Invalid proof")]
    InvalidProof,

    #[msg("Invalid public key")]
    InvalidPublicKey,

    #[msg("Invalid Merkle proof")]
    InvalidMerkleProof,

    #[msg("Product owner already exists")]
    ProductOwnerAlreadyExists,

    #[msg("Product owner not found")]
    ProductOwnerNotFound,

    #[msg("Suffix already exists")]
    SuffixAlreadyExists,

    #[msg("Product owner is not active")]
    ProductOwnerNotActive,

    #[msg("Invalid signature")]
    InvalidSignature,
      
    #[msg("Signature verification failed")]
    SignatureVerificationFailed,

    #[msg("Invalid chain name")]
    InvalidChainName,

    #[msg("Invalid user")]
    InvalidUser,
}
