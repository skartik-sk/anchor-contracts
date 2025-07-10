use anchor_lang::prelude::*;


declare_id!("281ZfcxdLxyZd5b435VyFwPa3GFQMs8GPyWuYgvFJe2V");

pub mod programs;
use crate::programs::*;

#[program]
pub mod vault {
    
  

    use super::*;
    pub fn init(ctx:Context<Init>)-> Result<()>{
       let _ =  ctx.accounts.init(ctx.bumps);
        Ok(())
    }

    pub fn deposit(ctx:Context<Deposit>,amount:u64)->Result<()>{
let _ =  ctx.accounts.deposit(amount);

        Ok(())


    }
    pub fn withdraw(ctx:Context<Withdraw>,amount:u64)->Result<()>{
let _ =  ctx.accounts.withdraw(amount);
        Ok(())

    }

    pub fn close(ctx:Context<Close>)->Result<()>{
let _ =  ctx.accounts.close();
        Ok(())

    }


}

#[account]
#[derive(InitSpace)]
pub struct VaultState{
    pub vault_bump:u8,
    pub state_bump:u8
}

