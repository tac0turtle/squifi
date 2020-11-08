//! Program state processor

use crate::access_control::token;
use fund::{
  accounts::fund::{Fund, FundType},
  error::{FundError, FundErrorCode},
};
use serum_common::pack::Pack;
use serum_lockup::accounts::TokenVault;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  info,
  program_pack::Pack as TokenPack,
  pubkey::Pubkey,
};
use std::convert::Into;

pub fn handler<'a>(
  program_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  owner: Pubkey,
  authority: Pubkey,
  max_balance: u64,
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

  access_control(AccessControlRequest {
    program_id,
    fund_acc_info,
    mint_acc_info,
    vault_acc_info,
    nonce: 0,
  })?;

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
    &mut |fund_acc: &mut Fund| {
      state_transition(StateTransitionRequest {
        fund_acc,
        owner,
        authority,
        mint: mint_acc_info.key,
        vault: *vault_acc_info.key,
        fund_type,
        nonce: 0,
        max_balance,
      })
      .map_err(Into::into)
    },
  );

  Ok(())
}

fn access_control<'a>(req: AccessControlRequest<'a>) -> Result<(), FundError> {
  info!("access-control: initialize");

  let AccessControlRequest {
    program_id,
    fund_acc_info,
    mint_acc_info,
    vault_acc_info,
    nonce,
  } = req;

  // {
  //   let _ = Fund::unpack(&fund_acc_info.try_borrow_data()?)?;
  //   if fund_acc_info.owner != program_id {
  //     return Err(FundErrorCode::NotOwnedByProgram)?;
  //   }
  // }
  {
    let vault = token(vault_acc_info)?;
    if vault.state != spl_token::state::AccountState::Initialized {
      return Err(FundErrorCode::NotInitialized)?;
    }
    let vault_authority =
      Pubkey::create_program_address(&TokenVault::signer_seeds(fund_acc_info.key, &0), program_id)
        .map_err(|_| FundErrorCode::InvalidVaultNonce)?;

    if vault.owner != vault_authority {
      return Err(FundErrorCode::InvalidVault)?;
    }
  }
  Ok(())
}

fn state_transition<'a>(req: StateTransitionRequest<'a>) -> Result<(), FundError> {
  info!("state-transition: initialize");

  let StateTransitionRequest {
    fund_acc,
    owner,
    authority,
    vault,
    mint,
    fund_type,
    nonce,
    max_balance,
  } = req;

  fund_acc.open = true;
  fund_acc.owner = owner;
  fund_acc.authority = authority;
  fund_acc.vault = vault;
  fund_acc.mint = *mint;
  fund_acc.max_balance = max_balance;
  fund_acc.balance = 0;
  fund_acc.fund_type = fund_type;
  fund_acc.nonce = nonce;

  info!("state-transition: success");

  Ok(())
}

struct AccessControlRequest<'a> {
  program_id: &'a Pubkey,
  fund_acc_info: &'a AccountInfo<'a>,
  mint_acc_info: &'a AccountInfo<'a>,
  vault_acc_info: &'a AccountInfo<'a>,
  nonce: u8,
}

struct StateTransitionRequest<'a> {
  fund_acc: &'a mut Fund,
  owner: Pubkey,
  mint: &'a Pubkey,
  vault: Pubkey,
  authority: Pubkey,
  fund_type: FundType,
  nonce: u8,
  max_balance: u64,
}
