use anchor_lang::prelude::*;

declare_id!("tVP3GDjVPoUKhqLmMadcrv2Zr4ssUR53EiTA7MZJThv");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hi Raj!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
