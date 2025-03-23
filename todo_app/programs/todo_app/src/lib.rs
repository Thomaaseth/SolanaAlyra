use anchor_lang::prelude::*;

declare_id!("GZgYbnUicwDA1N7QZeFygmx86Ncz2V4w83MHVdnxQeMs");

#[program]
pub mod todo_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
