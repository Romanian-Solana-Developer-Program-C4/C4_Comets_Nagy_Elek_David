#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DaWd2LYWf1pvUuRZBToqajv2UpaCST6mXBK9bUiNz9J8");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer_handler(ctx: Context<MakeOffer>) -> Result<()> {
        msg!("Initializing escrow program: {:?}", ctx.program_id);
        msg!("Make offer");
        Ok(())
    }

    pub fn take_offer_handler(ctx: Context<TakeOffer>) -> Result<()> {
        msg!("Initializing user account program: {:?}", ctx.program_id);
        msg!("Take offer");
        Ok(())
    }
}
