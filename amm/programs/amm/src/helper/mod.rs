#[macro_export]
macro_rules! assert_not_loacked {
    ($lock: expr) => {
        if $lock == true {
            return err!(LPErrors::PoolLocked);
        }
    };
}

#[macro_export]
macro_rules! assert_non_zero {
    ($arr: expr) => {
        if $arr.contains(&0u64) {
            return err!(LPErrors::ZeroBalance);
        }
    };
}

#[macro_export]
macro_rules! check_admin {
    ($var: expr) => {
        match $var.pool_account.authority {
            Some(admin_pubkey) => {
                require!(admin_pubkey == $var.user.key(), LPErrors::InvalidAdmin);
            }
            None => {
                return err!(LPErrors::NoAdmin);
            }
        }
    };
}
