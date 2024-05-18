use anchor_lang::prelude::*;

declare_id!("Dec1areProgram11111111111111111111111111111");

declare_program!(external);
use external::program::External;

#[program]
pub mod declare_program {
    use super::*;

    pub fn cpi(ctx: Context<Cpi>, value: u64) -> Result<()> {

        Ok(())
    }

    pub fn cpi_composite(ctx: Context<Cpi>, value: u64) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub cpi_my_account: AccountLoader<'info, external::accounts::MyAccount>,
    pub external_program: Program<'info, External>,
}

#[derive(Accounts)]
pub struct Utils<'info> {
    pub authority: Signer<'info>,
}
