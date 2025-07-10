use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{close_account, CloseAccount};
use anchor_spl::token_interface::{Mint,TokenInterface,transfer_checked,TransferChecked,TokenAccount};

use crate::state::Escrow;



#[derive(Accounts)]
pub struct Take<'info>{
  
    #[account(mut)]
    pub signer:Signer<'info>,

#[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_a:InterfaceAccount<'info,Mint>,
    pub mint_b:InterfaceAccount<'info,Mint>,
 #[account(
       init_if_needed,
       payer=signer,
        associated_token::mint = mint_a,
        associated_token::authority= signer
    )]
    pub self_asso_a:InterfaceAccount<'info,TokenAccount>,

    #[account(mut,
        associated_token::mint = mint_b,
        associated_token::authority= token_program
    )]
    pub self_ass_b:InterfaceAccount<'info,TokenAccount>,

    #[account(
       init_if_needed,
       payer=signer,
        associated_token::mint = mint_b,
        associated_token::authority= signer
    )]
    pub market_asso_b:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        close = signer,
        has_one = mint_a,
        has_one = mint_b,
        has_one = signer,
        seeds = [b"escrow",signer.key().as_ref(),escrow.ids.to_le_bytes().as_ref()],
        bump = escrow.bump,

    )]
    pub escrow:Account<'info,Escrow>,
    #[account(
        mut,
                associated_token::mint = mint_a,
        associated_token::authority= escrow
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program : Interface<'info,TokenInterface>,

    pub system_program : Program<'info,System>
}

pub fn take(context:Context<Take>)->Result<()> {

   let acc =  TransferChecked{
    from:context.accounts.market_asso_b.to_account_info(),
    to:context.accounts.maker.to_account_info(),
    mint:context.accounts.mint_b.to_account_info(),
    authority:context.accounts.signer.to_account_info(),

    };
    let cpi =CpiContext::new(context.accounts.token_program.to_account_info(), acc);

    transfer_checked(cpi, context.accounts.escrow.demand, context.accounts.mint_b.decimals)?;
    

    refund(context)?;
    Ok(())
}

pub fn refund(context: Context<Take>)->Result<()> { 


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
        to:context.accounts.market_asso_b.to_account_info(),
        mint:context.accounts.mint_a.to_account_info(),
        authority:context.accounts.escrow.to_account_info()
    };

    let cpi= CpiContext::new_with_signer(sp, tx,&signer_seeds);
    transfer_checked(cpi, context.accounts.vault.amount, context.accounts.mint_a.decimals)?;


    let colsetx= CloseAccount{
        account:context.accounts.vault.to_account_info(),
        authority:context.accounts.escrow.to_account_info(),
        destination:context.accounts.signer.to_account_info()
    };

    let cpis = CpiContext::new(sp2, colsetx);

    close_account(cpis)?;

    
    Ok(())

    
}