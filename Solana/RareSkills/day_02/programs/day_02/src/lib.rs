use anchor_lang::prelude::*;

declare_id!("rnch3PvgSdkx8qd8DAbxunj6jEkqWuL7eyVJCt5nWJS");

#[program]
pub mod day_02 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You sent {} and {}", a, b);
        msg!("You said: {}", message);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Array: {:?}", arr);
        Ok(())
    }

    pub fn subtract_notsafe(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        msg!("{} + {} = {}", a, b, a - b);
        Ok(())
    }

    pub fn subtract_safe(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let c = a.checked_sub(b).unwrap();
        msg!("{} - {} = {}", a, b, c);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
