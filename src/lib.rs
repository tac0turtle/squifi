use crate::types::FundType;
use serde::{Deserialize, Serialize};
use serum_common::pack::*;
use solana_client_gen::prelude::*;

pub mod error;
pub mod types;

#[cfg_attr(feature = "client", solana_client_gen)]
pub mod instruction {
  use super::*;
  #[derive(Serialize, Deserialize)]
  pub enum FundInstruction {
    /// Initializes a new Fund & Fund Account
    ///
    /// [writable] Fund to create
    /// [writable] Account to create
    /// [writable] Program controlled tokenvault.
    /// []Mint
    Initialize {
      /// Owner of the Fund
      owner: Pubkey,
      /// Authority of the Fund
      authority: Pubkey,
      /// Max Size of a fund
      max_balance: u32,
      /// fund type
      fund_type: FundType,
    },
  }
}

serum_common::packable!(instruction::FundInstruction);
