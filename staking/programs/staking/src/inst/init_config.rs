use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer= signer,
        space= 8+ Config::INIT_SPACE,
        seeds= [b"config"],
        bump,

    )]
    pub config: Account<'info, Config>,

    #[account(
    init,
    payer= signer,
    seeds= [b"reward",config.key().as_ref()],
    bump,
    mint::decimals= 6,
    mint::authority=config
)]
    pub reward: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitConfig<'info> {
    pub fn init_config(
        &mut self,
        points_per_stake: u8,
        max_unstake: u8,
        freeze_period: u32,
        bumps: InitConfigBumps,
    )->Result<()> {
        self.config.set_inner(Config {
            points_per_stake,
            max_unstake,
            freeze_period,
            rewards_bump: bumps.reward,
            bump: bumps.config,
        });
Ok(())

    }
}
