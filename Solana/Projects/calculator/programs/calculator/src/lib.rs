use anchor_lang::prelude::*;

declare_id!("EoHYfNYW4SzzVUFTpMuiavnv2cUs9afi7iw7feHPRKMy");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
