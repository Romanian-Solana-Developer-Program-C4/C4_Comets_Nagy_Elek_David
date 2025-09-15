use anchor_lang::prelude::*;

declare_id!("FyNhvMsRVReL6akKr4mjKZRV2hmxk7JTKEsXZdjoSfeG");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
