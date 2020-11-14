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
        /// 0. `[writable]` Fund to create
        /// 1. `[writable]` Tokenvault.
        /// 2. `[]`         Mint
        /// 3. `[]`         Rent sysvar
        /// 4. `[writable]` Whitelist to initialize.
        /// 5. `[writable]` Token mint representing the investment receipt.
        /// 6. `[writable]` Token account associated with the mint.
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
        /// 0. `[writable]` Tokenvault
        /// 1. `[writable]` Depositor token account
        /// 2. `[signer]`   Depositor authority
        /// 3. `[]`         Fund
        /// 4. `[]`         Tokenvault Authority
        /// 5. `[]`         SPL token program
        /// 6. `[writable]` Token mint representing the investment receipt.
        /// 7  `[writable]` Token account associated with the mint.
        /// 8. `[]`         Whitelist
        Deposit { amount: u64 },
        /// Withdraw funds from program account.
        ///
        /// 0. `[writable]` Tokenvault
        /// 1. `[writable]` Fund to transfer tokens out of
        /// 2. `[signer]`   Account to withdraw to
        /// 3. `[]`         Fund Authority
        /// 4. `[]`         SPL token program
        Withdraw { amount: u64 },
        /// Close fund, prohibit deposits
        ///
        /// 0. `[writable]` Fund
        /// 2. `[signer]`   FundOwner
        Close,
        /// Add a new entry to the Whitelist of a fund.
        ///
        /// 0. `[writable]` Fund
        /// 1. `[signer]`   FundOwner
        /// 2. `[writable]` whitelist
        WhitelistAdd { entry: Pubkey },
        /// Removes an entry from the funds Whitelist.
        ///
        /// 0. `[writable]` Fund
        /// 1. `[signer]`   FundOwner
        /// 2. `[writable]` whitelist
        WhitelistDelete { entry: Pubkey },
        /// InitilaizePayback creates a program address to pay back token holders
        ///
        /// 0. `[writable]` Payback Tokenvault (new).
        /// 1. `[writable]` Fund to transfer tokens out of
        /// 2. `[writable]` Depositor token account.
        /// 3. `[signer]`   Depositor auhtority.
        /// 4. `[]`         Token Program Account
        InitializePayback { amount: u64 },
        /// PayBack Withdrwal allows the holder of the Fund NFT to withdraw rewards.
        ///
        /// 0. `[]`         SPL Token program
        /// 1. `[]`         Fund SPL NFT program
        /// 2. `[]`         Payback Vault Authority
        /// 4. `[writable]` Payback Vault
        /// 3. `[writable]` Fund
        /// 5. `[]`         Withdraw Account
        PaybackWithdraw { amount: u64 },
        /// PayBack Deposit allows the fund owner to deposit more into the payback vault.
        ///
        /// 0. `[]`         SPL Token program
        /// 1. `[]`         Fund SPL NFT program
        /// 2. `[]`         Payback Vault Authority
        /// 4. `[writable]` Payback Vault
        /// 3. `[writable]` Fund
        /// 5. `[]`         Withdraw Account
        PaybackDeposit { amount: u64 },
    }
}

serum_common::packable!(instruction::FundInstruction);
