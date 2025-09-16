#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub points: u32,
    pub amount_staked: u64,
    pub bump: u8,
}
