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
    /// [] SPL token program
    /// []Mint
    Initialize {
      /// Owner of the Fund
      owner: Pubkey, // Optional in the future for when gov spending is implemented?
      /// Max Size of a fund
      max_balance: u32,
      /// fund type
      fund_type: FundType,
    },
    /// Deposit sends tokens to a fund.
    ///
    /// [writable] Prgram account
    /// [writable] Depositor
    /// [signer] Depositor authority
    /// [] Fund
    /// [] SPL token program
    Deposit { amount: u32 },
    /// Withdraw funds from program account.
    ///
    /// [writable] fund owner
    /// [writable] fund to withrdraw from
    /// [writable] program accounr
    /// [] fund authority
    /// [] SPL token program
    Withdraw { amount: u32 },
  }
}

serum_common::packable!(instruction::FundInstruction);
