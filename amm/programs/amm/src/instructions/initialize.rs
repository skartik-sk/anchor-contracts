use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constant::{LIQUID_POOL, LPMINT},
    state::pool::Pool,
};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = initializer
    )]
    pub token_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = initializer
    )]
    pub token_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = initializer,
        space = Pool::INIT_SPACE + 8,
        seeds = [LIQUID_POOL,seed.to_le_bytes().as_ref()],
        bump
    )]
    pub pool_account: Account<'info, Pool>,

    #[account(
        init,
        payer = initializer,
        seeds = [LPMINT,pool_account.key().to_bytes().as_ref()],
        bump,
        mint::authority = pool_account,
        mint::decimals = 6,
    )]
    pub lp_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = pool_account,
    )]
    pub token_x_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
            init,
            payer = initializer,
            associated_token::mint = mint_y,
            associated_token::authority = pool_account,
        )]
    pub token_y_vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializePool<'info> {
    pub fn init_pool(
        &mut self,
        seed: u64,
        fee: u16,
        admin: Option<Pubkey>,
        bump: InitializePoolBumps,
    ) -> Result<()> {
        self.pool_account.set_inner(Pool {
            seed,
            authority: admin,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            locked: false,
            pool_bump: bump.pool_account,
            lp_mint_bump: bump.lp_mint,
        });

        Ok(())
    }
}
