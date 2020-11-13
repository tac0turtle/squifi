use num_enum::IntoPrimitive;
use solana_client_gen::solana_sdk::program_error::ProgramError;
use thiserror::Error;

/// Errors that may be returned by the StakePool program.
#[derive(Debug, Error)]
pub enum FundError {
    #[error(transparent)]
    ProgramError(#[from] ProgramError),
    #[error("{0:?}")]
    ErrorCode(#[from] FundErrorCode),
}
#[derive(Debug, IntoPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum FundErrorCode {
    AlreadyInUse = 0,
    WrongSerialization = 1,
    NotOwnedByProgram = 2,
    NotInitialized = 3,
    InvalidVaultNonce = 4,
    InvalidVault = 5,
    InvalidAccountOwner = 6,
    AlreadyInitialized = 7,
    InvalidMint = 8,
    UnitializedTokenMint = 9,
    Unauthorized = 10,
    InvalidRentSysvar = 11,
    InvalidAccount = 12,
    FundBalanceOverflow = 13,
    FundClosed = 14,
    FundOpen = 15,
    WhitelistInvalidData = 16,
    PubKeyAlreadyExists = 17,
}

impl std::fmt::Display for FundErrorCode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        <Self as std::fmt::Debug>::fmt(self, fmt)
    }
}
impl std::error::Error for FundErrorCode {}

impl std::convert::From<FundError> for ProgramError {
    fn from(e: FundError) -> ProgramError {
        match e {
            FundError::ProgramError(e) => e,
            FundError::ErrorCode(c) => ProgramError::Custom(c.into()),
        }
    }
}
