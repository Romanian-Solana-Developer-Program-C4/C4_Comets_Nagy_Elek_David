#![allow(unexpected_cfgs)]
#![allow(deprecated)] // optional, silences internal realloc warning

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DaWd2LYWf1pvUuRZBToqajv2UpaCST6mXBK9bUiNz9J8");

#[program]
pub mod escrow {
    use anchor_spl::token::{transfer_checked, TransferChecked};

    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        msg!("Make offer");

        ctx.accounts.offer.set_inner(Offer {
            maker: ctx.accounts.maker.key(),
            token_a_mint: ctx.accounts.token_a_mint.key(),
            token_b_mint: ctx.accounts.token_b_mint.key(),
            token_a_offered_amount: 0, // TODO
            token_b_wanted_amount,
            // bump: ctx.accounts.offer.bump,
        });

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

    pub fn take_offer(_ctx: Context<TakeOffer>) -> Result<()> {
        msg!("Take offer");
        Ok(())
    }
}
