use anchor_lang::prelude::{borsh::de, *};

declare_id!("CcBa7Z5gMWVT5sBidifmdZVLuJbXBe9eSQCds4oWuZRt");

#[program]
pub mod calculator {

    use super::*;

    pub fn init(ctx:Context<Initialize>,initval:u32)->Result<()>{
        ctx.accounts.account.num = initval;

        Ok(())

    }

    pub fn add(ctx:Context<Add>,number:u32)->Result<()>{

        let data = &mut ctx.accounts.account;
        data.num += number;
      Ok(())
    }

    pub fn double(ctx:Context<Double>,)->Result<()>{
        let data = &mut ctx.accounts.account;
        data.num *=2;
      Ok(())
    }
    pub fn subtract(ctx:Context<Subtract>,number:u32)->Result<()>{
        let data = &mut ctx.accounts.account;
        data.num -= number;
      Ok(())
    }


}



#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init,
        payer = signer,
        space = 8 + 4 
    )]
    pub account:Account<'info,Data>,
  
    pub system_program: Program<'info, System>,
#[account(mut)]
    pub signer:Signer<'info>,
}


#[derive(Accounts)]
pub struct Add<'info>{
    #[account(
       mut)]
    pub account:Account<'info,Data>,
  
#[account(mut)]
    pub signer:Signer<'info>,
}

#[derive(Accounts)]
pub struct Double<'info>{
    #[account(
       mut)]
    pub account:Account<'info,Data>,
  
#[account(mut)]
    pub signer:Signer<'info>,
}


#[derive(Accounts)]
pub struct Subtract<'info>{
    #[account(
       mut)]
    pub account:Account<'info,Data>,
  
#[account(mut)]
    pub signer:Signer<'info>,
}



#[account]
pub struct Data{
    pub num:u32
}



