use crate::InitializeResponse;
use fund::accounts::fund::{Fund, FundType};
use fund::client::{Client as InnerClient, ClientError as InnerClientError};
use serum_common::client::rpc;
use serum_common::pack::Pack;
use solana_client_gen::prelude::*;
use solana_client_gen::solana_sdk;
use solana_client_gen::solana_sdk::instruction::AccountMeta;
use solana_client_gen::solana_sdk::pubkey::Pubkey;
use solana_client_gen::solana_sdk::system_instruction;

pub fn create_all_accounts_and_initialize_fundme(
    client: &InnerClient,
    token_mint: &Pubkey,
    fund_authority: &Pubkey,
    max_balance: u64,
    fund_type: FundType,
) -> Result<InitializeResponse, InnerClientError> {
    let fund_acc = Keypair::generate(&mut OsRng);
    let (fund_vault_authority, nonce) =
        Pubkey::find_program_address(&[fund_acc.pubkey().as_ref()], client.program());

    let fund_vault = serum_common::client::rpc::create_token_account(
        client.rpc(),
        &token_mint,
        &fund_vault_authority,
        client.payer(),
    )
    .map_err(|e| InnerClientError::RawError(e.to_string()))?;

    let wl_kp = Keypair::generate(&mut OsRng);
    let instructions = {
        let create_fund_acc_instr = {
            let lamports = client
                .rpc()
                .get_minimum_balance_for_rent_exemption(Fund::default().size().unwrap() as usize)
                .map_err(InnerClientError::RpcError)?;
            system_instruction::create_account(
                &client.payer().pubkey(),
                &fund_acc.pubkey(),
                lamports,
                Fund::default().size().unwrap(),
                client.program(),
            )
        };

        let accounts = [
            AccountMeta::new(fund_acc.pubkey(), false),
            AccountMeta::new_readonly(fund_vault.pubkey(), false),
            AccountMeta::new_readonly(*token_mint, false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ];

        let initialize_instr = fund::instruction::initialize(
            *client.program(),
            &accounts,
            *fund_authority,
            max_balance,
            fund_type,
            nonce,
        );

        vec![
            create_fund_acc_instr,
            // create_whitelist_acc_instr,
            initialize_instr,
        ]
    };

    let tx = {
        let (recent_hash, _fee_calc) = client
            .rpc()
            .get_recent_blockhash()
            .map_err(|e| InnerClientError::RawError(e.to_string()))?;
        let signers = vec![client.payer(), &fund_acc, &wl_kp];
        Transaction::new_signed_with_payer(
            &instructions,
            Some(&client.payer().pubkey()),
            &signers,
            recent_hash,
        )
    };

    client
        .rpc()
        .send_and_confirm_transaction_with_spinner_and_config(
            &tx,
            client.options().commitment,
            client.options().tx,
        )
        .map_err(InnerClientError::RpcError)
        .map(|sig| InitializeResponse {
            tx: sig,
            fund: fund_acc.pubkey(),
            vault_authority: fund_vault_authority,
            vault: fund_vault.pubkey(),
            whitelist: None,
            nonce,
        })
}
