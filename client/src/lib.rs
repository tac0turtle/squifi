use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signature;

mod inner;

pub struct InitializeResponse {
    pub tx: Signature,
    pub fund: Pubkey,
    pub vault: Pubkey,
    pub vault_authority: Pubkey,
    pub whitelist: Option<Pubkey>,
    pub nonce: u8,
}
