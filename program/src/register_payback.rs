use crate::access_control;
use fund::{
    accounts::Fund,
    error::{FundError, FundErrorCode},
};
use serum_common::pack::Pack;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    info,
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

    let fund_acc_info = next_account_info(acc_infos)?;
    let owner_acc_info = next_account_info(acc_infos)?;
    let token_program_acc_info = next_account_info(acc_infos)?;

    access_control(AccessControlRequest {
        program_id,
        fund_acc_info,
        owner_acc_info,
    })?;

    Fund::unpack_mut(
        &mut fund_acc_info.try_borrow_mut_data()?,
        &mut |fund_acc: &mut Fund| {
            state_transistion(StateTransistionRequest {
                fund_acc,
                owner_acc_info,
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
        owner_acc_info,
    } = req;

    if !owner_acc_info.is_signer {
        return Err(FundErrorCode::Unauthorized.into());
    }

    let _ = access_control::fund(fund_acc_info, program_id)?;

    let _ = access_control::withdraw(program_id, fund_acc_info, owner_acc_info);

    Ok(())
}

fn state_transistion(req: StateTransistionRequest) -> Result<(), FundError> {
    let StateTransistionRequest {
        fund_acc,
        owner_acc_info,
        token_program_acc_info,
        amount,
    } = req;

    info!("State-Transistion: Initialize Payback");

    fund_acc.add_payback_bal(amount);
    let per_share = fund_acc.shares.checked_div(amount).unwrap();
    fund_acc.add_payback_per_share(per_share);

    info!("State-Transistion: Initialize Payback Success");
    Ok(())
}

struct AccessControlRequest<'a, 'b> {
    program_id: &'a Pubkey,
    fund_acc_info: &'a AccountInfo<'b>,
    owner_acc_info: &'a AccountInfo<'b>,
}

struct StateTransistionRequest<'a, 'b> {
    fund_acc: &'a mut Fund,
    owner_acc_info: &'a AccountInfo<'b>,
    token_program_acc_info: &'a AccountInfo<'b>,
    amount: u64,
}
