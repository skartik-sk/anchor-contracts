use anchor_lang::accounts::signer;
use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;
use anchor_lang::system_program::transfer;

use crate::VaultState;

#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(
        mut
    )]
    pub signer:Signer<'info>,

    #[account(

        seeds=[b"state",signer.key().as_ref()],
        bump
    )]

    pub vault_state:Account<'info,VaultState>,

    #[account(
        mut,
       seeds= [b"vault",vault_state.key().as_ref()],
       bump

    )]
    pub vault:SystemAccount<'info>,

    pub system_program:Program<'info,System>
    

}



impl <'info> Deposit<'info> {
    pub fn deposit(&mut self,amount:u64)->Result<()>{
        let sp = self.system_program.to_account_info();

        let tx = Transfer{
            from:self.signer.to_account_info(),
            to:self.vault.to_account_info()
        };

        let tra = CpiContext::new(sp, tx);

        transfer(tra, amount)?;

        Ok(())


    }
}