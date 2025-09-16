use crate::state::stake_account::StakeAccount;
use crate::state::user_account::UserAccount;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct Stake<'info> {
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::owner = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init,
        payer = user,
        space =8+ StakeAccount::INIT_SPACE,
        seeds = [b"receipt".as_ref(), user.key().as_ref(), mint.key().as_ref()],
        bump,

    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&self) -> Result<()> {}
}
