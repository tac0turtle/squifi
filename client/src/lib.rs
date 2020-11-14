use solana_sdk::{
    signature::{Keypair},
    pubkey::Pubkey,
    transaction::Transaction,
};
use solana_client::{
    rpc_client::RpcClient,
};
use serum_common::client::rpc::{create_account_rent_exempt, send_txn};
use fund::{
  accounts::{fund::FundType},
  instruction::FundInstruction,
};

#[cfg(feature = "client")]
lazy_static::lazy_static! {
    pub static ref SIZE: u64 = Vesting::default()
                .size()
                .expect("Vesting has a fixed size");
}

pub fn create_fund(
    client: &RpcClient, 
    owner: &PubKey, 
    payer: &Keypair, 
    max_balance: u64,
    fund_type: FundType
) -> Result<Keypair> {
    let account = create_account_rent_exempt(client, payer, SIZE, owner);

    let signers = vec![payer, &account];
    
    let create_func_instruction = FundInstruction::Initialize(
        &owner,
        &account.pubkey(),
        max_balance,
        fund_type,
    );

    let (recent_hash, _fee_calc) = client.get_recent_blockhash()?;

    let txn = Transaction::new_signed_with_payer(
        &[create_func_instructions],
        Some(&payer.pubkey()),
        &signers,
        recent_hash,
    );
    
    send_txn(client, &txn, false)?;
    Ok(account)
}

pub fn check_balance(fund: Pubkey) {
    println!("Checking balance...")
}