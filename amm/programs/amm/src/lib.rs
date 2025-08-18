use anchor_lang::prelude::*;

declare_id!("35VG4tyiMkNsAbJXsiUH6jPg5tSWRKSfpQWrboVbao5U");

pub mod constant;
pub mod errors;
pub mod helper;
pub mod instructions;
pub mod state;

use crate::{helper::*, instructions::*, state::*};

#[program]
pub mod amm_anchor {

    use super::*;

    pub fn initialize(
        ctx: Context<InitializePool>,
        seed: u64,
        fees: u16,
        admin: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init_pool(seed, fees, admin, ctx.bumps)?;
        Ok(())
    }

    pub fn deposite(
        ctx: Context<DepositePair>,
        lp_amount: u64,
        max_token_x: u64,
        max_token_y: u64,
    ) -> Result<()> {
        ctx.accounts.deposite(lp_amount, max_token_x, max_token_y)?;
        Ok(())
    }

    pub fn update(
        ctx: Context<Update>,
       
    ) -> Result<()> {
        ctx.accounts.update_lock()?;
        Ok(())
    }


    pub fn swap(
        ctx: Context<Swap>,
        is_x:bool,
        amount:u64,
        min_slippage_amount:u64
    ) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min_slippage_amount)?;
        Ok(())
    }

    pub fn withdraw_token(ctx: Context<WithdrawTokens>,min_x:u64,min_y:u64,amount_lp:u64) -> Result<()> {
        ctx.accounts.withdraw(min_x, min_y, amount_lp)?;
        Ok(())
    }
}
