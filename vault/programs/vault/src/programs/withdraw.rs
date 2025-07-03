use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::VaultState;

#[derive(Accounts)]
pub struct Withdraw<'info>{
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



impl <'info> Withdraw<'info> {
    pub fn withdraw(&mut self,amount:u64)->Result<()>{

 let sp = self.system_program.to_account_info();

        let tx = Transfer{
            from:self.vault.to_account_info(),
            to:self.signer.to_account_info()
        };

        let vault_sate_key= self.vault_state.to_account_info().key();

        let seeds = &[b"vault",vault_sate_key.as_ref(),&[self.vault_state.vault_bump]];

        let signer_seed = &[&seeds[..]];

        let tra = CpiContext::new_with_signer(sp, tx,signer_seed);

        transfer(tra, amount)?;

        Ok(())
        
    



     


    }
}