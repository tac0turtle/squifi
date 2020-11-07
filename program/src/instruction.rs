//! Instruction types

use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::mem::size_of;

use crate::state::FundType;

/// Inital values for the Fund
#[derive(Clone, Copy, Debug)]
pub struct InitArgs {
  /// Owner of the Fund
  pub owner: Pubkey, // Optional in the future for when gov spending is implemented?
  /// Max Size of a fund
  pub max_balance: u32,
  /// fund type
  pub fund_type: FundType,
}

/// Instructions supported by the Fund program.
#[derive(Clone, Debug)]
pub enum FundInstruction {
  /// Initializes a new Fund & Fund Account
  ///
  /// [writable] Fund to create
  /// [writable] Account to create
  /// [writable] Program controlled tokenvault.
  /// [] SPL token program
  /// []Mint
  Initialize(InitArgs),
}

serum_common::packable!(FundInstruction);
