use anchor_lang::prelude::*;
use anchor_spl::token::{
    self,
    Mint,
    Token,
    TokenAccount,
    MintTo,
    InitializeMint,
    InitializeAccount,
    Transfer
}

declare_id!("Hf6mGsNZNcGxTVwmRVRbzZsAL2ezW7NNMxcivRLssE6X");

#[program]
pub mod memecoin {
    use super::*;

    //initializes the mint and mints some tokens
    pub fn initialize_mint(ctx: Context<InitializeMintAccounts>, bump: u8, decimals: u8, amount: u64) -> Result<()> {
        //initializing mint
        let cpi_accounts = token::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let init_mint_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_mint(init_mint_ctx, decimals, ctx.accounts.authority.key, None)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMintAccounts<'info> {

}
