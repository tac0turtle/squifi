//! Instruction types

use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::mem::size_of;

/// Instructions supported by the StakePool program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum FundInstruction {
  // Initializes a new Fund.
  //
  // Fund to create
  // Owner
  // Fund token
  // Token program ID
  Initialize,
}

impl FundInstruction {
  pub fn deserialize(input: &[u8]) -> Result<Self, ProgramError> {
    if input.len() < size_of::<u8>() {
      return Err(ProgramError::InvalidAccountData);
    }
    Ok(match input[0] {
      0 => Self::Initialize,
      _ => return Err(ProgramError::InvalidAccountData),
    })
  }

  pub fn serialize(&self) -> Result<Vec<u8>, ProgramError> {
    let mut output = vec![0u8; size_of::<StakePoolInstruction>()];
    match self {
      Self::Initialize(init) => {
        output[0] = 0;
      }
    }
    Ok(output)
  }
}

pub fn initialize(
  program_id: &Pubkey,
  fund: &Pubkey,
  owner: &Pubkey,
  pool_mint: &Pubkey,
  owner_pool_account: &Pubkey,
  token_program_id: &Pubkey,
) -> Result<Instruction, ProgramError> {
  let init_data = FundInstruction::Initialize;
  let data = init_data.serialize()?;
  let accounts = vec![
    AccountMeta::new(*stake_pool, true),
    AccountMeta::new_readonly(*owner, false),
    AccountMeta::new_readonly(*pool_mint, false),
    AccountMeta::new_readonly(*owner_pool_account, false),
    AccountMeta::new_readonly(*token_program_id, false),
  ];
  Ok(Instruction {
    program_id: *program_id,
    accounts,
    data,
  })
}
