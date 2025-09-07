// pub mod instructions;

declare_id!("escrow_program");
#[program]
mod escrow {
    use super::*;
    pub fn make_offer(ctx: Context<MakeOffer>) -> ProgramResult {
        msg!("Make offer");
        Ok(())
    }
}
