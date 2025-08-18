use anchor_lang::prelude::*;

#[error_code]
pub enum LPErrors {
    #[msg("pool is locked ")]
    PoolLocked,
    
    #[msg("Error while trying to deposite Token")]
    AMMError,

    #[msg("Slippage Exceed Occured")]
    SlippageExceed,

    #[msg("Provided balance is zero")]
    ZeroBalance,

    #[msg("Invalid admin trying to invoke fuction")]
    InvalidAdmin,

    #[msg("There is no Admin for this liquidity pool")]
    NoAdmin
}
