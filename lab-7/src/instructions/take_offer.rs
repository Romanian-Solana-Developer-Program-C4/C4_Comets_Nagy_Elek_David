use crate::state::Offer;

// offer id 1 -> wanted btc, offered usdc
// offer id 2 -> wanted fartcoin, offered usdc

pub fn handler(ctx: Context<TakeOffer>, id: u64) -> Result<()> {
    // taker sends token B to maker
    // vault sends token A to taker

    let transfer_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
    );

    transfer_checked(
        cpi_context,
        ctx.accounts.offer.token_a_offered_amount,
        ctx.accounts.token_mint_a.decimals,
    );

    Ok(())
}

// taker -> calls take_offer created by Maker
// taker -> sends token B to Maker and receives token A from vault

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct TakeOffer {
    // acounts
    // 1. Signer (taker)
    // 2. Taken mint B
    // 3. Taker token account A
    // 4. Maker token account B
    // 6. Token mint A
    // 7. Token program
    // 8. Vault Token account A
    // 9. Taker token account B
    // 10. Offer
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub taker_token_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub taker_token_account_b: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub maker_token_account_b: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}
