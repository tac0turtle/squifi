use serde::{Deserialize, Serialize};
use serum_common::pack::*;
use solana_client_gen::prelude::*;

pub mod accounts;
pub mod error;

#[cfg_attr(feature = "client", solana_client_gen)]
pub mod instruction {
  use super::*;
  #[derive(Serialize, Deserialize)]
  pub enum FundInstruction {
    /// Initializes a new Fund & Fund Account
    ///
    /// 0. `[]`         Vault.
    /// 1. `[writable]` Fund to create
    /// 2. `[writable]` Program controlled tokenvault.
    /// 3. `[]`         Mint
    /// 4. `[]` todo add rent sysvar
    Initialize {
      /// Owner of the Fund
      owner: Pubkey,
      /// Authority of the Fund
      authority: Pubkey,
      /// Max Size of a fund
      max_balance: u64,
      /// fund type
      fund_type: accounts::fund::FundType,
    },
    /// Deposit sends tokens to a fund.
    ///
    /// 0. `[writable]` Vault
    /// 1. `[writable]` Depositor token account
    /// 2. `[signer]`   Depositor authority
    /// 3. `[]`         Fund
    /// 4. `[]`         Vault Authority
    /// 4. `[]`         SPL token program
    Deposit { amount: u64 },
    /// Withdraw funds from program account.
    ///
    /// 0. `[writable]` Tokenvault
    /// 1. `[writable]` Fund to transfer tokens out of
    /// 2. `[]`         Account to withdraw to
    /// 3. `[]`         Fund Authority
    /// 4. `[]`         SPL token program
    Withdraw { amount: u64 },
  }
}

serum_common::packable!(instruction::FundInstruction);
