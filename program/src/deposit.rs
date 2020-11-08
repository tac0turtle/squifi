use fund::{accounts::fund::Fund, error::FundError};
use serum_common::pack::Pack;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  info,
  program::invoke_signed,
  program_pack::Pack as TokenPack,
  pubkey::Pubkey,
};
use spl_token::{instruction::transfer, ID};
use std::convert::Into;

pub fn handler(progam_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> Result<(), FundError> {
  info!("process deposit");

  let acc_infos = &mut accounts.iter();

  let vault_acc_info = next_account_info(acc_infos)?;
  let depositor_acc_info = next_account_info(acc_infos)?;
  let depositor_authority_acc_info = next_account_info(acc_infos)?;
  let fund_acc_info = next_account_info(acc_infos)?;
  let token_program_acc_info = next_account_info(acc_infos)?;

  Fund::unpack_mut(
    &mut fund_acc_info.try_borrow_mut_data()?,
    &mut |fund_acc: &mut Fund| {
      state_transistion(StateTransistionRequest {
        fund_acc,
        depositor_acc_info,
        depositor_authority_acc_info,
        vault_acc_info,
        token_program_acc_info,
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

  // let AccessControlRequest {} = req;

  Ok(())
}
fn state_transistion(req: StateTransistionRequest) -> Result<(), FundError> {
  let StateTransistionRequest {
    fund_acc,
    depositor_acc_info,
    vault_acc_info,
    depositor_authority_acc_info,
    token_program_acc_info,
    amount,
  } = req;

  fund_acc.add(amount);
  // Send tokens from depositor to fund account.
  info!("SPL token transfer");
  // Now transfer SPL funds from the depositor, to the
  // program-controlled account.
  {
    info!("invoke SPL token transfer");
    let deposit_instruction = transfer(
      &ID,
      depositor_acc_info.key,
      vault_acc_info.key,
      depositor_authority_acc_info.key,
      &[],
      amount as u64,
    )?;
    invoke_signed(
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
  fund_acc: &'c mut Fund,
  depositor_acc_info: &'a AccountInfo<'b>,
  depositor_authority_acc_info: &'a AccountInfo<'b>,
  vault_acc_info: &'a AccountInfo<'b>,
  token_program_acc_info: &'a AccountInfo<'b>,
  amount: u64,
}
