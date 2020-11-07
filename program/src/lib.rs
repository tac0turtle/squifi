pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "exclude_entrypoint"))]
pub mod entrypoint;

pub use solana_program;

solana_program::declare_id!("SPQQL11111111111111111111111111111111111111");