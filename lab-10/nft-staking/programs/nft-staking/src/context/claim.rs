use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

// use crate::errors::*;
use crate::state::{StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    #[account(mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump,
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(mut,
        // same seed as user_account same account [!]
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    // #[account(mut,
    //     seeds = [b"receipt".as_ref(), user.key().as_ref(), mint.key().as_ref()],
    //     bump,
    //     // constraint = stake_account.owner.key().as_ref() == user.key().as_ref(),
    // )]
    // pub stake_account: Account<'info, StakeAccount>,
    #[account(mut,
        seeds = [b"config".as_ref(), config.key().as_ref()],
        bump = config.bump,

    )]
    pub config: Account<'info, StakeConfig>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    // pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let shares = self.user_account.points;

        let mint_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            authority: self.config.to_account_info(),
            to: self.user_mint_ata.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            mint_accounts,
        );

        mint_to(
            cpi_ctx,
            shares * 10u64.pow(self.reward_mint.decimals as u32),
        )?;

        self.user_account.points = 0;

        Ok(())
    }
}
