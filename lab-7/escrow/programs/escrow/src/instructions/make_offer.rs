use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

// use crate::state::Offer;
use crate::{Offer, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_a_mint: Account<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_b_mint: Account<'info, Mint>,

    #[account(mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_a_account: Account<'info, TokenAccount>,
    // pub maker_token_a_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        seeds = [b"offer".as_ref(), maker.key().as_ref(), id.to_le_bytes().as_ref()],
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(init,
        payer = maker,
        associated_token::mint = token_a_mint,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}
