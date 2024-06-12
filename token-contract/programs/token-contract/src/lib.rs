use anchor_lang::prelude::*;

declare_id!("FY5EW81jpzuVjjq4hWdA2hiua37D2yduFf2cF9hJAAqX");

#[program]
pub mod token_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
