use crate::access_control::token;
use fund::{
  error::{FundError, FundErrorCode},
  types::{Account as FundAccount, Fund, FundType},
};

use serum_common::pack::Pack;
use serum_lockup::accounts::TokenVault;
use solana_sdk::{
  account_info::{next_account_info, AccountInfo},
  info,
  program_pack::Pack as TokenPack,
  pubkey::Pubkey,
};

pub fn initialize<'a>(
  program_id: &'a Pubkey,
  accounts: &'a [AccountInfo],
  owner: Pubkey,
  max_balance: u32,
  fund_type: FundType,
) -> Result<(), FundError> {
  info!("Initialize Fund");

  let account_info_iter = &mut accounts.iter();
  let fund_acc_info = next_account_info(account_info_iter)?;
  let account_acc_info = next_account_info(account_info_iter)?;
  let vault_acc_info = next_account_info(account_info_iter)?;
  let mint_acc_info = next_account_info(account_info_iter)?;

  // Create PrgramAccount

  // 1. Checks
  {
    let _ = Fund::unpack(&fund_acc_info.try_borrow_data()?)?;
    if fund_acc_info.owner != program_id {
      return Err(FundErrorCode::NotOwnedByProgram)?;
    }
  }
  {
    let vault = token(vault_acc_info)?;
    if vault.state != spl_token::state::AccountState::Initialized {
      return Err(FundErrorCode::NotInitialized)?;
    }
    let vault_authority = Pubkey::create_program_address(
      &TokenVault::signer_seeds(account_acc_info.key, &0),
      program_id,
    )
    .map_err(|_| FundErrorCode::InvalidVaultNonce)?;

    if vault.owner != vault_authority {
      return Err(FundErrorCode::InvalidVault)?;
    }
  }

  // 2. Creation
  info!("create program account");
  FundAccount::unpack_mut(
    &mut account_acc_info.try_borrow_mut_data()?,
    &mut |acc: &mut FundAccount| {
      acc.mint = *mint_acc_info.key;
      acc.nounce = 0;
      acc.vault = *vault_acc_info.key;
      Ok(())
    },
  );
  // create a fund
  // 1. Checks
  let _ = Fund::unpack(&fund_acc_info.try_borrow_data()?)?;
  if fund_acc_info.owner != program_id {
    return Err(FundErrorCode::NotOwnedByProgram)?;
  }
  // 2. Creation
  info!("create fund");
  Fund::unpack_mut(
    &mut fund_acc_info.try_borrow_mut_data()?,
    &mut |fund: &mut Fund| {
      fund.fund_type = fund_type;
      fund.owner = owner;
      fund.account = vault_acc_info.key.clone();
      fund.max_balance = max_balance;
      fund.balance = 0; // TODO: with raises we should allow the creator to send funds
      Ok(())
    },
  );

  Ok(())
}
