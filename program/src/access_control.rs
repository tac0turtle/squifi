use fund::error::{FundError, FundErrorCode};
use serum_common::pack::Pack;
use solana_sdk::{account_info::AccountInfo, program_pack::Pack as TokenPack};

use spl_token::state::Account as TokenAccount;
use std::convert::Into;

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
