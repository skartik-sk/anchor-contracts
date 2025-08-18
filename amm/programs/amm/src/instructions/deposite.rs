use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};

// Providing the liquidity hear

use crate::{
    constant::{LIQUID_POOL, LPMINT},
    errors::LPErrors,
    state::Pool,
};

#[derive(Accounts)]
pub struct DepositePair<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // Liquidity Provider

    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    pub mint_y: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_token_x: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_token_y: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        has_one = mint_x,
        has_one = mint_y,
        seeds = [LIQUID_POOL,pool_account.seed.to_le_bytes().as_ref()],
        bump = pool_account.pool_bump
    )]
    pub pool_account: Box<Account<'info, Pool>>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = pool_account,
    )]
    pub token_vault_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = pool_account,
    )]
    pub token_vault_y: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LPMINT,pool_account.key().to_bytes().as_ref()],
        bump = pool_account.lp_mint_bump
    )]
    pub lp_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = lp_mint,
        associated_token::authority = user
    )]
    pub user_lp_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> DepositePair<'info> {
    pub fn deposite(&mut self, lp_amount: u64, max_token_x: u64, max_token_y: u64) -> Result<()> {
        let (amount_x, amount_y) = match self.lp_mint.supply == 0
            && self.token_vault_x.amount == 0
            && self.token_vault_y.amount == 0
        {
            true => (max_token_x, max_token_y),
            false => {
                // Taking the token pair in exact ratio
                let amounts = constant_product_curve::ConstantProduct::xy_deposit_amounts_from_l(
                    max_token_x,
                    max_token_y,
                    self.lp_mint.supply,
                    lp_amount,
                    self.lp_mint.decimals as u32,
                )
                .unwrap();

                (amounts.x, amounts.y)
            }
        };

        require!(
            amount_x <= max_token_x && amount_y <= max_token_y,
            LPErrors::SlippageExceed
        );
        self.deposite_token(true, amount_x)?;
        self.deposite_token(false, amount_y)?;
        self.mint_lp(lp_amount)?;

        Ok(())
    }

    pub fn deposite_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let mint;
        let decimals: u8;

        let (from, to) = match is_x {
            true => {
                decimals = self.mint_x.decimals;
                mint = self.mint_x.to_account_info();

                (
                    self.user_token_x.to_account_info(),
                    self.token_vault_x.to_account_info(),
                )
            }
            false => {
                decimals = self.mint_y.decimals;
                mint = self.mint_y.to_account_info();
                (
                    self.user_token_y.to_account_info(),
                    self.token_vault_y.to_account_info(),
                )
            }
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            authority: self.user.to_account_info(),
            from,
            mint,
            to,
        };

        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(ctx, amount, decimals)?;

        Ok(())
    }

    pub fn mint_lp(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let accounts = MintTo {
            authority: self.pool_account.to_account_info(),
            mint: self.lp_mint.to_account_info(),
            to: self.user.to_account_info(),
        };

        let pool_seeds = self.pool_account.seed.to_le_bytes();

        let seeds = &[
            LIQUID_POOL,
            pool_seeds.as_ref(),
            &[self.pool_account.pool_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, accounts, signer_seeds);
        mint_to(ctx, amount)?;
        Ok(())
    }
}
