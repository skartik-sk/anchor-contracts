use anchor_lang::prelude::*;

use crate::VaultState;



#[derive(Accounts)]
pub struct Init<'info>{
    #[account(
        mut
    )]
    pub signer:Signer<'info>,

    #[account(
        init,
        payer=signer,
        space=VaultState::INIT_SPACE+8,
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

impl <'info> Init<'info> {
    pub fn init(&mut self,bumps:InitBumps)->Result<()>{
        self.vault_state.vault_bump=bumps.vault;
        self.vault_state.state_bump=bumps.vault_state;
        Ok(())


    }
}


