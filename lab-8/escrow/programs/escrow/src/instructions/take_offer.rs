use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    // close_account,
    transfer_checked,
    Mint,
    // CloseAccount,
    TokenAccount,
    TokenInterface,
    TransferChecked,
};

use crate::Offer;

pub fn take_offer_handler(ctx: Context<TakeOffer>, _id: u64) -> Result<()> {
    msg!("Take offer");
    // taker sends token B to maker
    // vault sends token A to taker

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"offer".as_ref(),
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.offer.id.to_le_bytes(),
        &[ctx.accounts.offer.bump],
    ]];

    let transfer_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.taker_token_a_account.to_account_info(),
        mint: ctx.accounts.token_a_mint.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer_seeds,
    );
    transfer_checked(
        cpi_context,
        ctx.accounts.offer.token_a_offered_amount,
        ctx.accounts.token_a_mint.decimals,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(mint::token_program = token_program)]
    pub token_a_mint: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(init_if_needed,
        payer = taker,
        associated_token::mint = token_a_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_a_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_b_account: InterfaceAccount<'info, TokenAccount>,

    #[account(init_if_needed,
        payer = taker,
        associated_token::mint = token_b_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program)]
    pub maker_token_b_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut,
        close = maker,
        has_one = maker,
        has_one = token_b_mint,
        has_one = token_a_mint,
        seeds = [b"offer".as_ref(), maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump,
    )]
    pub offer: Account<'info, Offer>,

    #[account(mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = offer,
        associated_token::token_program = token_program,)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}
