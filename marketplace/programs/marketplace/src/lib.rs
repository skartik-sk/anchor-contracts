use anchor_lang::prelude::*;

declare_id!("8DJpSGGS8RbaAdC9UtP1ds5Meja1G457oLE8o9zm2oCD");

mod state;
mod error;

mod instructions;
use instructions::*;
use error::*;

#[program]
pub mod anchor_marketplace {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;

        Ok(())
    }

    pub fn listing(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;

        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;

        Ok(())
    }

    // pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
    //     ctx.accounts.send_sol()?;
    //     ctx.accounts.send_nft()?;
    //     ctx.accounts.close_mint_vault()?;

    //     Ok(())
    // }
}