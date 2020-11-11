use clap::{crate_name, crate_description, crate_version, value_t, Arg, App, SubCommand };
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

struct Config {
    rpc: RpcClient,
    pool: Pubkey,
    owner: Box<dyn Signer>,
    fee_payer: Box<dyn Signer>,
}

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg({
            let arg = Arg::with_name("config_file")
                .short("c")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use");
            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(&config_file)
            } else {
                arg
            }
        })
        .subcommand(
            SubCommand::with_name("create")
                .about("creates a new pool")
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("lists pools user account has access to")
        )
        .subcommand(
            SubCommand::with_name("balance")
                .about("show the balance of a pool")
                .arg(
                    Arg::with_name("pool")
                        .validator(is_pubkey)
                        .value_name("POOL")
                        .index(1)
                        .help("the pool with which you wish to check the balance of")
                        .takes_value(true)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("deposit")
                .about("deposit tokens into a pool")
        )
        .subcommand(
            SubCommand::with_name("proposals")
                .about("create, view and vote on proposals")
                .subcommand(
                    SubCommand::with_name("create")
                        .about("create a proposal for a pool")
                )
                .subcommand(
                    SubCommand::with_name("list")
                        .about("create a proposal for a pool")
                )
                .subcommand(
                    SubCommand::with_name("vote")
                        .about("create a proposal for a pool")
                )
        )
        .subcommand(
            SubCommand::with_name("withdraw")
                .about("withdraw allocated tokens")
        )
        .subcommand(
            SubCommand::with_name("destroy")
                .about("destroys the pool")
        )
        .get_matches();
    
    let config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };
        let json_rpc_url = value_t!(matches, "json_rpc_url", String)
            .unwrap_or_else(|_| cli_config.json_rpc_url.clone());
    
        let mut wallet_manager = None;
        let owner = signer_from_path(
            &matches,
            &cli_config.keypair_path,
            "owner",
            &mut wallet_manager,
        )
        .unwrap_or_else(|e| {
            println!("error: {}", e);
            exit(1);
        });

        let fee_payer = signer_from_path(
            &matches,
            &cli_config.keypair_path,
            "fee_payer",
            &mut wallet_manager,
        )
        .unwrap_or_else(|e| {
            println!("error: {}", e);
            exit(1);
        });
        
        Config {
            rpc: RpcClient::new(json_rpc_url),
            owner: owner,
            fee_payer: fee_payer
        }
    };
    
    let _ = match matches.subcommand() {
        ("init", Some(_arg_matches)) => {
            command_create_pool(&config)
        }
        ("balance", Some(arg_matches)) => {
            let pool = pubkey_of(arg_matches, "pool").unwrap();
            command_balance(&config, pool)
        }
        _ => unreachable!(),
    };
}

fn command_create_pool(config: &Config) {
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

