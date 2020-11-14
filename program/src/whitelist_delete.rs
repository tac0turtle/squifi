use crate::access_control;
use fund::{
    accounts::whitelist::Whitelist,
    error::{FundError, FundErrorCode},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    info,
    pubkey::Pubkey,
};

pub fn handler(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    entry: Pubkey,
) -> Result<(), FundError> {
    info!("handler: whitelist_add");

    let acc_infos = &mut accounts.iter();

    let fund_acc_info = next_account_info(acc_infos)?;
    let fund_owner_acc_info = next_account_info(acc_infos)?;
    let whitelist_acc_info = next_account_info(acc_infos)?;

    access_control(AccessControlRequest {
        program_id,
        fund_acc_info,
        fund_owner_acc_info,
        whitelist_acc_info,
    })?;

    let whitelist = Whitelist::new(whitelist_acc_info.clone())?;

    state_transistion(StateTransistionRequest { whitelist, entry })
}

fn access_control(req: AccessControlRequest) -> Result<(), FundError> {
    info!("access-control: whitelist_add");

    let AccessControlRequest {
        program_id,
        fund_acc_info,
        fund_owner_acc_info,
        whitelist_acc_info,
    } = req;

    // check owner
    let _ = access_control::check_owner(program_id, fund_acc_info, fund_owner_acc_info)?;
    let fund = access_control::fund(fund_acc_info, program_id)?;
    let _ = access_control::whitelist(whitelist_acc_info.clone(), &fund, program_id)?;

    Ok(())
}

fn state_transistion(req: StateTransistionRequest) -> Result<(), FundError> {
    let StateTransistionRequest { whitelist, entry } = req;

    whitelist
        .delete(entry)?
        .ok_or(FundErrorCode::WhitelistNotFound)?;

    Ok(())
}

struct AccessControlRequest<'a, 'b> {
    program_id: &'a Pubkey,
    fund_acc_info: &'a AccountInfo<'b>,
    fund_owner_acc_info: &'a AccountInfo<'b>,
    whitelist_acc_info: &'a AccountInfo<'b>,
}

struct StateTransistionRequest<'a> {
    whitelist: Whitelist<'a>,
    entry: Pubkey,
}
