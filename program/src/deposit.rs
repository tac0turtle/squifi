use crate::access_control;
use fund::{
    accounts::{
        fund::{Fund, FundType},
        vault::TokenVault,
    },
    error::{FundError, FundErrorCode},
};
use serum_common::pack::Pack;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    info, program,
    pubkey::Pubkey,
};
use spl_token::instruction;
use std::convert::Into;

pub fn handler(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Result<(), FundError> {
    info!("Handler: deposit");

    let acc_infos = &mut accounts.iter();

    let vault_acc_info = next_account_info(acc_infos)?;
    let depositor_acc_info = next_account_info(acc_infos)?;
    let depositor_authority_acc_info = next_account_info(acc_infos)?;
    let fund_acc_info = next_account_info(acc_infos)?;
    let vault_authority_acc_info = next_account_info(acc_infos)?;
    let token_program_acc_info = next_account_info(acc_infos)?;

    let nft_mint_acc_info = acc_infos.next(); // optional
    let nft_token_acc_info = acc_infos.next(); //optional
    let whitelist_acc_info = acc_infos.next(); // optional

    access_control(AccessControlRequest {
        program_id,
        amount,
        fund_acc_info,
        depositor_acc_info,
        depositor_authority_acc_info,
        vault_acc_info,
        vault_authority_acc_info,
        nft_mint_acc_info,
        nft_token_acc_info,
        whitelist_acc_info,
    })?;

    Fund::unpack_mut(
        &mut fund_acc_info.try_borrow_mut_data()?,
        &mut |fund_acc: &mut Fund| {
            state_transistion(StateTransistionRequest {
                accounts,
                fund_acc,
                fund_acc_info,
                depositor_authority_acc_info,
                depositor_acc_info,
                vault_acc_info,
                vault_authority_acc_info,
                token_program_acc_info,
                nft_mint_acc_info,
                nft_token_acc_info,
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
        amount,
        fund_acc_info,
        depositor_acc_info,
        depositor_authority_acc_info,
        vault_acc_info,
        vault_authority_acc_info,
        nft_mint_acc_info,
        nft_token_acc_info,
        whitelist_acc_info,
    } = req;

    if !depositor_authority_acc_info.is_signer {
        return Err(FundErrorCode::Unauthorized)?;
    }
    {
        // let rent = access_control::rent(rent_acc_info)?;
        let fund = access_control::fund(fund_acc_info, program_id)?;
        let _ = access_control::vault_join(
            vault_acc_info,
            vault_authority_acc_info,
            fund_acc_info,
            program_id,
        )?;
        let _ = access_control::check_balance(fund_acc_info, amount)?;
        let _ = access_control::fund_open(fund_acc_info, program_id)?;
        // check if the despoitor is part of the whitelist.
        if fund.fund_type.eq(&FundType::Raise {
            private: (true || false),
        }) {
            let _ = access_control::check_nft(
                &fund,
                nft_mint_acc_info
                    .ok_or(FundErrorCode::NFTMintMissing)
                    .unwrap(),
                nft_token_acc_info
                    .ok_or(FundErrorCode::NFTTokenAccountMissing)
                    .unwrap(),
            )?;
        }
        if fund.fund_type.eq(&FundType::Raise { private: true }) {
            let _ = access_control::check_depositor(
                program_id,
                whitelist_acc_info
                    .ok_or(FundErrorCode::NFTTokenAccountMissing)
                    .unwrap()
                    .clone(),
                &fund,
                depositor_acc_info,
            )?;
        }
    }

    info!("access control deposit success");

    Ok(())
}
fn state_transistion(req: StateTransistionRequest) -> Result<(), FundError> {
    let StateTransistionRequest {
        accounts,
        fund_acc,
        fund_acc_info,
        depositor_acc_info,
        vault_acc_info,
        vault_authority_acc_info,
        depositor_authority_acc_info,
        token_program_acc_info,
        nft_mint_acc_info,
        nft_token_acc_info,
        amount,
    } = req;

    {
        if fund_acc.fund_type.eq(&FundType::Raise {
            private: (true || false),
        }) {
            info!("invoke SPL token mint");
            let mint_to_instr = instruction::mint_to(
                &spl_token::ID,
                nft_token_acc_info.unwrap().key,
                nft_mint_acc_info.unwrap().key,
                vault_authority_acc_info.key,
                &[],
                amount,
            )?;

            let signer_seeds = TokenVault::signer_seeds(fund_acc_info.key, &fund_acc.nonce);

            program::invoke_signed(&mint_to_instr, &accounts[..], &[&signer_seeds])?;
        }
    }

    fund_acc.add(amount);
    // Send tokens from depositor to fund account.
    // Now transfer SPL funds from the depositor, to the
    // program-controlled account.
    {
        info!("invoke SPL token transfer");
        let deposit_instruction = instruction::transfer(
            &spl_token::ID,
            depositor_acc_info.key,
            vault_acc_info.key,
            depositor_authority_acc_info.key,
            &[],
            amount as u64,
        )?;
        program::invoke_signed(
            &deposit_instruction,
            &[
                depositor_acc_info.clone(),
                depositor_authority_acc_info.clone(),
                vault_acc_info.clone(),
                token_program_acc_info.clone(),
            ],
            &[],
        )?;
    }
    info!("state transition deposit success");

    Ok(())
}

struct AccessControlRequest<'a, 'b> {
    program_id: &'a Pubkey,
    amount: u64,
    fund_acc_info: &'a AccountInfo<'b>,
    depositor_acc_info: &'a AccountInfo<'b>,
    depositor_authority_acc_info: &'a AccountInfo<'b>,
    vault_acc_info: &'a AccountInfo<'b>,
    vault_authority_acc_info: &'a AccountInfo<'b>,
    nft_mint_acc_info: Option<&'a AccountInfo<'b>>,
    nft_token_acc_info: Option<&'a AccountInfo<'b>>,
    whitelist_acc_info: Option<&'a AccountInfo<'b>>,
}

struct StateTransistionRequest<'a, 'b, 'c> {
    accounts: &'a [AccountInfo<'b>],
    fund_acc: &'c mut Fund,
    fund_acc_info: &'a AccountInfo<'b>,
    depositor_acc_info: &'a AccountInfo<'b>,
    depositor_authority_acc_info: &'a AccountInfo<'b>,
    vault_acc_info: &'a AccountInfo<'b>,
    vault_authority_acc_info: &'a AccountInfo<'b>,
    token_program_acc_info: &'a AccountInfo<'b>,
    nft_token_acc_info: Option<&'a AccountInfo<'b>>,
    nft_mint_acc_info: Option<&'a AccountInfo<'b>>,
    amount: u64,
}
