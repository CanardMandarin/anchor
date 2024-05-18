use anchor_lang::prelude::*;
use bytemuck::Zeroable;
use bytemuck::Pod;

declare_id!("Externa111111111111111111111111111111111111");

#[program]
pub mod external {
    use super::*;

    pub fn init(_ctx: Context<Init>) -> Result<()> {
        Ok(())
    }

    pub fn update(ctx: Context<Update>, value: u64) -> Result<()> {
        let mut my_account = ctx.accounts.my_account.load_mut()?;
        my_account.field = value;
        Ok(())
    }

    pub fn update_composite(ctx: Context<UpdateComposite>, value: u64) -> Result<()> {
        let mut my_account = ctx.accounts.update.my_account.load_mut()?;
        my_account.field = value;
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
    pub my_account: AccountLoader<'info, MyAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    pub authority: Signer<'info>,
    #[account(mut, seeds = [authority.key.as_ref()], bump)]
    pub my_account: AccountLoader<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct UpdateComposite<'info> {
    pub update: Update<'info>,
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize)]
#[repr(C)]
pub struct MyStruct {
    pub my_value: u64,
}

unsafe impl Pod for MyStruct {}
unsafe impl Zeroable for MyStruct {}

#[account(zero_copy)]
pub struct MyAccount {
    pub field: u64,
    pub my_struct: MyStruct
}

#[event]
pub struct MyEvent {
    pub value: u32,
}