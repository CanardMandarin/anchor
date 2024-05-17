use anchor_lang::prelude::*;

declare_id!("Externa111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum MyUpdate {
    Hello { hello: u32 },
    Bye(u32)
}

#[program]
pub mod external {
    use super::*;

    pub fn init(_ctx: Context<Init>) -> Result<()> {
        Ok(())
    }

    pub fn update(ctx: Context<Update>, value: MyUpdate) -> Result<()> {
        ctx.accounts.my_account.field = match value {
            MyUpdate::Hello { hello } => hello,
            MyUpdate::Bye(v) => v
        };

        Ok(())
    }

    pub fn update_composite(ctx: Context<UpdateComposite>, value: u32) -> Result<()> {
        ctx.accounts.update.my_account.field = value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 4,
        seeds = [authority.key.as_ref()],
        bump
    )]
    pub my_account: Account<'info, MyAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    pub authority: Signer<'info>,
    #[account(mut, seeds = [authority.key.as_ref()], bump)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct UpdateComposite<'info> {
    pub update: Update<'info>,
}

#[account]
pub struct MyAccount {
    pub field: u32,
}

#[event]
pub struct MyEvent {
    pub value: u32,
}
