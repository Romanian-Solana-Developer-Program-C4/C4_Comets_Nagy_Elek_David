#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;

pub use constants::*;
pub use instructions::{MakeOffer, TakeOffer};
pub use state::*;

declare_id!("DaWd2LYWf1pvUuRZBToqajv2UpaCST6mXBK9bUiNz9J8");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        msg!("Make offer");
        Ok(())
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    // Define your accounts here
    #[account(mut)]
    pub user: Signer<'info>,
    // Add other accounts as needed
}
