#![feature(proc_macro_hygiene)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_spl::token::{self, TokenAccount, Transfer};

#[program]
mod squifi {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
    #[access_control(Initialize::accounts(&ctx, nonce))]
    pub fn create_fund(
        ctx: Context<Initialize>,
        authority: Pubkey,
        max_balance: u64,
        nonce: u8,
    ) -> Result<()> {

        let fund = &mut ctx.accounts.fund;
        fund.beneficiary = beneficiary;
        fund.mint = ctx.accounts.fund.mint;
        fund.vault = *ctx.accounts.fund.to_account_info().key;
        fund.period_count = period_count;
        fund.max_balance = max_balance;
        fund.nonce = nonce;

        token::transfer(ctx.accounts.into(), deposit_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Vesting.
    #[account(init)]
    fund: ProgramAccount<'info, Fund>,
    #[account(mut)]
    owner: AccountInfo<'info>,
    #[account(signer)]
    owner_authority: AccountInfo<'info>,
    #[account(mut)]
    vault: CpiAccount<'info, TokenAccount>,
    // Misc.
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    fn accounts(ctx: &Context<Initialize>, nonce: u8) -> Result<()> {
        let vault_authority = Pubkey::create_program_address(
            &[
                ctx.accounts.vesting.to_account_info().key.as_ref(),
                &[nonce],
            ],
            ctx.program_id,
        )
        .map_err(|_| ErrorCode::InvalidProgramAddress)?;
        if ctx.accounts.vault.owner != vault_authority {
            return Err(ErrorCode::InvalidVaultOwner)?;
        }

        Ok(())
    }
}


// Create fund
#[account]
pub struct Fund {
    /// check to see if a fund is ininitialized
    pub initialized: bool,
    /// open defines if a fund is open for deposits
    pub open: bool,
    /// max size of the fund
    pub max_balance: u64,
    /// balance of the
    pub balance: u64,
    /// Nonce of the program account
    pub nonce: u8,
    /// Mint of the SPL token locked up.
    pub mint: Pubkey,
    /// Address of the token vault controlled by the Safe.
    pub vault: Pubkey,
}


#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Copy, Clone)]
pub struct WhitelistEntry {
    pub program_id: Pubkey,
}

#[error]
pub enum ErrorCode {
    #[msg("Invalid program address. Did you provide the correct nonce?")]
    InvalidProgramAddress,
    #[msg("Invalid vault owner.")]
    InvalidVaultOwner,
}
