use anchor_lang::prelude::*;

declare_id!("");

#[program]

mod hello_world{
    use super::*;

    pub fn initialize(ctx: Context<Test>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }

    #[derive(Accounts)]
    
    pub struct Test {
    }
}
