use anchor_lang::prelude::*;

declare_id!("6VpH7jvQCXNVW1oozrSVh1EJDdC9RgZYC5D8YRrqwkcv");

#[program]
pub mod day_04 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
