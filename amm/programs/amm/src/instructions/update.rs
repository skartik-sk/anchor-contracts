use anchor_lang::prelude::*;

use crate::{check_admin, constant::LIQUID_POOL, errors::LPErrors, state::Pool};

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(

        mut,
        seeds = [LIQUID_POOL,pool_account.seed.to_le_bytes().as_ref()],
        bump = pool_account.pool_bump
    )]
    pub pool_account: Account<'info, Pool>,
}

impl<'info> Update<'info> {
    pub fn update_lock(&mut self) -> Result<()> {
        check_admin!(self);

        self.pool_account.locked = !self.pool_account.locked;
        Ok(())
    }
}
