//! Program state processor

use crate::{
  error::FundError,
  instruction::InitArgs,
  state::{Account, Fund, FundType},
};
use serum_lockup::accounts::TokenVault;
use solana_program::{
  account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, info,
  pubkey::Pubkey,
};
use spl_token::state::Account as SPLAccount;

pub fn initialize(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  init_args: InitArgs,
) -> ProgramResult {
  info!("Initialize Fund");

  let account_info_iter = &mut accounts.iter();
  let fund_acc_info = next_account_info(account_info_iter)?;
  let account_acc_info = next_account_info(account_info_iter)?;
  let vault_acc_info = next_account_info(account_info_iter)?;
  let token_program_acc_info = next_account_info(account_info_iter)?;
  let mint_acc_info = next_account_info(account_info_iter)?;

  // Create PrgramAccount

  // 1. Checks
  {
    let fund = Fund::unpack(&fund_acc_info.try_borrow_data()?)?;
    if fund_acc_info.owner != program_id {
      return Err(FundError::NotOwnedByProgram)?;
    }

    let vault = SPLAccount::unpack(&vault_acc_info.try_borrow_data()?)?;
    if vault.state != spl_token::state::AccountState::Initialized {
      return Err(FundError::NotInitialized)?;
    }
    let vault_authority = Pubkey::create_program_address(
      &TokenVault::signer_seeds(account_acc_info.key, 0),
      program_id,
    )
    .map_err(|_| FundError::InvalidVaultNonce)?;

    if vault.owner != vault_authority {
      return Err(FundError::InvalidVault)?;
    }
  }

  // 2. Creation
  info!("create program account")
  Account::unpack_mut(
    &mut account_acc_info.try_borrow_mut_data()?,
    &mut |acc: &mut Account| {
      acc.mint = mint_acc_info;
      acc.nounce = 0;
      acc.vault = vault_acc_info.key;
    },
  );
  // create a fund
  
  // 1. Checks
  
  let vesting = Fund::unpack(&fund_acc_info.try_borrow_data()?)?;
  
  if fund_acc_info.owner != program_id {
    return Err(FundError::NotOwnedByProgram)?;
  }
  
  // 2. Creation
  info!("create fund")
  Fund::unpack_mut(
    &mut fund_acc_info.try_borrow_mut_data()?,
    &mut |fund: &mut Fund| {
      fund.fund_type = init_args.fund_type;
      fund.owner = init_args.owner;
      fund.account = vault_acc_info.key.clone();
      fund.max_balance = init_args.max_balance;
      fund.balance = 0; // TODO: with raises we should allow the creator to send funds
    },
  );

  Ok(())
}
