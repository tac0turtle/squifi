//! State transition types

use crate::error::Error;
use crate::instruction::{unpack, Fee};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};
use std::mem::size_of;

/// Initialized program details.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fund {
  /// Owner authority
  /// allows for updating the Fund authority
  pub owner: Pubkey,
  // /// Deposit authority bump seed
  // /// for `create_program_address(&[state::StakePool account, "deposit"])`
  // pub deposit_bump_seed: u8,
  // /// Withdrawal authority bump seed
  // /// for `create_program_address(&[state::StakePool account, "withdrawal"])`
  // pub withdraw_bump_seed: u8,
  /// Pool Mint
  pub pool_mint: Pubkey,
  /// Owner fee account
  pub owner_fee_account: Pubkey,
  /// Pool token program id
  pub token_program_id: Pubkey,
  /// total under management
  pub raised_total: u64,
  /// total pool
  pub fund_total: u64,
}

impl Fund {}
