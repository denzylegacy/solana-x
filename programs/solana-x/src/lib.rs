use anchor_lang::prelude::*;

declare_id!("6FQdAmTBJ7yXaVYuj6PDUDjd5qxcg2vPGsRPKwqW6Bkn");

#[program]
pub mod solana_x {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
