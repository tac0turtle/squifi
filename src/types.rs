use serde::{Deserialize, Serialize};
use serum_common::pack::*;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum FundType {
  /// similar to a gofundme
  FundMe,
  Raise, //TODO implement
}

/// Initialized program details.
/// Fund is a program account.
/// The Owner of the fund has the right to withdraw all or some of the funds
#[derive(Debug, Serialize, Deserialize)]
pub struct Fund {
  /// open defines if deposits are still allowed or if the fund is empty.
  pub open: bool,
  /// type of fund
  pub fund_type: FundType,
  /// fund Owner
  pub owner: Pubkey,
  /// accont represents an program key to hold spl tokens.
  pub account: Pubkey,
  /// max size of the fund
  pub max_balance: u32,
  /// balance of the
  pub balance: u32,
}

impl Fund {
  pub fn deduct(&mut self, amount: u32) {
    self.balance -= amount;
  }
  pub fn add(&mut self, amount: u32) {
    self.balance += amount;
  }

  pub fn try_close_empty(&mut self) -> bool {
    if self.balance == 0 {
      self.open = false;
      true
    } else {
      false
    }
  }
}

serum_common::packable!(Fund);

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Account {
  /// Mint
  pub mint: Pubkey,
  /// Nounce of the program account
  pub nounce: u8,
  /// Address of the token vault controlled by the Safe.
  pub vault: Pubkey,
}

serum_common::packable!(Account);
