use anchor_lang::prelude::*;
use trident_derive_accounts_snapshots::AccountsSnapshots;

const MAGIC_NUMBER: u8 = 254;

declare_id!("9dPEN7KzV6LzPvGDBR5ZumpyfCVXukX3dMgBP5eQFGiJ");

#[program]
pub mod unchecked_arithmetic_0 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.user.key();
        counter.count = 0;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, input1: u8, input2: u8) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("Value1: {}", input1);
        msg!("Value2: {}", input2);

        // counter.count = buggy_math_function(input1, input2).into();
        counter.count = 5;

        Ok(())
    }
}

pub fn buggy_math_function(input1: u8, input2: u8) -> u8 {
    // INFO uncommenting the if statement can prevent
    // div-by-zero and subtract with overflow panic
    if input2 >= MAGIC_NUMBER {
        return 5;
    }
    let divisor = MAGIC_NUMBER - input2;
    input1 / divisor
}

#[derive(AccountsSnapshots, Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 40
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[derive(AccountsSnapshots, Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
