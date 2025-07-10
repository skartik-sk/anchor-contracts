use anchor_lang::prelude::*;

declare_id!("7xA83viuVsLMN6FQfMtdJY8a9axTW1nqiJTNrHnfpr6K");
pub mod state;
pub mod handler;
pub use handler::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make_escrow(
        context :Context<Make>,
        seeds:u64,
        deposit:u64,
        demand:u64
        

    )->Result<()>
    
     {
        msg!("making account initiated");
        handler::make::intilize_and_deposit(context, seeds, deposit,demand)?;


        


Ok(())
        
    }

    pub fn refund(    context :Context<Refund>,)-> Result<()>{
        msg!("refund initiated");
        handler::refund::refund(context)?;
        Ok(())
    }
    
    pub fn take(    context :Context<Take>,
        
    )->Result<()>{
        msg!("swap inittiated"); 
        handler::take::take(context)?;
        Ok(())
        
    }



}

