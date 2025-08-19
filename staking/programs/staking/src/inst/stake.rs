use crate::state::*;
use anchor_lang::{accounts::signer, prelude::*, solana_program::program::invoke, Bumps};
use anchor_spl::{
    associated_token::AssociatedToken, token::{transfer, Mint, Token, TokenAccount, Transfer}, token_2022::AmountToUiAmount
};

#[derive(Accounts)]
pub struct Stake<'info> {
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
        init_if_needed,
        payer= signer,
        seeds=[b"vault",mint1.key().as_ref()],
        bump,
        token::mint=mint1,
    token::authority= config
    )]
    pub vault_mint: Account<'info, TokenAccount>,

    #[account(
    init,
    payer=signer,
    space= 8+StakeAccount::INIT_SPACE,
    seeds= [b"stake",signer.key().as_ref(),mint1.key().as_ref()],
    bump
)]
    pub stake_acc: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> Stake<'info> {
    pub fn stake_it(&mut self, bump: StakeBumps) -> Result<()> {
        let clock = Clock::get()?;
        let cpi = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_mint.to_account_info(),
                to: self.vault_mint.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        );

        transfer(cpi, 1)?;

        self.stake_acc.set_inner(StakeAccount {
            owner: self.signer.key(),
            mint: self.mint1.key(),
            last_staked_timestamp: clock.unix_timestamp,
            bump: bump.stake_acc,
        });
        self.user.staked_amount= self.user.staked_amount.saturating_add(1);

        msg!("{}",self.user.staked_amount);
         msg!("{}",self.user.points);

        Ok(())
    }
}
