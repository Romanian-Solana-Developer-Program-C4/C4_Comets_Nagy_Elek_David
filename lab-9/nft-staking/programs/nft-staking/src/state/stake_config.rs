#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub authority: Pubkey,
    pub max_stake: u8,
    pub freeze_period: i32,
    pub points_per_stake: u8,
    pub rewards_bump: u8,
    pub bump: u8,
}
