use solana_clap_utils::{
    input_parsers::pubkey_of,
    input_validators::{is_keypair, is_pubkey},
    keypair::signer_from_path,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::lamports_to_sol,
    signature::{Keypair, Signer, Signature},
    pubkey::Pubkey,
    transaction::Transaction,
};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
};
use std::process::exit;
use fund::{
  accounts::{fund::FundType},
  instruction::FundInstruction,
};

type Error = Box<dyn std::error::Error>;

pub fn create_pool(rpc_client: RpcClient, owner: Signer) {
    let pool = Keypair::new();
    println!("Creating account {}", pool.pubkey());
    
    let mut new_pool_tx = Transaction::new_with_payer(
        &[FundInstruction::Initialize(
            config.owner.pubkey(),
            config.owner.pubkey(),
            100, // hard-coded settings
            FundType::FundMe,
        )?], 
        Some(&config.fee_payer.pubkey()),
    );
    
    let (recent_blockhash, fee_calculator) = config.rpc_client.get_recent_blockhash()?;
    check_fee_payer_balance(config, fee_calculator.calculate_fee(&new_pool_tx.message()))?;
    new_pool_tx.sign(&config.owner.as_ref(), recent_blockhash, &config.fee_payer.as_ref());
    
    let signature = config
        .rpc
        .send_and_confirm_transaction_with_spinner_and_commitment(
            &transaction,
            CommitmentConfig::single(),
        )?;
    println!("Signature: {}", signature);
    Ok(())
}

fn command_balance(config: &Config, pool: Pubkey) {
    println!("Checking balance...")
}

fn check_fee_payer_balance(config: &Config, required_balance: u64) -> Result<(), Error> {
    let balance = config.rpc.get_balance(&config.fee_payer.pubkey())?;
    if balance < required_balance {
        Err(format!(
            "Fee payer, {}, has insufficient balance: {} required, {} available",
            config.fee_payer.pubkey(),
            lamports_to_sol(required_balance),
            lamports_to_sol(balance)
        )
        .into())
    } else {
        Ok(())
    }
}

fn check_owner_balance(config: &Config, required_balance: u64) -> Result<(), Error> {
    let balance = config.rpc.get_balance(&config.owner.pubkey())?;
    if balance < required_balance {
        Err(format!(
            "Owner, {}, has insufficient balance: {} required, {} available",
            config.owner.pubkey(),
            lamports_to_sol(required_balance),
            lamports_to_sol(balance)
        )
        .into())
    } else {
        Ok(())
    }
}