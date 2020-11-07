use clap::{crate_description, crate_name, crate_version, value_t, Arg, App, SubCommand };
use solana_clap_utils::{
  input_validators::{is_keypair},
  keypair::signer_from_path,
};
use solana_sdk::{
  signature::{Keypair, Signer},
  pubkey::Pubkey,
  transaction::Transaction,
};
use solana_client::{
  rpc_client::RpcClient,
};
use std::process::exit;

struct Config {
  rpc: RpcClient,
  user: Box<dyn Signer>,
}

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg({
          let arg = Arg::with_name("config_file")
              .short("C")
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
                .arg(
                  Arg::with_name("owner")
                      .long("owner")
                      .value_name("KEYPAIR")
                      .validator(is_keypair)
                      .takes_value(true)
                      .help(
                          "Specify the pool owner account. \
                          This may be a keypair file, the ASK keyword. \
                          Defaults to the client keypair.",
                      ),
              )
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
        let user = signer_from_path(
            &matches,
            &cli_config.keypair_path,
            "user",
            &mut wallet_manager,
        )
        .unwrap_or_else(|e| {
            println!("error: {}", e);
            exit(1);
        });
        
        Config {
          rpc: RpcClient::new(json_rpc_url),
          user: user,
        }
    };
    
    let _ = match matches.subcommand() {
      ("init", Some(_arg_matches)) => {
        command_init_pool(&config)
      }
      _ => unreachable!(),
    };
}

fn command_init_pool(config: &Config) {
  let pool = Keypair::new();
  println!("Creating account {}", pool.pubkey());
  
  let new_pool_tx = Transaction::new_with_payer(
      &[fund::initialize(
        &fund::id(),
        &pool.pubkey(),
        config.user.pubkey(),
        
      )], 
      Some(&config.user.pubkey(),
  );
  
  
}
