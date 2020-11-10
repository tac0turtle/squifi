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
  info!("process deposit");

  let acc_infos = &mut accounts.iter();

  let vault_acc_info = next_account_info(acc_infos)?;
  let depositor_acc_info = next_account_info(acc_infos)?;
  let depositor_authority_acc_info = next_account_info(acc_infos)?;
  let fund_acc_info = next_account_info(acc_infos)?;
  let vault_authority_acc_info = next_account_info(acc_infos)?;
  let token_program_acc_info = next_account_info(acc_infos)?;
  let nft_mint_acc_info = next_account_info(acc_infos)?;
  let nft_token_acc_info = next_account_info(acc_infos)?;

  access_control(AccessControlRequest {
    program_id,
    amount,
    fund_acc_info,
    depositor_authority_acc_info,
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
        depositor_acc_info,
        depositor_authority_acc_info,
        vault_acc_info,
        vault_authority_acc_info,
        token_program_acc_info,
        nft_mint_acc_info,
        nft_token_acc_info,
        amount,
      })
      .map_err(Into::into)
    },
  )?;

  Ok(())
}

fn access_control(req: AccessControlRequest) -> Result<(), FundError> {
  // 1. correct fund
  // 2. correct account
  // 3. if provided deposit_amount + balance < max_balance

  let AccessControlRequest {
    program_id,
    amount: _, //todo check amount
    fund_acc_info,
    depositor_authority_acc_info,
    vault_acc_info,
    vault_authority_acc_info,
  } = req;

  if !depositor_authority_acc_info.is_signer {
    return Err(FundErrorCode::Unauthorized)?;
  }

  // let rent = access_control::rent(rent_acc_info)?;
  let _ = access_control::fund(fund_acc_info, program_id)?;
  let _ = access_control::vault_join(
    vault_acc_info,
    vault_authority_acc_info,
    fund_acc_info,
    program_id,
  )?;

  Ok(())
}
fn state_transistion(req: StateTransistionRequest) -> Result<(), FundError> {
  let StateTransistionRequest {
    accounts,
    fund_acc,
    fund_acc_info,
    depositor_acc_info,
    vault_acc_info,
    vault_authority_acc_info,
    depositor_authority_acc_info,
    token_program_acc_info,
    nft_mint_acc_info,
    nft_token_acc_info,
    amount,
  } = req;

  {
    if fund_acc.fund_type.eq(&FundType::PublicRaise) {
      let mint_to_instr = instruction::mint_to(
        &spl_token::ID,
        nft_mint_acc_info.key,
        nft_token_acc_info.key,
        vault_authority_acc_info.key,
        &[],
        amount,
      )?;

      let signer_seeds = TokenVault::signer_seeds(fund_acc_info.key, &fund_acc.nonce);

      program::invoke_signed(&mint_to_instr, &accounts[..], &[&signer_seeds])?;
    }
  }

  fund_acc.add(amount);
  // Send tokens from depositor to fund account.
  info!("SPL token transfer");
  // Now transfer SPL funds from the depositor, to the
  // program-controlled account.
  {
    info!("invoke SPL token transfer");
    let deposit_instruction = instruction::transfer(
      &ID,
      depositor_acc_info.key,
      vault_acc_info.key,
      depositor_authority_acc_info.key,
      &[],
      amount as u64,
    )?;
    program::invoke_signed(
      &deposit_instruction,
      &[
        depositor_acc_info.clone(),
        depositor_authority_acc_info.clone(),
        vault_acc_info.clone(),
        token_program_acc_info.clone(),
      ],
      &[],
    )?;

    Ok(())
  }
}

struct AccessControlRequest<'a, 'b> {
  program_id: &'a Pubkey,
  amount: u64,
  fund_acc_info: &'a AccountInfo<'b>,
  depositor_authority_acc_info: &'a AccountInfo<'b>,
  vault_acc_info: &'a AccountInfo<'b>,
  vault_authority_acc_info: &'a AccountInfo<'b>,
}

struct StateTransistionRequest<'a, 'b, 'c> {
  accounts: &'a [AccountInfo<'b>],
  fund_acc: &'c mut Fund,
  fund_acc_info: &'a AccountInfo<'b>,
  depositor_acc_info: &'a AccountInfo<'b>,
  depositor_authority_acc_info: &'a AccountInfo<'b>,
  vault_acc_info: &'a AccountInfo<'b>,
  vault_authority_acc_info: &'a AccountInfo<'b>,
  token_program_acc_info: &'a AccountInfo<'b>,
  nft_token_acc_info: &'a AccountInfo<'b>,
  nft_mint_acc_info: &'a AccountInfo<'b>,
  amount: u64,
}
