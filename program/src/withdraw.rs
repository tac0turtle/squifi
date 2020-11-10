use crate::access_control;
use fund::{
  accounts::{
    fund::{Fund, FundType},
    vault::TokenVault,
  },
  error::{FundError, FundErrorCode},
};
use serum_common::pack::Pack;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  info, program,
  pubkey::Pubkey,
};
use spl_token::{instruction, ID};
use std::convert::Into;

pub fn handler(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  amount: u64,
) -> Result<(), FundError> {
  info!("handler: withdraw");

  let acc_infos = &mut accounts.iter();
  let vault_acc_info = next_account_info(acc_infos)?;
  let fund_acc_info = next_account_info(acc_infos)?;
  let withdraw_acc_info = next_account_info(acc_infos)?;
  let vault_authority_acc_info = next_account_info(acc_infos)?;
  let token_program_acc_info = next_account_info(acc_infos)?;
  let nft_mint_acc_info = next_account_info(acc_infos)?;
  let nft_token_acc_info = next_account_info(acc_infos)?;

  access_control(AccessControlRequest {
    program_id,
    amount,
    fund_acc_info,
    withdraw_acc_info,
    vault_acc_info,
    vault_authority_acc_info,
  })?;

  Fund::unpack_mut(
    &mut fund_acc_info.try_borrow_mut_data()?,
    &mut |fund_acc: &mut Fund| {
      state_transistion(StateTransistionRequest {
        accounts,
        fund_acc,
        fund_acc_info,
        withdraw_acc_info,
        vault_acc_info,
        vault_authority_acc_info,
        token_program_acc_info,
        nft_token_acc_info,
        nft_mint_acc_info,
        amount,
      })
      .map_err(Into::into)
    },
  )?;

  Ok(())
}

fn access_control(req: AccessControlRequest) -> Result<(), FundError> {
  let AccessControlRequest {
    program_id,
    amount: _,
    fund_acc_info,
    withdraw_acc_info,
    vault_acc_info,
    vault_authority_acc_info,
  } = req;

  if !withdraw_acc_info.is_signer {
    return Err(FundErrorCode::Unauthorized)?;
  }

  {
    let _ = access_control::fund(fund_acc_info, program_id)?;
    let _ = access_control::vault_join(
      vault_acc_info,
      vault_authority_acc_info,
      fund_acc_info,
      program_id,
    )?;
  }

  let _ = access_control::withdraw(program_id, fund_acc_info, withdraw_acc_info);

  Ok(())
}

fn state_transistion(req: StateTransistionRequest) -> Result<(), FundError> {
  let StateTransistionRequest {
    fund_acc,
    accounts,
    fund_acc_info,
    withdraw_acc_info,
    vault_acc_info,
    vault_authority_acc_info,
    token_program_acc_info,
    nft_mint_acc_info,
    nft_token_acc_info,
    amount,
  } = req;

  {
    if fund_acc.fund_type.eq(&FundType::PublicRaise) {
      let burn_instruction = instruction::burn(
        &spl_token::ID,
        nft_token_acc_info.key,
        nft_mint_acc_info.key,
        &withdraw_acc_info.key,
        &[],
        amount,
      )?;
      program::invoke_signed(&burn_instruction, &accounts[..], &[])?;
    }
  }
  {
    fund_acc.deduct(amount);
    // transfer from program account to owner of fund
    info!("invoking token transfer");
    let withdraw_instruction = instruction::transfer(
      &ID,
      vault_acc_info.key,
      withdraw_acc_info.key,
      &vault_authority_acc_info.key,
      &[],
      amount as u64,
    )?;

    let signer_seeds = TokenVault::signer_seeds(fund_acc_info.key, &fund_acc.nonce);

    program::invoke_signed(
      &withdraw_instruction,
      &[
        vault_acc_info.clone(),
        withdraw_acc_info.clone(),
        vault_authority_acc_info.clone(),
        token_program_acc_info.clone(),
      ],
      &[&signer_seeds],
    )?;
  }

  Ok(())
}

struct AccessControlRequest<'a, 'b> {
  program_id: &'a Pubkey,
  amount: u64,
  fund_acc_info: &'a AccountInfo<'b>,
  withdraw_acc_info: &'a AccountInfo<'b>,
  vault_acc_info: &'a AccountInfo<'b>,
  vault_authority_acc_info: &'a AccountInfo<'b>,
}

struct StateTransistionRequest<'a, 'b, 'c> {
  fund_acc: &'c mut Fund,
  accounts: &'a [AccountInfo<'b>],
  fund_acc_info: &'a AccountInfo<'b>,
  nft_token_acc_info: &'a AccountInfo<'b>,
  nft_mint_acc_info: &'a AccountInfo<'b>,
  withdraw_acc_info: &'a AccountInfo<'b>,
  vault_acc_info: &'a AccountInfo<'b>,
  vault_authority_acc_info: &'a AccountInfo<'b>,
  token_program_acc_info: &'a AccountInfo<'b>,
  amount: u64,
}
