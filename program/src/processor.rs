//! Program state processor

use crate::{
  error::FundError,
  instruction::{FundInstruction, InitArgs},
  state::{Fund, State},
};
// use num_traits::FromPrimitive;
use solana_program::{
  account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, info,
  pubkey::Pubkey,
};

pub struct Processor {}

impl Processor {
  pub fn process_initialize(
    program_id: &Pubkey,
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

#[cfg(test)]
mod test {
  use super::*;
  use crate::instruction::{initialize, InitArgs};
  use solana_sdk::account::Account;
  use spl_token::processor::Processor as TokenProcessor;

  /// Test program id for the stake-pool program.
  const FUND_PROGRAM_ID: Pubkey = Pubkey::new_from_array([2u8; 32]);

  /// Test program id for the token program.
  const TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array([1u8; 32]);

  /// Actual stake account program id, used for tests
  fn fund_program_id() -> Pubkey {
    "FUND11111111111111111111111111111111111111"
      .parse::<Pubkey>()
      .unwrap()
  }

  struct FundInfo {
    pub fund_key: Pubkey,
    pub fund_account: Account,
    pub max: u64,
    pub owner_key: Pubkey,
    pub mint_key: Pubkey,
    pub mint_account: Account,
  }

  fn create_fund(max: u64) -> FundInfo {
    let fund_key = Pubkey::new_unique();
    let owner_key = Pubkey::new_unique();

    let mut fund_account = Account::new(0, State::LEN, &FUND_PROGRAM_ID);
    let mut owner_account = Account::default();

    let (mint_key, mut mint_account) = create_mint(&TOKEN_PROGRAM_ID, &withdraw_authority_key);

    // StakePool Init
    let _result = do_process_instruction(
      initialize(
        &FUND_PROGRAM_ID,
        &fund_key,
        &owner_key,
        &mint_key,
        &TOKEN_PROGRAM_ID,
        InitArgs { max },
      )
      .unwrap(),
      vec![
        &mut fund_account,
        &mut owner_account,
        &mut mint_account,
        &mut Account::default(),
      ],
    )
    .expect("Error on stake pool initialize");

    FundInfo {
      fund_key: fund_key,
      fund_account: fund_account,
      max,
      owner_key,
      mint_key,
      mint_account,
    }
  }

  #[test]
  fn test_initialize() {
    let fund_info = create_fund(123 as u64);
    // Read account data
    let state = State::deserialize(&fund_info.fund_account.data).unwrap();
    match state {
      State::Unallocated => panic!("Stake pool state is not initialized after init"),
      State::Init(fund) => {
        assert_eq!(fund.max, fund_info.max);

        assert_eq!(fund.owner, fund_info.owner_key);
        assert_eq!(fund.pool_mint, fund_info.mint_key);
        assert_eq!(fund.token_program_id, TOKEN_PROGRAM_ID);

        assert_eq!(fund.stake_total, 0);
        assert_eq!(fund.pool_total, 0);
      }
    }
  }
}
