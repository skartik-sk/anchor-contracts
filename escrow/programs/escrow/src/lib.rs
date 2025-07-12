

#![allow(unexpected_cfgs,ambiguous_glob_reexports)]

use anchor_lang::prelude::*;
pub mod handler;
pub mod state;
pub use handler::*;
//           22222222222222222222222222222222222222222222
declare_id!("6MwpYpbT8RasEECigH2sFBr8xvNVRNHLkyUQhz1tzrL3");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_escrow(
        context: Context<Make>,
        ids: u64,
        deposit: u64,
        demand: u64,
    ) -> Result<()> {
        msg!("making account initiated");
        handler::make::intilize_and_deposit(context, ids, deposit, demand)?;
        Ok(())
    }

    

    pub fn take(context: Context<Take>) -> Result<()> {
        msg!("swap inittiated");
        handler::take::take(context)?;
        Ok(())
    }

    pub fn refund_it(context: Context<Refund>) -> Result<()> {
        msg!("refund initiated");
        handler::refund::_refund(context)?;
        Ok(())
    }
}
