// use anyhow::anyhow;
use fund::client::Client as InnerClient;
use fund::{
    accounts::fund::{FundType, SIZE},
    error::FundError,
    // instruction::FundInstruction,
};
// use serum_common::client::rpc;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use thiserror::Error;

pub fn create_fund(
    client: &RpcClient,
    owner: &Pubkey,
    payer: &Pubkey,
    max_balance: u64,
    fund_type: FundType,
) -> Result<(), ClientError> {
    // let account = serum_common::client::rpc::create_account_rent_exempt(client, payer, SIZE, owner);

    // let signers = vec![payer, &account];

    // let create_fund_instruction =
    //     FundInstruction::Initialize(&owner, &account., max_balance, fund_type);

    // let (recent_hash, _fee_calc) = client.get_recent_blockhash()?;

    // let txn = Transaction::new_signed_with_payer(
    //     &[create_fund_instruction],
    //     Some(&payer.pubkey()),
    //     &signers,
    //     recent_hash,
    // );

    // serum_common::client::rpc::send_txn(client, &txn, false)?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Error invoking rpc: {0}")]
    RpcError(#[from] solana_client::client_error::ClientError),
    #[error("Any error: {0}")]
    Any(#[from] anyhow::Error),
    #[error("Fund error: {0}")]
    FundError(#[from] FundError),
}
