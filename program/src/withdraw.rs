/// [writable] fund owner
/// [writable] fund to withrdraw from
/// [writable] program account
/// [] fund authority
/// [] SPL token program
// Withdraw { amount: u32 },

pub fn withdraw(
  progam_id: &'a Pubkey,
  accounts: &'a [AccountInfo<'a>],
  withdraw_amount: u32,
) -> Result<(), FundError> {
  info!("process withdraw");

  let acc_infos = &mut accounts.iter();

  let fund_owner_acc_info = next_account_info(acc_infos)?;
  let fund_acc_info = next_account_info(acc_infos)?;
  let program_acc_info = next_account_info(acc_infos)?;
  let fund_authority_acc_info = next_account_info(acc_infos)?;
  let token_program_info = next_account_info(acc_infos)?;

  // validate
  // 1. check owner of fund is owner in message
  // 2. check balance is greater than x of

  // transfer from program account to owner of fund
  {
    info!("invoking token transfer");
    let withdraw_instruction = transfer(
      &spl_token::ID,
      program_acc_info.key,
      beneficiary_token_acc_info.key,
      &fund_authority_acc_info.key,
      &[],
      amount,
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
  }

  Ok(())
}
