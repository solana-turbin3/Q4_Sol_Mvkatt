use anchor_lang::prelude::*;

declare_id!("HhHSExGs8rfwQhKdRRCQM4nXN2cx9pEk9cR65RGveAT2");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
