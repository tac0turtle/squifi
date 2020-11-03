//! Program state processor

use crate::{
  error::FundError,
  instruction::{FundInstruction, InitArgs},
  state::{Fund, State},
};

use solana_program::{
  account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, info,
  pubkey::Pubkey,
};

pub struct Processor {}

impl Processor {
  pub fn process_initialize(
    _program_id: &Pubkey,
    init: InitArgs,
    accounts: &[AccountInfo],
  ) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let fund_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    let pool_mint_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    // Stake pool account should not be already initialized
    if State::Unallocated != State::deserialize(&fund_info.data.borrow())? {
      return Err(FundError::AlreadyInUse.into());
    }

    let fund = State::Init(Fund {
      owner: *owner_info.key,
      pool_mint: *pool_mint_info.key,
      token_program_id: *token_program_info.key,
      fund_total: 0,
      raised_total: 0,
      max: init.max,
    });
    fund.serialize(&mut fund_info.data.borrow_mut())
  }
  pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = FundInstruction::deserialize(input)?;
    match instruction {
      FundInstruction::Initialize(init) => {
        info!("Instruction Init");
        Self::process_initialize(program_id, init, accounts)
      }
    }
  }
}
