use anchor_lang::prelude::*;

declare_id!("FyNhvMsRVReL6akKr4mjKZRV2hmxk7JTKEsXZdjoSfeG");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn set_favorite(ctx: Context<SetFavorite>) -> Result<()> {
        let favorite = &mut ctx.accounts.favorites;
        favorite.number = 1;
        favorite.color = 2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct SetFavorite<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + Favorites::INIT_SPACE)]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    pub color: u8,
    #[max_len(5,50)]
    pub hobbies: Vec<String>,
}
