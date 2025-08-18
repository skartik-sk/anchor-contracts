use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        burn, mint_to, transfer_checked, Burn, Mint, MintTo, TokenAccount, TokenInterface,
        TransferChecked,
    },
};
use constant_product_curve::ConstantProduct;

use crate::{
    constant::{LIQUID_POOL, LPMINT},
    state::Pool,
};

// accounts
// - user
// - user_token_x
// - user_token_y
// - vault_token_x
// - vault_token_y
// - mint_x
// - mint_y
// - pool_account
// - lp_mint_account
// - lp_user_token_account

#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    pub mint_y: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [LIQUID_POOL,pool_account.seed.to_le_bytes().as_ref()],
        bump = pool_account.pool_bump
    )]
    pub pool_account: Box<Account<'info, Pool>>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_token_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_token_y: Box<InterfaceAccount<'info, TokenAccount>>,

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
       mut,
        associated_token::mint = lp_mint,
        associated_token::authority = user
    )]
    pub user_lp_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> WithdrawTokens<'info> {
    pub fn withdraw(
        &mut self,
        min_x: u64,     // Minimum X withdraw
        min_y: u64,     // Minimum Y withdraw
        amount_lp: u64, // Amount of lp token needs to be burn
    ) -> Result<()> {
        let XYAmount = ConstantProduct::xy_withdraw_amounts_from_l(
            min_x,
            min_y,
            self.lp_mint.supply,
            amount_lp,
            self.lp_mint.decimals as u32,
        )
        .unwrap();

        self.withdraw_tokens(true, XYAmount.x)?;
        self.withdraw_tokens(false, XYAmount.y)?;
        self.burn_lp_tokens(amount_lp)?;

        Ok(())
    }

    fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let mint;
        let (from, to, decimal) = match is_x {
            true => {
                mint = self.mint_x.to_account_info();
                (
                    self.token_vault_x.to_account_info(),
                    self.user_token_y.to_account_info(),
                    self.mint_x.decimals,
                )
            }
            false => {
                mint = self.mint_y.to_account_info();
                (
                    self.token_vault_x.to_account_info(),
                    self.user_token_x.to_account_info(),
                    self.mint_x.decimals,
                )
            }
        };

        let accounts = TransferChecked {
            authority: self.pool_account.to_account_info(),
            from,
            to,
            mint,
        };

        let pool_seed = self.pool_account.seed.to_le_bytes();
        let seeds = &[
            LIQUID_POOL,
            pool_seed.as_ref(),
            &[self.pool_account.pool_bump],
        ];
        let signer_seed = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seed,
        );

        transfer_checked(ctx, amount, decimal)?;

        Ok(())
    }

    fn burn_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let accounts = Burn {
            mint: self.lp_mint.to_account_info(),
            from: self.user_lp_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        burn(ctx, amount)?;
        Ok(())
    }
}
