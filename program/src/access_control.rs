use fund::{
  accounts::{fund::Fund, vault::TokenVault},
  error::{FundError, FundErrorCode},
};
use serum_common::pack::Pack;
use solana_program::{account_info::AccountInfo, program_pack::Pack as TokenPack, pubkey::Pubkey};

use spl_token::state::{Account as TokenAccount, Mint};

pub fn token(acc_info: &AccountInfo) -> Result<TokenAccount, FundError> {
  if *acc_info.owner != spl_token::ID {
    return Err(FundErrorCode::InvalidAccountOwner)?;
  }

  let token = TokenAccount::unpack(&acc_info.try_borrow_data()?)?;
  if token.state != spl_token::state::AccountState::Initialized {
    return Err(FundErrorCode::NotInitialized)?;
  }

  Ok(token)
}

pub fn fund(acc_info: &AccountInfo, program_id: &Pubkey) -> Result<Fund, FundError> {
  if acc_info.owner != program_id {
    return Err(FundErrorCode::InvalidAccountOwner)?;
  }

  let fund = Fund::unpack(&acc_info.try_borrow_data()?)?;
  if !fund.initialized {
    return Err(FundErrorCode::NotInitialized)?;
  }

  Ok(fund)
}

pub fn mint(acc_info: &AccountInfo) -> Result<Mint, FundError> {
  if *acc_info.owner != spl_token::ID {
    return Err(FundErrorCode::InvalidMint)?;
  }
  let mint = Mint::unpack(&acc_info.try_borrow_data()?)?;
  if !mint.is_initialized {
    return Err(FundErrorCode::UnitializedTokenMint)?;
  }

  Ok(mint)
}

pub fn vault(acc_info: &AccountInfo, fund: &Fund) -> Result<TokenAccount, FundError> {
  let vault = token(acc_info)?;
  if *acc_info.key != fund.vault {
    return Err(FundErrorCode::InvalidVault)?;
  }
  Ok(vault)
}

pub fn vault_join(
  acc_info: &AccountInfo,
  vault_authority_acc_info: &AccountInfo,
  fund_acc_info: &AccountInfo,
  program_id: &Pubkey,
) -> Result<TokenAccount, FundError> {
  let fund = fund(fund_acc_info, program_id)?;
  let vault = vault(acc_info, &fund)?;
  let va = vault_authority(
    vault_authority_acc_info,
    fund_acc_info.key,
    &fund,
    program_id,
  )?;

  if va != vault.owner {
    return Err(FundErrorCode::InvalidVault)?;
  }
  if va != *vault_authority_acc_info.key {
    return Err(FundErrorCode::InvalidVault)?;
  }

  Ok(vault)
}

pub fn vault_authority(
  vault_authority_acc_info: &AccountInfo,
  fund_addr: &Pubkey,
  fund: &Fund,
  program_id: &Pubkey,
) -> Result<Pubkey, FundError> {
  let va = Pubkey::create_program_address(
    &TokenVault::signer_seeds(fund_addr, &fund.nonce),
    program_id,
  )
  .map_err(|_| FundErrorCode::InvalidVaultNonce)?;
  if va != *vault_authority_acc_info.key {
    return Err(FundErrorCode::InvalidVault)?;
  }

  Ok(va)
}

pub fn withdraw(
  program_id: &Pubkey,
  fund_acc_info: &AccountInfo,
  withdraw_acc_beneficiary_info: &AccountInfo,
) -> Result<Fund, FundError> {
  let fund = Fund::unpack(&fund_acc_info.try_borrow_data()?)?;

  if fund_acc_info.owner != program_id {
    return Err(FundErrorCode::InvalidAccount)?;
  }
  if !fund.initialized {
    return Err(FundErrorCode::NotInitialized)?;
  }
  if fund.owner != *withdraw_acc_beneficiary_info.key {
    return Err(FundErrorCode::Unauthorized)?;
  }

  Ok(fund)
}

pub fn check_balance(fund_acc_info: &AccountInfo, amount: u64) -> Result<(), FundError> {
  let fund = Fund::unpack(&fund_acc_info.try_borrow_data()?)?;

  if fund.balance + amount > fund.max_balance {
    return Err(FundErrorCode::FundBalanceOverflow)?;
  }

  Ok(())
}
