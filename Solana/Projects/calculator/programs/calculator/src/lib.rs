use anchor_lang::prelude::*;

declare_id!("EoHYfNYW4SzzVUFTpMuiavnv2cUs9afi7iw7feHPRKMy");

#[program]
pub mod calculator {
    use super::*;

    pub fn add(_ctx: Context<Add>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_add(b).unwrap();
        msg!("adding:{} + {} = {}", a, b, result);
        Ok(())
    }

    pub fn subtract(_ctx: Context<Subtract>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_sub(b).unwrap();
        msg!("subtracting:{} - {} = {}", a, b, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Add {}

#[derive(Accounts)]
pub struct Subtract {}
