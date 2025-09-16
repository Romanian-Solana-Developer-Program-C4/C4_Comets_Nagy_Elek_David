#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning

use crate::state::StakeConfig;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + StakeConfig::INIT_SPACE,
        seeds = [b"config".as_ref(), authority.key().as_ref()],
        bump,
    )]
    pub stake_config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = authority,
        seed = [b"rewards".as_ref(),stake_config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = authority,
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize(
        &mut self,
        max_stake: u8,
        freeze_period: i32,
        points_per_stake: u8,
        bump: u8,
    ) -> Result<()> {
        require!(max_stake > 0, ErrorCode::InvalidMaxStake);
        require!(freeze_period > 0, ErrorCode::InvalidFreezePeriod);
        require!(points_per_stake > 0, ErrorCode::InvalidPointsPerStake);

        self.stake_config.set_inner(StakeConfig {
            max_stake,
            freeze_period,
            points_per_stake,
            bump,
        });

        Ok(())
    }
}
