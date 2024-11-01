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

    pub fn multiply(_ctx: Context<Multiply>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_mul(b).unwrap();
        msg!("multiplying:{} * {} = {}", a, b, result);
        Ok(())
    }

    pub fn divide(_ctx: Context<Divide>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_div(b).unwrap();
        msg!("dividing:{} / {} = {}", a, b, result);
        Ok(())
    }

    pub fn square_root(_ctx: Context<SquareRoot>, a: f64) -> Result<()> {
        let result = a.sqrt();
        msg!("square root of:{} = {}", a, result);
        Ok(())
    }

    pub fn log_10(_ctx: Context<Log10>, a: f64) -> Result<()> {
        let result = a.log10();
        msg!("log10 of:{} = {}", a, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Add {}

#[derive(Accounts)]
pub struct Subtract {}

#[derive(Accounts)]
pub struct Multiply {}

#[derive(Accounts)]
    pub struct Divide {}

#[derive(Accounts)]
pub struct SquareRoot {}

#[derive(Accounts)]
pub struct Log10 {}
