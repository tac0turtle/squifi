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
    /// 2. `[writable]` Account to create
    /// 3. `[writable]` Program controlled tokenvault.
    /// 4. `[]`         Mint
    /// 5. `[]` todo add rent sysvar
    Initialize {
      /// Owner of the Fund
      owner: Pubkey,
      /// Authority of the Fund
      authority: Pubkey,
      /// Max Size of a fund
      max_balance: u32,
      /// fund type
      fund_type: accounts::fund::FundType,
    },
    /// Deposit sends tokens to a fund.
    ///
    /// `[writable]` Prgram controlled token vault
    /// `[writable]` Depositor token account
    /// `[signer]`   Depositor authority
    /// `[]`         Fund
    /// `[]`         Fund Authority
    /// `[]`         SPL token program
    Deposit { amount: u32 },
    /// Withdraw funds from program account.
    ///
    /// `[writable]` Fund owner
    /// `[writable]` Fund to withrdraw from
    /// `[writable]` Program controlled tokenvault
    /// `[]` Fund Authority
    /// `[]` SPL token program
    Withdraw { amount: u32 },
  }
}

serum_common::packable!(instruction::FundInstruction);
