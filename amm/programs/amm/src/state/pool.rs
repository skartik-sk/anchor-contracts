use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub seed: u64, // Seed to be able to create multiple Liquid pools
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16,     // Swap fee in **basis points**
    pub locked: bool, // If the pool is locked
    pub pool_bump: u8,
    pub lp_mint_bump: u8, // Bump seed for the LP token,
}
