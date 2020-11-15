use crate::access_control;
use fund::{
    accounts::{
        fund::{Fund, FundType},
        vault::TokenVault,
    },
    error::{FundError, FundErrorCode},
};
use serum_common::pack::Pack;
use solana_program::program_pack::Pack as TokenPack;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    info, program,
    pubkey::Pubkey,
};
use spl_token::instruction;
use spl_token::state::Account;
use std::convert::Into;

pub fn handler(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Result<(), FundError> {
    info!("Handler: payback_withdraw");
    let acc_infos = &mut accounts.iter();

    let token_program_acc_info = next_account_info(acc_infos)?;
    let nft_program_acc_info = next_account_info(acc_infos)?;
    let payback_vault_authority_acc_info = next_account_info(acc_infos)?;
    let payback_vault_acc_info = next_account_info(acc_infos)?;
    let fund_acc_info = next_account_info(acc_infos)?;
    let withdraw_acc_info = next_account_info(acc_infos)?;
    let nft_withdraw_acc_info = next_account_info(acc_infos)?;

    access_control(AccessControlRequest {
        program_id,
        fund_acc_info,
        withdraw_acc_info,
        nft_program_acc_info,
        nft_withdraw_acc_info,
        amount,
    })?;

    Fund::unpack_mut(
        &mut fund_acc_info.try_borrow_mut_data()?,
        &mut |fund_acc: &mut Fund| {
            state_transistion(StateTransistionRequest {
                fund_acc,
                fund_acc_info,
                withdraw_acc_info,
                payback_vault_acc_info,
                payback_vault_authority_acc_info,
                token_program_acc_info,
                // nft_withdraw_acc_info,
                amount,
            })
            .map_err(Into::into)
        },
    )?;
    Ok(())
}

fn access_control(req: AccessControlRequest) -> Result<(), FundError> {
    let AccessControlRequest {
        program_id,
        fund_acc_info,
        withdraw_acc_info,
        nft_program_acc_info,
        nft_withdraw_acc_info,
        amount,
    } = req;

    if !withdraw_acc_info.is_signer {
        return Err(FundErrorCode::Unauthorized.into());
    }

    let fund = access_control::fund(fund_acc_info, program_id)?;

    if !fund.fund_type.eq(&FundType::Raise {
        private: (true || false),
    }) {
        return Err(FundErrorCode::InvalidFund.into());
    }

    let token_acc = access_control::token(nft_program_acc_info)?;

    if token_acc.mint != fund.nft_mint {
        return Err(FundErrorCode::InvalidTokenAccountMint.into());
    }

    let dest_account = Account::unpack(&nft_withdraw_acc_info.data.borrow())?;

    if dest_account.amount >= amount {
        return Err(FundErrorCode::InvalidPayBackWithdrawlAddress.into());
    }

    if amount > fund.shares {
        return Err(FundErrorCode::WithdrawlSizeOverflow.into());
    }

    let _ = access_control::fund(fund_acc_info, program_id)?;

    Ok(())
}

fn state_transistion(req: StateTransistionRequest) -> Result<(), FundError> {
    let StateTransistionRequest {
        fund_acc,
        fund_acc_info,
        withdraw_acc_info,
        payback_vault_acc_info,
        payback_vault_authority_acc_info,
        token_program_acc_info,
        // nft_withdraw_acc_info: _,
        amount,
    } = req;

    info!("State-Transistion: Withdraw Payback");

    {
        // let nft_account_data = Account::unpack(&nft_withdraw_acc_info.data.borrow())?;
        // let amount = fund_acc.get_share_dist(); // todo handle

        // transfer from program account to owner of fund
        info!("invoking token transfer");
        let withdraw_instruction = instruction::transfer(
            &spl_token::ID,
            payback_vault_acc_info.key,
            withdraw_acc_info.key,
            &payback_vault_authority_acc_info.key,
            &[],
            amount, //todo current mapping is 1:1, not ideal
        )?;

        let signer_seeds = TokenVault::signer_seeds(fund_acc_info.key, &fund_acc.nonce);

        program::invoke_signed(
            &withdraw_instruction,
            &[
                payback_vault_acc_info.clone(),
                withdraw_acc_info.clone(),
                payback_vault_authority_acc_info.clone(),
                token_program_acc_info.clone(),
            ],
            &[&signer_seeds],
        )?;
    }

    info!("State-Transistion: Withdraw Payback Success");
    Ok(())
}

struct AccessControlRequest<'a, 'b> {
    program_id: &'a Pubkey,
    fund_acc_info: &'a AccountInfo<'b>,
    withdraw_acc_info: &'a AccountInfo<'b>,
    nft_program_acc_info: &'a AccountInfo<'b>,
    nft_withdraw_acc_info: &'a AccountInfo<'b>,
    amount: u64,
}

struct StateTransistionRequest<'a, 'b> {
    fund_acc: &'a mut Fund,
    fund_acc_info: &'a AccountInfo<'b>,
    withdraw_acc_info: &'a AccountInfo<'b>,
    payback_vault_acc_info: &'a AccountInfo<'b>,
    payback_vault_authority_acc_info: &'a AccountInfo<'b>,
    token_program_acc_info: &'a AccountInfo<'b>,
    // nft_withdraw_acc_info: &'a AccountInfo<'b>,
    amount: u64,
}
