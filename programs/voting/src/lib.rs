use anchor_lang::prelude::*;

declare_id!("EPqmhp1uTuRjpkK7e75Vk9rFVHwBf9JKmD1j43hHHHpt");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
