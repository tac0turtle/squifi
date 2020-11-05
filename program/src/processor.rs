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
    let fund = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;

    if State::Unallocated != State::deserialize(&fund.data.borrow())? {
      return Err(FundError::AlreadyInUse.into());
    }

    let init_fund = State::Init(Fund {
      owner: init.owner,
      mint: *mint.key,
      fund: *fund.key,
      max: init.max,
      nounce: 0,
    });
    init_fund.serialize(&mut fund.data.borrow_mut())
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
