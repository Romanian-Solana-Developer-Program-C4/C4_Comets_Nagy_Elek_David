#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning
use anchor_lang::prelude::*;

declare_id!("AESehE9tH7VxNKfcBspX7ZkE3F5zupjHNFMhn9edrw5t");

pub mod context;
pub mod state;

pub use context::*;
// pub use state::*;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeConfig>,
        max_stake: u8,
        freeze_period: i32,
        points_per_stake: u8,
    ) -> Result<()> {
        msg!("Initialize staking program: {:?}", ctx.program_id);

        let _ =
            ctx.accounts
                .initialize_config(max_stake, freeze_period, points_per_stake, &ctx.bumps);
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        msg!(
            "Initialize user acount from program id: {:?}",
            ctx.program_id
        );

        let _ = ctx.accounts.initialize_user(&ctx.bumps);
        Ok(())
    }
}
