use crate::{error::CustomError, state::*};
use anchor_lang::{accounts::signer, prelude::*, solana_program::program::invoke, Bumps};
use anchor_spl::{
    associated_token::AssociatedToken, token::{transfer, Mint, Token, TokenAccount, Transfer}, token_2022::AmountToUiAmount
};

#[derive(Accounts)]
pub struct UnStake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[b"user",signer.key().as_ref()],
        bump=user.bump
    )]
    pub user: Account<'info, UserAccount>,

    #[account(
    mut,
    seeds= [b"config"],
    bump=config.bump
)]
    pub config: Account<'info, Config>,

    pub mint1: Account<'info, Mint>,

    #[account(
    mut,
    associated_token::mint=mint1,
    associated_token::authority= signer

)]
    pub user_mint: Account<'info, TokenAccount>,

    #[account(
     mut,
        seeds=[b"vault",mint1.key().as_ref()],
        bump,
    )]
    pub vault_mint: Account<'info, TokenAccount>,

    #[account(
    mut,
    seeds= [b"stake",signer.key().as_ref(),mint1.key().as_ref()],
    bump=stake_acc.bump,
    close = user
    )]  
    pub stake_acc: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> UnStake<'info> {
    pub fn unstake_it(&mut self, bump: UnStakeBumps) -> Result<()> {
        let clock = Clock::get()?;
          require!(
             self.stake_acc.last_staked_timestamp >= self.config.freeze_period as i64,
            CustomError::NotFrozen
        );

        // Ensure user has at least one NFT staked
        require!(
            self.user.staked_amount > 0,
            CustomError::NothingToUnstake
        );

        self.user.staked_amount=self.user.staked_amount.checked_sub(1).ok_or(
            CustomError::Overflow
        )?;


        self.user.points=self.user.points.checked_add(self.config.points_per_stake as u64).ok_or(
            CustomError::Overflow
        )?;

                let seeds: &[&[u8]] = &[b"config", &[self.config.bump]];
        let signer: &[&[&[u8]]; 1] = &[seeds];
        



        let cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            Transfer {
                from: self.vault_mint.to_account_info(),
                to: self.user_mint.to_account_info(),
                authority: self.config.to_account_info(),
            },
            signer
        );

        transfer(cpi, 1)?;

      

        Ok(())
    }
}
