//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    info,
    program_error::{PrintProgramError, ProgramError},
};

use thiserror::Error;

/// Errors that may be returned by the StakePool program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum FundError {
    // The account cannot be initialized because it is already being used.
    #[error("AlreadyInUse")]
    AlreadyInUse,
    // The proposed state is not valid
    #[error("InvalidState")]
    InvalidState,
    // The proposed state is not valid
    #[error("WrongSerialization")]
    WrongSerialization,
}

impl From<FundError> for ProgramError {
    fn from(e: FundError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for FundError {
    fn type_of() -> &'static str {
        "FundError"
    }
}

impl PrintProgramError for FundError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            FundError::AlreadyInUse => info!("Error: AlreadyInUse"),
            FundError::InvalidState => info!("Error: InvalidState"),
        }
    }
}
