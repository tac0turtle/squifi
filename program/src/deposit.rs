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

pub fn deposit<'a>(
  progam_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  amount: u32,
) -> Result<(), FundError> {
  info!("process deposit");

  let acc_infos = &mut accounts.iter();

  let program_acc_info = next_account_info(acc_infos)?;
  let depositor_acc_info = next_account_info(acc_infos)?;
  let depositor_authority_acc_info = next_account_info(acc_infos)?;
  let fund_acc_info = next_account_info(acc_infos)?;
  let token_program_acc_info = next_account_info(acc_infos)?;

  // Validate

  // 1. correct fund
  // 2. correct account
  // 3. if provided deposit_amount + balance < max_balance

  Fund::unpack_mut(
    &mut fund_acc_info.try_borrow_mut_data()?,
    &mut |fund_acc: &mut Fund| {
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
          program_acc_info.key,
          depositor_authority_acc_info.key,
          &[],
          amount as u64,
        )?;
        invoke_signed(
          &deposit_instruction,
          &[
            depositor_acc_info.clone(),
            depositor_authority_acc_info.clone(),
            program_acc_info.clone(),
            token_program_acc_info.clone(),
          ],
          &[],
        )?;
        Ok(())
      }
    },
  );

  Ok(())
}
