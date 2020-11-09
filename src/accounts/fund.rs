use serde::{Deserialize, Serialize};
use serum_common::pack::*;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FundType {
  /// similar to a gofundme
  FundMe,
  // Raise,
}

/// The Owner of the fund has the right to withdraw all or some of the funds
#[derive(Debug, Serialize, Deserialize)]
pub struct Fund {
  /// check to see if a fund is ininitialized
  pub initialized: bool,
  /// open defines if a fund is open for deposits
  pub open: bool,
  /// type of fund
  pub fund_type: FundType,
  /// fund Owner
  pub owner: Pubkey,
  /// Owner authority
  pub authority: Pubkey,
  /// max size of the fund
  pub max_balance: u64,
  /// balance of the
  pub balance: u64,
  /// Nonce of the program account
  pub nonce: u8,
  /// Mint
  pub mint: Pubkey,
  /// Address of the token vault controlled by the Safe.
  pub vault: Pubkey,
}

impl Fund {
  pub fn deduct(&mut self, amount: u64) {
    if self.balance > 0 {
      self.balance -= amount;
    }
  }
  pub fn add(&mut self, amount: u64) {
    self.balance += amount;
  }
}

serum_common::packable!(Fund);
