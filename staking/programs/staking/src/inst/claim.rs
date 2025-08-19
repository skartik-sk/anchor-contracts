use crate::{error::CustomError, state::*};
use anchor_lang::{accounts::signer, prelude::*, solana_program::program::invoke, Bumps};
use anchor_spl::{
    token::{transfer, Mint, Token, TokenAccount, Transfer},
    token_2022::{mint_to, AmountToUiAmount, MintTo},
};

#[derive(Accounts)]
pub struct Claim<'info> {
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



        #[account(
  mut,
    seeds= [b"reward",config.key().as_ref()],
    bump= config.rewards_bump,
)]
    pub reward: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = reward,
        associated_token::authority = signer
    )]
    pub user_reward_mint: Account<'info, TokenAccount>,


    pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
}

impl<'info> Claim<'info> {
    pub fn claim_it(&mut self) -> Result<()> {
        let amount = self.user.points;
         require!(amount > 0, CustomError::NoRewardsToClaim);



                let seeds: &[&[u8]] = &[b"config", &[self.config.bump]];
        let signer: &[&[&[u8]]; 1] = &[seeds];
        

              let cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.reward.to_account_info(),
                to: self.user_reward_mint.to_account_info(),
                authority: self.config.to_account_info(),
            },
            signer
        );

        mint_to(cpi, amount.into())?;



        self.user.staked_amount = 0;
       
     

        Ok(())
    }
}
