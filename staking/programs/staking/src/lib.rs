use anchor_lang::prelude::*;

declare_id!("9XEr99r4qobvtd5aoxS2kejppgPF6iSzpXx8oF6j6q59");
pub mod state;
pub mod inst;
pub mod error;

use inst::*;
use state::*;
#[program]
pub mod staking {
    use super::*;
     pub fn initialize_config(
        ctx: Context<InitConfig>,
        points_per_stake: u8,
        max_unstake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .init_config(points_per_stake, max_unstake, freeze_period, ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.init_user(ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake_it(ctx.bumps)
    }

    pub fn unstake(ctx: Context<UnStake>) -> Result<()> {
        ctx.accounts.unstake_it(ctx.bumps)
    }

    pub fn claim_rewards(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim_it()
    }

   
}
