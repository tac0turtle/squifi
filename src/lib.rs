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
  }
}

serum_common::packable!(instruction::FundInstruction);
