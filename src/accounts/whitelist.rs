use crate::error::{FundError, FundErrorCode};
use arrayref::{array_mut_ref, array_ref};
use solana_client_gen::solana_sdk::account_info::AccountInfo;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

/// Whitelist maintaining the list of program-derived-addresses the Locked
/// SRM program is allowed to delegate funds to. This is used, for example,
/// to allow locked SRM to be sent to the staking program.
///
/// Note that the whitelist backing storage is too large to be able to pack/unpack
/// it on the BPF stack. As a result, we just wrap the raw data array
/// and access the data as needed with the api accessors provided here.
///
/// This makes it a bit unsafe to use--since Solana's data storage
/// is wrapped in a RefCell, so be careful when you're mutating the
/// whitelist to avoid a RefCell induced panic.
#[derive(Debug)]
pub struct Whitelist<'a> {
    pub acc_info: AccountInfo<'a>,
}

impl<'a> Whitelist<'a> {
    /// Byte size for a single item in the whitelist.
    pub const ITEM_SIZE: usize = 32;
    /// Number of items in the whitelist.
    pub const LEN: usize = 50;
    /// Byte size of the entire whitelist.
    pub const SIZE: usize = 32 * Whitelist::LEN;

    pub fn new(acc_info: AccountInfo<'a>) -> Result<Self, FundError> {
        if acc_info.try_data_len()? != Whitelist::SIZE {
            return Err(FundErrorCode::WhitelistInvalidData)?;
        }
        Ok(Self { acc_info })
    }

    /// Returns the PubKey at the given index.
    pub fn get_at(&self, index: usize) -> Result<Pubkey, FundError> {
        let data = self.acc_info.try_borrow_data()?;
        let key = array_ref![data, index * Whitelist::ITEM_SIZE, Whitelist::ITEM_SIZE];
        Ok(Pubkey::new(key))
    }

    /// Inserts the given PubKey at the given index.
    pub fn add_at(&self, index: usize, item: Pubkey) -> Result<(), FundError> {
        let mut data = self.acc_info.try_borrow_mut_data()?;
        let dst = array_mut_ref![data, index * Whitelist::ITEM_SIZE, Whitelist::ITEM_SIZE];
        dst.copy_from_slice(item.as_ref());

        Ok(())
    }

    /// Inserts the given PubKey at the first available index.
    /// Returns Some(index) where the entry was inserted. If the Whitelist
    /// is full, returns None.
    pub fn push(&self, entry: Pubkey) -> Result<Option<usize>, FundError> {
        let existing_idx = self.index_of(&entry)?;
        if let Some(_) = existing_idx {
            return Err(FundErrorCode::PubKeyAlreadyExists)?;
        }
        let pk = Pubkey::new_from_array([0; 32]);
        let idx = self.index_of(&pk)?;
        if let Some(idx) = idx {
            self.add_at(idx, entry)?;
            return Ok(Some(idx));
        }
        Ok(idx)
    }

    /// Deletes the given entry from the Whitelist.
    pub fn delete(&self, entry: Pubkey) -> Result<Option<usize>, FundError> {
        let idx = self.index_of(&entry)?;
        if let Some(idx) = idx {
            let pk = Pubkey::new_from_array([0; 32]);
            self.add_at(idx, pk)?;
            return Ok(Some(idx));
        }
        Ok(idx)
    }

    fn index_of(&self, e: &Pubkey) -> Result<Option<usize>, FundError> {
        for k in (0..Whitelist::SIZE).step_by(Whitelist::ITEM_SIZE) {
            let curr_idx = k / Whitelist::ITEM_SIZE;
            let entry = &self.get_at(curr_idx)?;
            if entry == e {
                return Ok(Some(k));
            }
        }
        Ok(None)
    }
}
