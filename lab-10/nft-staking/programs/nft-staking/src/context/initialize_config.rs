use crate::state::StakeConfig;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
#[instruction(max_stake: u8, freeze_period: i32, points_per_stake: u8)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [b"config".as_ref(), authority.key().as_ref()],
        space = 8 + StakeConfig::INIT_SPACE,
        bump,
    )]
    pub stake_config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = authority,
        seeds = [b"rewards".as_ref(), stake_config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = stake_config,
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self,
        max_stake: u8,
        freeze_period: i64,
        points_per_stake: u8,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.stake_config.set_inner(StakeConfig {
            authority: self.authority.key(),
            max_stake,
            freeze_period,
            points_per_stake,
            bump: bumps.stake_config,
            rewards_bump: bumps.rewards_mint,
        });
        Ok(())
    }
}
