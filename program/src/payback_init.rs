use crate::access_control;
use fund::{
    accounts::fund::Fund,
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
    info!("Handler: payback_init");
    let acc_infos = &mut accounts.iter();

    let payback_vault_acc_info = next_account_info(acc_infos)?;
    let fund_acc_info = next_account_info(acc_infos)?;
    let depositor_acc_info = next_account_info(acc_infos)?;
    let depositor_authority_acc_info = next_account_info(acc_infos)?;
    let token_program_acc_info = next_account_info(acc_infos)?;

    access_control(AccessControlRequest {
        program_id,
        fund_acc_info,
        depositor_acc_info,
        depositor_authority_acc_info,
    })?;

    Fund::unpack_mut(
        &mut fund_acc_info.try_borrow_mut_data()?,
        &mut |fund_acc: &mut Fund| {
            state_transistion(StateTransistionRequest {
                fund_acc,
                depositor_acc_info,
                depositor_authority_acc_info,
                payback_vault_acc_info,
                token_program_acc_info,
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
        depositor_acc_info,
        depositor_authority_acc_info,
    } = req;

    if !depositor_authority_acc_info.is_signer {
        return Err(FundErrorCode::Unauthorized.into());
    }

    let _ = access_control::fund(fund_acc_info, program_id)?;

    let _ = access_control::withdraw(program_id, fund_acc_info, depositor_acc_info);

    Ok(())
}

fn state_transistion(req: StateTransistionRequest) -> Result<(), FundError> {
    let StateTransistionRequest {
        fund_acc,
        depositor_acc_info,
        depositor_authority_acc_info,
        payback_vault_acc_info,
        token_program_acc_info,
        amount,
    } = req;

    info!("State-Transistion: Initialize Payback");

    fund_acc.payback_vault = *payback_vault_acc_info.key;
    fund_acc.add_payback_bal(amount);

    {
        {
            info!("invoke SPL token transfer");
            let deposit_instruction = instruction::transfer(
                &spl_token::ID,
                depositor_acc_info.key,
                payback_vault_acc_info.key,
                depositor_authority_acc_info.key,
                &[],
                amount,
            )?;
            program::invoke_signed(
                &deposit_instruction,
                &[
                    depositor_acc_info.clone(),
                    depositor_authority_acc_info.clone(),
                    payback_vault_acc_info.clone(),
                    token_program_acc_info.clone(),
                ],
                &[],
            )?;
        }
    }

    info!("State-Transistion: Initialize Payback Success");
    Ok(())
}

struct AccessControlRequest<'a, 'b> {
    program_id: &'a Pubkey,
    fund_acc_info: &'a AccountInfo<'b>,
    depositor_acc_info: &'a AccountInfo<'b>,
    depositor_authority_acc_info: &'a AccountInfo<'b>,
}

struct StateTransistionRequest<'a, 'b> {
    fund_acc: &'a mut Fund,
    depositor_acc_info: &'a AccountInfo<'b>,
    depositor_authority_acc_info: &'a AccountInfo<'b>,
    payback_vault_acc_info: &'a AccountInfo<'b>,
    token_program_acc_info: &'a AccountInfo<'b>,
    amount: u64,
}
