pub struct Processor {}

impl Processor {
  pub fn process_initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let fund_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    let pool_mint_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    // Stake pool account should not be already initialized
    if State::Unallocated != State::deserialize(&fund_info.data.borrow())? {
      return Err(Error::AlreadyInUse.into());
    }

    // Numerator should be smaller than or equal to denominator (fee <= 1)
    if init.fee.numerator > init.fee.denominator {
      return Err(Error::FeeTooHigh.into());
    }

    let fund = State::Init(Fund {
      owner: *owner_info.key,
      // deposit_bump_seed: Self::find_authority_bump_seed(
      //   program_id,
      //   stake_pool_info.key,
      //   Self::AUTHORITY_DEPOSIT,
      // ),
      // withdraw_bump_seed: Self::find_authority_bump_seed(
      //   program_id,
      //   stake_pool_info.key,
      //   Self::AUTHORITY_WITHDRAW,
      // ),
      pool_mint: *pool_mint_info.key,
      // owner_fee_account: *owner_fee_info.key,
      token_program_id: *token_program_info.key,
      stake_total: 0,
      pool_total: 0,
      // fee: init.fee,
    });
    fund.serialize(&mut fund.data.borrow_mut())
  }
}
