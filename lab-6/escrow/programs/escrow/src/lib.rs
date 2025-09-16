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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
