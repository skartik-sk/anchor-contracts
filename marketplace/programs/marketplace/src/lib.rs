use anchor_lang::prelude::*;

declare_id!("41NF3e7ThxSRwgpYWvpRfDrmvza9gXcPTyqEqQ3ZKQkJ");

pub mod contants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod market_place {

    use super::*;

    pub fn initialize(ctx: Context<InitMarketplace>, name: String, fee: u16) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.initialize_marketplace(name, fee, bumps)?;
        Ok(())
    }

    pub fn deposite_nft(ctx: Context<List>, price: u16) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.initialize_list(price, bumps)?;
        ctx.accounts.deposite_nft()?;
        Ok(())
    }

    pub fn withdraw_nft(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist_nft()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }

    pub fn purchase_nft(ctx: Context<PurchaseNFT>) -> Result<()> {
        ctx.accounts.deposite_amount()?;
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_accounts()?;
        Ok(())
    }
}
