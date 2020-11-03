//! Instruction types

use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::mem::size_of;

/// Inital values for the Fund
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitArgs {
  // Max Size of a fund
  pub max: u64,
}

/// Instructions supported by the Fund program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum FundInstruction {
  // Initializes a new Fund.
  //
  // Fund to create
  // Owner
  // Fund token
  // Token program ID
  // InitArgs
  Initialize(InitArgs),
}

impl FundInstruction {
  pub fn deserialize(input: &[u8]) -> Result<Self, ProgramError> {
    if input.len() < size_of::<u8>() {
      return Err(ProgramError::InvalidAccountData);
    }
    Ok(match input[0] {
      0 => {
        let val: &InitArgs = unpack(input)?;
        Self::Initialize(*val)
      }
      _ => return Err(ProgramError::InvalidAccountData),
    })
  }

  pub fn serialize(&self) -> Result<Vec<u8>, ProgramError> {
    let mut output = vec![0u8; size_of::<FundInstruction>()];
    match self {
      Self::Initialize(init) => {
        output[0] = 0;
        #[allow(clippy::cast_ptr_alignment)]
        let value = unsafe { &mut *(&mut output[1] as *mut u8 as *mut InitArgs) };
        *value = *init;
      }
    }
    Ok(output)
  }
}

/// Unpacks a reference from a bytes buffer.
pub fn unpack<T>(input: &[u8]) -> Result<&T, ProgramError> {
  if input.len() < size_of::<u8>() + size_of::<T>() {
    return Err(ProgramError::InvalidAccountData);
  }
  #[allow(clippy::cast_ptr_alignment)]
  let val: &T = unsafe { &*(&input[1] as *const u8 as *const T) };
  Ok(val)
}

pub fn initialize(
  program_id: &Pubkey,
  fund: &Pubkey,
  owner: &Pubkey,
  fund_mint: &Pubkey,
  owner_pool_account: &Pubkey,
  token_program_id: &Pubkey,
  init_args: InitArgs,
) -> Result<Instruction, ProgramError> {
  let init_data = FundInstruction::Initialize(init_args);
  let data = init_data.serialize()?;
  let accounts = vec![
    AccountMeta::new(*fund, true),
    AccountMeta::new_readonly(*owner, false),
    AccountMeta::new_readonly(*fund_mint, false),
    AccountMeta::new_readonly(*owner_pool_account, false),
    AccountMeta::new_readonly(*token_program_id, false),
  ];
  Ok(Instruction {
    program_id: *program_id,
    accounts,
    data,
  })
}
