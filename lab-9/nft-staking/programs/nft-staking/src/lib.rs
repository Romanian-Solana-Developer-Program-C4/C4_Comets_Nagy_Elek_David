#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning

use anchor_lang::prelude::*;

pub mod context;
pub mod state;

pub use context::*;

declare_id!("C2MxgtPCJaRL4wKLgH1YFbURs4bwmSbrRgtyMQMFBQpR");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeConfig>,
        max_stake: u8,
        freeze_period: i32,
        points_per_stake: u8,
    ) -> Result<()> {
        let bump = *ctx
            .bumps
            .get("stake_config")
            .ok_or(ErrorCode::BumpNotAvailable)?;

        ctx.accounts
            .initialize_config(max_stake, freeze_period, points_per_stake, bump)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        msg!("Initializing user account program: {:?}", ctx.program_id);
        ctx.accounts.initialize_user(&ctx.bumps)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Bump not found in account info")]
    BumpNotAvailable,
    #[msg("Max stake must be greater than 0")]
    InvalidMaxStake,
    #[msg("Freeze period must be greater than 0")]
    InvalidFreezePeriod,
    #[msg("Points per stake must be greater than 0")]
    InvalidPointsPerStake,
}

#[derive(Accounts)]
pub struct InitializeConfig {}
