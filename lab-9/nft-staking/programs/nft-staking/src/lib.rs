#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning
use anchor_lang::prelude::*;

declare_id!("AESehE9tH7VxNKfcBspX7ZkE3F5zupjHNFMhn9edrw5t");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
