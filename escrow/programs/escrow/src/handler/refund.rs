use anchor_lang::prelude::*;
use anchor_spl::token::{close_account, CloseAccount};
use anchor_spl::token_interface::{Mint,TokenInterface,transfer_checked,TransferChecked,TokenAccount};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Refund<'info>{

  #[account(mut)]
    pub signer:Signer<'info>,
    pub mint_a:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority= signer
    )]
    pub market_asso_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        close = signer,
        seeds = [b"escrow",signer.key().as_ref(),escrow.ids.to_le_bytes().as_ref()],
        bump = escrow.bump,
        constraint  = (signer.key()==escrow.signer.key())

    )]
    pub escrow:Account<'info,Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority= escrow
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,


    pub token_program : Interface<'info,TokenInterface>,

    pub system_program : Program<'info,System>
}

pub fn refund(context: Context<Refund>)->Result<()> { 


          let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            context.accounts.signer.key.as_ref(),
            &context.accounts.escrow.ids.to_le_bytes()[..],
            &[context.accounts.escrow.bump]
        ]];

        let sp = context.accounts.token_program.to_account_info();
        let sp2= sp.clone();
        
    let tx = TransferChecked{
        from:context.accounts.vault.to_account_info(),
        to:context.accounts.market_asso_a.to_account_info(),
        mint:context.accounts.mint_a.to_account_info(),
        authority:context.accounts.escrow.to_account_info()
    };

    let cpi= CpiContext::new_with_signer(sp, tx,&signer_seeds);
    transfer_checked(cpi, context.accounts.vault.amount, context.accounts.mint_a.decimals);


    let colsetx= CloseAccount{
        account:context.accounts.vault.to_account_info(),
        authority:context.accounts.escrow.to_account_info(),
        destination:context.accounts.signer.to_account_info()
    };

    let cpis = CpiContext::new(sp2, colsetx);

    close_account(cpis)?;

    
    Ok(())

    
}