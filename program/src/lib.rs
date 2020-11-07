//! Program entrypoint

use fund::{
  error::{FundError, FundErrorCode},
  instruction::FundInstruction,
};
use serum_common::pack::Pack;
use solana_program::{
  account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, info, pubkey::Pubkey,
};

pub use solana_program;

solana_program::declare_id!("SPQQL11111111111111111111111111111111111111");

pub(crate) mod access_control;
mod initialize;

entrypoint!(process_instruction);
fn process_instruction<'a>(
  program_id: &Pubkey,
  accounts: &'a [AccountInfo<'a>],
  instruction_data: &[u8],
) -> ProgramResult {
  info!("process-instruction");

  let instruction: FundInstruction = FundInstruction::unpack(instruction_data)
    .map_err(|_| FundError::ErrorCode(FundErrorCode::WrongSerialization))?;

  let result = match instruction {
    FundInstruction::Initialize {
      owner,
      max_balance,
      fund_type,
    } => initialize::initialize(program_id, accounts, owner, max_balance, fund_type),
  };

  result?;

  info!("process-instruction success");

  Ok(())
}
