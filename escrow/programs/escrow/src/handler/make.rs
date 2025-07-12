
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint,TokenInterface,transfer_checked,TransferChecked,TokenAccount};

use crate::state::Escrow;


#[derive(Accounts)]
#[instruction(ids:u64)]

pub struct Make<'info>{

    #[account(mut)]
    pub signer:Signer<'info>,
    pub mint_a:InterfaceAccount<'info,Mint>,
    pub mint_b:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority= signer
    )]
    pub market_asso_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer= signer,
        space= 8+ Escrow::INIT_SPACE,
        seeds = [b"escrow",signer.key().as_ref(),ids.to_le_bytes().as_ref()],
        bump

    )]
    pub escrow:Account<'info,Escrow>,
    #[account(
        init,
        payer= signer,
        associated_token::mint = mint_a,
        associated_token::authority= escrow
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program : Program<'info,AssociatedToken>,

    pub token_program : Interface<'info,TokenInterface>,

    pub system_program : Program<'info,System>
}

pub fn intilize_and_deposit( context :Context<Make>,
        ids:u64,
        deposit:u64,
        demand:u64
        )->Result<()> {
            msg!("started initilizing ");
            
            context.accounts.escrow.set_inner(Escrow { ids: ids, mint_a: context.accounts.mint_a.key(), mint_b: context.accounts.mint_b.key(), bump: context.bumps.escrow, demand: demand , signer: context.accounts.signer.key() });
            msg!(" initilizing done ");
            
            deposit_to_vault(context, deposit)?;
            Ok(())
            
            
        }
        
        pub fn deposit_to_vault(context :Context<Make>,
            deposit:u64)->Result<()> {
                msg!("started vault deposit ");
                let tsxacc = TransferChecked{
                    from:context.accounts.market_asso_a.to_account_info(),
                    to:context.accounts.vault.to_account_info(),
                    mint:context.accounts.mint_a.to_account_info(),
                    authority:context.accounts.signer.to_account_info()
                    
                    
                };
                
                let cpi = CpiContext::new(context.accounts.token_program.to_account_info(),tsxacc);
                
                
                transfer_checked(cpi, deposit, context.accounts.mint_a.decimals)?;
                msg!(" deposit done ");
        Ok(())    
    
}
