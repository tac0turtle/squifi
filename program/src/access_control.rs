use fund::{
  error::{FundError, FundErrorCode},
  types::Fund,
};
use serum_common::pack::Pack;
use solana_sdk::{account_info::AccountInfo, program_pack::Pack as TokenPack, pubkey::Pubkey};

use spl_token::state::Account as TokenAccount;

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
  // if !fund.initialized {
  //   return Err(FundErrorCode::NotInitialized)?;
  // }

  Ok(fund)
}

// pub fn account(
//   acc_info: &AccountInfo,
//   vault_authority_acc_info: &AccountInfo,
//   safe_acc_info: &AccountInfo,
//   program_id: &Pubkey,
// ) -> Result<TokenAccount, FundError> {
//   let fund = fund(safe_acc_info, program_id)?;
//   let account = token(acc_info)?;
//   if *acc_info.key != safe.vault {
//     return Err(FundErrorCode::InvalidVault)?;
//   }

//   let va = vault_authority(
//     vault_authority_acc_info,
//     safe_acc_info.key,
//     &safe,
//     program_id,
//   )?;

//   if va != vault.owner {
//     return Err(FundErrorCode::InvalidVault)?;
//   }
//   if va != *vault_authority_acc_info.key {
//     return Err(FundErrorCode::InvalidVault)?;
//   }

//   Ok(vault)
// }
