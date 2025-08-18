use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{
    assert_non_zero, assert_not_loacked,
    constant::{LIQUID_POOL, LPMINT},
    errors::LPErrors,
    state::Pool,
};

// Accounts
// - swaping user
// - mint_x
// - mint_y
// - pool_account
// - vaul_x
// - vaul_y
// - swaper_token_x
// - swaper_token_y
// - lp_mint_account

// X = Pop-cat Coin 🙀(20)
// Y = Bonk Coin 🐶(8)

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    pub mint_y: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [LPMINT,pool_account.key().to_bytes().as_ref()],
        bump = pool_account.lp_mint_bump
    )]
    pub lp_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [LIQUID_POOL,pool_account.seed.to_le_bytes().as_ref()],
        bump = pool_account.pool_bump,
        has_one = mint_x,
        has_one = mint_y,
    )]
    pub pool_account: Account<'info, Pool>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = pool_account,
    )]
    pub vault_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = pool_account,
    )]
    pub vault_y: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,

    )]
    pub user_token_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,

    )]
    pub user_token_y: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, is_x: bool, amount: u64, min_slippage_amount: u64) -> Result<()> {
        // - lets say user wants some pop-cat(x) coins by swaping with bonk coins(y)
        // - we have to find no.of bonk coins for "n" number of pop-cat coins so that with keeping slippage in mind (x*y = k)
        // - then we should check for slippage, If slippage occured then stop trx
        // - after checks transfer bonk coin to user and add pop-cat coin and remove bonk coin from liquidity.

        assert_not_loacked!(self.pool_account.locked);
        assert_non_zero!([amount]);

        // CHECK: Do try this code while testing
        //   let (withdraw_amount) =  match is_x {
        //         true => {
        //             let amount_y = ConstantProduct::y2_from_x_swap_amount(self.vault_x.amount, self.vault_y.amount, amount).unwrap();
        //             amount_y
        //         },
        //         false => {
        //             let amount_x = ConstantProduct::x2_from_y_swap_amount(self.vault_x.amount, self.vault_y.amount, amount).unwrap();
        //             amount_x
        //         }
        //     };

        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.lp_mint.supply,
            self.pool_account.fee,
            None,
        )
        .unwrap();

        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let amount_curve = curve.swap(p, amount, min_slippage_amount).unwrap();

        // - Hear amount_curve.

        self.deposit_token(is_x, amount_curve.deposit)?;
        self.withdraw_token(is_x, amount_curve.withdraw)?;

        Ok(())
    }

    fn deposit_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let mint;

        let (from, to, decimal) = match is_x {
            true => {
                mint = self.mint_x.to_account_info();
                (
                    self.user_token_x.to_account_info(),
                    self.vault_x.to_account_info(),
                    self.mint_x.decimals,
                )
            }

            false => {
                mint = self.mint_y.to_account_info();
                (
                    self.user_token_y.to_account_info(),
                    self.vault_y.to_account_info(),
                    self.mint_y.decimals,
                )
            }
        };

        let accounts = TransferChecked {
            authority: self.user.to_account_info(),
            from, // token_account
            to,   // vault
            mint,
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, amount, decimal)?;

        Ok(())
    }

    fn withdraw_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let mint;
        let (from, to, decimal) = match is_x {
            true => {
                mint = self.mint_x.to_account_info();
                (
                    self.vault_y.to_account_info(),
                    self.user_token_y.to_account_info(),
                    self.mint_x.decimals,
                )
            }
            false => {
                mint = self.mint_y.to_account_info();
                (
                    self.vault_x.to_account_info(),
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
}
