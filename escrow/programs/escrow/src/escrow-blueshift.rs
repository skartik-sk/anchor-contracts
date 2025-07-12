use anchor_lang::prelude::*;
use anchor_spl::token::{close_account, CloseAccount};
use anchor_spl::token_interface::{Mint,TokenInterface,transfer_checked,TransferChecked,TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

 

 
declare_id!("22222222222222222222222222222222222222222222");
 
#[program]
pub mod blueshift_anchor_escrow {
    use super::*;
 
    pub fn make(ctx: Context<Make>, seed: u64, recieve: u64, amount: u64) -> Result<()> {
  require_gte!(recieve, 0, EscrowError::InvalidAmount);
  require_gte!(amount, 0, EscrowError::InvalidAmount);
 
  // Save the Escrow Data
  ctx.accounts.populate_escrow(seed, recieve, ctx.bumps.escrow)?;
 
  // Deposit Tokens
  ctx.accounts.deposit_tokens(amount)?;

  Ok(())
    }
 
    pub fn take(ctx: Context<Take>) -> Result<()> {

 ctx.accounts.transfer_to_maker()?;
 
  // Withdraw and close the Vault
  ctx.accounts.withdraw_and_close_vault()?;

  Ok(())
    }
 
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close_vault()?;

  Ok(())
    }
}




 
#[error_code]
pub enum EscrowError {
  #[msg("Invalid amount")]
  InvalidAmount,
  #[msg("Invalid maker")]
  InvalidMaker,
  #[msg("Invalid mint a")]
  InvalidMintA,
  #[msg("Invalid mint b")]
  InvalidMintB,
}
 
 #[account]
#[derive(InitSpace)]
  pub struct Escrow {
  pub seed: u64,
  pub maker: Pubkey,
  pub mint_a: Pubkey,
  pub mint_b: Pubkey,
  pub receive: u64,
  pub bump: u8,
}


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
  #[account(mut)]
  pub maker: Signer<'info>,
  #[account(
    init,
    payer = maker,
    space = 1+Escrow::INIT_SPACE,
    seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
    bump,
  )]
  pub escrow: Account<'info, Escrow>,
 
  /// Token Accounts
  #[account(
    mint::token_program = token_program
  )]
  pub mint_a: InterfaceAccount<'info, Mint>,
  #[account(
    mint::token_program = token_program
  )]
  pub mint_b: InterfaceAccount<'info, Mint>,
  
  #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = maker,
    associated_token::token_program = token_program
  )]
  pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

  #[account(
    init,
    payer = maker,
    associated_token::mint = mint_a,
    associated_token::authority = escrow,
    associated_token::token_program = token_program
  )]
  pub vault: InterfaceAccount<'info, TokenAccount>,
 
  /// Programs
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
  /// # Create the Escrow
  fn populate_escrow(&mut self, seed: u64, amount: u64, bump: u8) -> Result<()> {
    self.escrow.set_inner(Escrow {
      seed,
      maker: self.maker.key(),
      mint_a: self.mint_a.key(),
      mint_b: self.mint_b.key(),
      receive: amount,
      bump,
    });
 
    Ok(())
  }
 
  /// # Deposit the tokens
  fn deposit_tokens(&self, amount: u64) -> Result<()> {
    transfer_checked(
      CpiContext::new(
        self.token_program.to_account_info(),
        TransferChecked {
          from: self.maker_ata_a.to_account_info(),
          mint: self.mint_a.to_account_info(),
          to: self.vault.to_account_info(),
          authority: self.maker.to_account_info(),
      }), amount, self.mint_a.decimals
    )?;
 
    Ok(())
  }
}





#[derive(Accounts)]
pub struct Take<'info> {
#[account(mut)]
pub taker: Signer<'info>,
#[account(mut)]
pub maker: SystemAccount<'info>,
#[account(
  mut,
  close = maker,
  seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
  bump = escrow.bump,
  has_one = maker @ EscrowError::InvalidMaker,
  has_one = mint_a @ EscrowError::InvalidMintA,
  has_one = mint_b @ EscrowError::InvalidMintB,
)]
pub escrow: Box<Account<'info, Escrow>>,
 
/// Token Accounts
pub mint_a: Box<InterfaceAccount<'info, Mint>>,
pub mint_b: Box<InterfaceAccount<'info, Mint>>,
#[account(
  mut,
  associated_token::mint = mint_a,
  associated_token::authority = escrow,
  associated_token::token_program = token_program
)]
pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
#[account(
  init_if_needed,
  payer = taker,
  associated_token::mint = mint_a,
  associated_token::authority = taker,
  associated_token::token_program = token_program
)]
pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,
#[account(
  mut,
  associated_token::mint = mint_b,
  associated_token::authority = taker,
  associated_token::token_program = token_program
)]
pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
#[account(
  init_if_needed,
  payer = taker,
  associated_token::mint = mint_b,
  associated_token::authority = maker,
  associated_token::token_program = token_program
)]
pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
 
/// Programs
pub associated_token_program: Program<'info, AssociatedToken>,
pub token_program: Interface<'info, TokenInterface>,
pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
  fn transfer_to_maker(&mut self) -> Result<()> {
    transfer_checked(
      CpiContext::new(
        self.token_program.to_account_info(),
          TransferChecked {
          from: self.taker_ata_b.to_account_info(),
          to: self.maker_ata_b.to_account_info(),
          mint: self.mint_b.to_account_info(),
          authority: self.taker.to_account_info(),
        },
      ), self.escrow.receive, self.mint_b.decimals
    )?;
 
    Ok(())
  }
 
  fn withdraw_and_close_vault(&mut self) -> Result<()> {
    // Create the signer seeds for the Vault
    let signer_seeds: [&[&[u8]]; 1] = [&[
      b"escrow",
      self.maker.to_account_info().key.as_ref(),
      &self.escrow.seed.to_le_bytes()[..],
      &[self.escrow.bump],
    ]];
 
    // Transfer Token A (Vault -> Taker)
    transfer_checked(
      CpiContext::new_with_signer(
        self.token_program.to_account_info(),
        TransferChecked {
          from: self.vault.to_account_info(),
          to: self.taker_ata_a.to_account_info(),
          mint: self.mint_a.to_account_info(),
          authority: self.escrow.to_account_info(),
        },
        &signer_seeds
      ), self.vault.amount, self.mint_a.decimals
    )?;
 
    // Close the Vault
    close_account(
      CpiContext::new_with_signer(
        self.token_program.to_account_info(),
        CloseAccount {
          account: self.vault.to_account_info(),
          authority: self.escrow.to_account_info(),
          destination: self.maker.to_account_info(),
        },
        &signer_seeds
      )
    )?;
 
    Ok(())
  }
}






#[derive(Accounts)]
pub struct Refund<'info> {

#[account(mut)]
pub maker: SystemAccount<'info>,
#[account(
  mut,
  close = maker,
  seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
  bump = escrow.bump,
  has_one = maker @ EscrowError::InvalidMaker,
  has_one = mint_a @ EscrowError::InvalidMintA,
  has_one = mint_b @ EscrowError::InvalidMintB,
)]
pub escrow: Box<Account<'info, Escrow>>,
 
/// Token Accounts
pub mint_a: Box<InterfaceAccount<'info, Mint>>,
pub mint_b: Box<InterfaceAccount<'info, Mint>>,
#[account(
  mut,
  associated_token::mint = mint_a,
  associated_token::authority = escrow,
  associated_token::token_program = token_program
)]
pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

#[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = maker,
    associated_token::token_program = token_program
  )]
  pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
 
/// Programs
pub associated_token_program: Program<'info, AssociatedToken>,
pub token_program: Interface<'info, TokenInterface>,
pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
 
 
  fn withdraw_and_close_vault(&mut self) -> Result<()> {
    // Create the signer seeds for the Vault
    let signer_seeds: [&[&[u8]]; 1] = [&[
      b"escrow",
      self.maker.to_account_info().key.as_ref(),
      &self.escrow.seed.to_le_bytes()[..],
      &[self.escrow.bump],
    ]];
 
    // Transfer Token A (Vault -> Taker)
    transfer_checked(
      CpiContext::new_with_signer(
        self.token_program.to_account_info(),
        TransferChecked {
          from: self.vault.to_account_info(),
          to: self.maker_ata_a.to_account_info(),
          mint: self.mint_a.to_account_info(),
          authority: self.escrow.to_account_info(),
        },
        &signer_seeds
      ), self.vault.amount, self.mint_a.decimals
    )?;
 
    // Close the Vault
    close_account(
      CpiContext::new_with_signer(
        self.token_program.to_account_info(),
        CloseAccount {
          account: self.vault.to_account_info(),
          authority: self.escrow.to_account_info(),
          destination: self.maker.to_account_info(),
        },
        &signer_seeds
      )
    )?;
 
    Ok(())
  }
}






