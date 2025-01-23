use anchor_lang::prelude::*;

declare_id!("Hf6mGsNZNcGxTVwmRVRbzZsAL2ezW7NNMxcivRLssE6X");

#[program]
pub mod memecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
