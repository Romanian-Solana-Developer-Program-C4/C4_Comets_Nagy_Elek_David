use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

// use crate::state::Offer;
use crate::{Offer, ANCHOR_DISCRIMINATOR};

pub fn handler(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    msg!("Make offer");

    // Save offer
    ctx.accounts.offer.set_inner(Offer {
        id,
        maker: ctx.accounts.maker.key(),
        token_a_mint: ctx.accounts.token_a_mint.key(),
        token_b_mint: ctx.accounts.token_b_mint.key(),
        token_a_offered_amount,
        token_b_wanted_amount,
        bump: ctx.accounts.offer.bump,
    });

    // Transfer tokens to vault
    let transfer_accounts = TransferChecked {
        from: ctx.accounts.maker_token_a_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token_a_mint.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        // program
        ctx.accounts.token_program.to_account_info(),
        // accounts
        transfer_accounts,
    );

    transfer_checked(
        cpi_context,
        token_a_offered_amount,
        ctx.accounts.token_a_mint.decimals,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_a_mint: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_a_account: InterfaceAccount<'info, TokenAccount>,

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
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}
