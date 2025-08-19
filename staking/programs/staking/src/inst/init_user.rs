
use crate::state::*;
use anchor_lang::{prelude::*, Bumps};
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]

pub struct InitializeUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer= signer,
        space= 8+ UserAccount::INIT_SPACE,
        seeds= [b"user",signer.key().as_ref()],
        bump,

    )]
    pub user: Account<'info,UserAccount>,
    pub system_program: Program<'info,System>,

}

impl<'info> InitializeUser<'info> {

    pub fn init_user(&mut self,
        bumps:InitializeUserBumps
    )->Result<()>{
        self.user.set_inner(UserAccount{
            points:0,
            staked_amount:0,
            bump:bumps.user
        });
        Ok(())
    }
}