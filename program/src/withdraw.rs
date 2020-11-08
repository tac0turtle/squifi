use fund::{accounts::fund::Fund, error::FundError};
use serum_common::pack::Pack;
use serum_lockup::accounts::token_vault::TokenVault;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  info,
  program::invoke_signed,
  pubkey::Pubkey,
};
use spl_token::{instruction::transfer, ID};

pub fn handler<'a>(
  progam_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  amount: u32,
) -> Result<(), FundError> {
  info!("handler: withdraw");

  let acc_infos = &mut accounts.iter();

  let fund_owner_acc_info = next_account_info(acc_infos)?;
  let fund_acc_info = next_account_info(acc_infos)?;
  let program_acc_info = next_account_info(acc_infos)?;
  let fund_authority_acc_info = next_account_info(acc_infos)?;
  let token_program_acc_info = next_account_info(acc_infos)?;

  Fund::unpack_mut(
    &mut fund_acc_info.try_borrow_mut_data()?,
    &mut |fund_acc: &mut Fund| {
      fund_acc.deduct(amount);
      // transfer from program account to owner of fund
      info!("invoking token transfer");
      let withdraw_instruction = transfer(
        &ID,
        program_acc_info.key,
        fund_owner_acc_info.key,
        &fund_authority_acc_info.key,
        &[],
        amount as u64,
      )?;

      let fund = Fund::unpack(&fund_acc_info.try_borrow_data()?)?;
      let signer_seeds = TokenVault::signer_seeds(fund_acc_info.key, &fund.nonce);

      invoke_signed(
        &withdraw_instruction,
        &[
          program_acc_info.clone(),
          fund_owner_acc_info.clone(),
          fund_authority_acc_info.clone(),
          token_program_acc_info.clone(),
        ],
        &[&signer_seeds],
      )?;
      Ok(())
    },
  )?;

  Ok(())
}

fn access_control() -> Result<(), FundError> {
  // validate
  // 1. check owner of fund is owner in message
  // 2. check balance is greater than x of
  Ok(())
}
fn state_transistion(req: StateTransistionRequest) -> Result<(), FundError> {
  fund_acc.deduct(amount);
  // transfer from program account to owner of fund
  info!("invoking token transfer");
  let withdraw_instruction = transfer(
    &ID,
    program_acc_info.key,
    fund_owner_acc_info.key,
    &fund_authority_acc_info.key,
    &[],
    amount as u64,
  )?;

  let fund = Fund::unpack(&fund_acc_info.try_borrow_data()?)?;
  let signer_seeds = TokenVault::signer_seeds(fund_acc_info.key, &fund.nonce);

  invoke_signed(
    &withdraw_instruction,
    &[
      program_acc_info.clone(),
      fund_owner_acc_info.clone(),
      fund_authority_acc_info.clone(),
      token_program_acc_info.clone(),
    ],
    &[&signer_seeds],
  )?;
  Ok(())
}

struct AccessControlRequest<'a> {
  program_is: &'a Pubkey,
}

struct StateTransistionRequest<'a> {}
