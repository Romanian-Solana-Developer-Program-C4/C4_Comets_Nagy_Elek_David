#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning

use anchor_lang::prelude::*;

declare_id!("FyNhvMsRVReL6akKr4mjKZRV2hmxk7JTKEsXZdjoSfeG");

// Anchor accounts use 8 bytes to determine their type
pub const ANCHOR_ACCOUNT_DISCRIMINATOR_LENGTH: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        // let user_pubkey = ctx.accounts.user.key();
        msg!("Greetings from {}", ctx.program_id);
        msg!("User {ctx.accounts.user.key()}'s favorite number is {number}, favorite color is {color}, and favorite hobbies are {hobbies}");

        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_ACCOUNT_DISCRIMINATOR_LENGTH + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>, // stored in account heap, BPF-safe
}
