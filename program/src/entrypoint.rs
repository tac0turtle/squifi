//! Program entrypoint

use crate::{
  error::FundError,
  initialize,
  instruction::{FundInstruction, InitArgs},
};
use solana_program::{
  account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, info, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction<'a>(
  program_id: &Pubkey,
  accounts: &'a [AccountInfo<'a>],
  instruction_data: &[u8],
) -> ProgramResult {
  info!("process-instruction");

  let instruction: FundInstruction =
    FundInstruction::unpack(instruction_data).map_err(|_| FundError::WrongSerialization)?;

  let result = match instruction {
    FundInstruction::Initialize { InitArgs } => {
      initialize::initialize(program_id, accounts, InitArgs)
    }
  };
  Ok(())
}
