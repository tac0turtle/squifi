//! State transition types

use crate::error::FundError;
use crate::instruction::unpack;
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};
use std::mem::size_of;

/// Initialized program details.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fund {
  /// Owner authority
  /// allows for updating the Fund authority
  pub owner: Pubkey,
  // Pool Mint
  pub pool_mint: Pubkey,
  // Pool token program id
  pub token_program_id: Pubkey,
  // total under management
  pub raised_total: u64,
  // total pool
  pub fund_total: u64,
  // max size of the fund
  pub max: u64,
}

impl Fund {}

/// Program states.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
  /// Unallocated state, may be initialized into another state.
  Unallocated,
  /// Initialized state.
  Init(Fund),
}

impl State {
  /// Length of state data when serialized
  pub const LEN: usize = size_of::<u8>() + size_of::<Fund>();
  /// Deserializes a byte buffer into a [State](struct.State.html).
  pub fn deserialize(input: &[u8]) -> Result<State, ProgramError> {
    if input.len() < size_of::<u8>() {
      return Err(ProgramError::InvalidAccountData);
    }
    Ok(match input[0] {
      0 => State::Unallocated,
      1 => {
        // We send whole input here, because unpack skips the first byte
        let swap: &Fund = unpack(&input)?;
        State::Init(*swap)
      }
      _ => return Err(ProgramError::InvalidAccountData),
    })
  }
  pub fn serialize(&self, output: &mut [u8]) -> ProgramResult {
    if output.len() < size_of::<u8>() {
      return Err(ProgramError::InvalidAccountData);
    }
    match self {
      Self::Unallocated => output[0] = 0,
      Self::Init(swap) => {
        if output.len() < size_of::<u8>() + size_of::<Fund>() {
          return Err(ProgramError::InvalidAccountData);
        }
        output[0] = 1;
        #[allow(clippy::cast_ptr_alignment)]
        let value = unsafe { &mut *(&mut output[1] as *mut u8 as *mut Fund) };
        *value = *swap;
      }
    }
    Ok(())
  }
  /// Gets the `Fund` from `State`
  pub fn fund(&self) -> Result<Fund, ProgramError> {
    if let State::Init(swap) = &self {
      Ok(*swap)
    } else {
      Err(FundError::InvalidState.into())
    }
  }
}
