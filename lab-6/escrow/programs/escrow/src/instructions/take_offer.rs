use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    // add other accounts as needed
}
