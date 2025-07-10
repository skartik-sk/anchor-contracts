use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub ids:u64,
    pub mint_a:Pubkey,
    pub mint_b:Pubkey,
    pub bump:u8,
    pub demand:u64,
    pub signer:Pubkey

}