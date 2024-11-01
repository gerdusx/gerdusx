use anchor_lang::prelude::*;

declare_id!("6VpH7jvQCXNVW1oozrSVh1EJDdC9RgZYC5D8YRrqwkcv");

#[program]
pub mod day_04 {
    use super::*;

    pub fn limit_range(_ctx: Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::TooSmall);
        require!(a <= 100, MyError::TooBig);

        Ok(())
    }

    pub fn always_errors(_ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        return Err(MyError::AlwaysErrors.into());
    }
}

#[derive(Accounts)]
pub struct LimitRange {}


#[error_code]
pub enum MyError {
    #[msg("a is too small")]
    TooSmall,
    #[msg("a is too big")]
    TooBig,
    #[msg("Always errors")]
    AlwaysErrors,
}
