// use num_bigint::BigInt;
// use once_cell::sync::Lazy;
// use std::ops::Neg;



// This is the lower BOND :  for actual synthetic asset and limit order collateral amounts. Those
// amounts can't be 0 to prevent order replay and arbitrary actual fees.
pub const POSITIVE_AMOUNT_LOWER_BOUND: u64 = 1;

// A valid balance satisfies BALANCE_LOWER_BOND :  < balance < BALANCE_UPPER_BOUND : .
pub const BALANCE_UPPER_BOUND: i128 = 1 << 63;
pub const BALANCE_LOWER_BOUND: i128 = -BALANCE_UPPER_BOUND;

pub const TOTAL_VALUE_UPPER_BOUND: i128 = 1 << 63;
pub const TOTAL_VALUE_UPPER_BOUND_SHIFT_32: i128 = TOTAL_VALUE_UPPER_BOUND << 32;
pub const TOTAL_VALUE_UPPER_BOUND_SHIFT_63: i128 = TOTAL_VALUE_UPPER_BOUND<<63;
pub const TOTAL_VALUE_LOWER_BOUND: i128 = -TOTAL_VALUE_UPPER_BOUND;
pub const TOTAL_VALUE_LOWER_BOUND_SHIFT_32: i128 = -TOTAL_VALUE_UPPER_BOUND_SHIFT_32;
pub const TOTAL_VALUE_LOWER_BOUND_SHIFT_63: i128 = -TOTAL_VALUE_UPPER_BOUND_SHIFT_63;

pub const TOTAL_RISK_UPPER_BOUND: u128 = u128::MAX;

pub const POSITION_MAX_SUPPORTED_N_ASSETS: u64 = 1 << 6;

// Fixed point (.32) representation of the number 1.
// changing FXP_32_ONE must changing RISK_FACTOR_UPPER_BOUND Synchronously
// pub static FXP_32_ONE: BigInt = BigInt::from(1 << 32);
pub static FXP_32_ONE: i128 = 1 << 32;

pub const SHIFT_32: usize = 32;

//pub const ORDER_ID_UPPER_BOUND: u128 = 1<< 64;

// General Cairo pub constants.
//pub const SIGNED_MESSAGE_BOUND: Lazy<BigInt> = Lazy::new(|| BigInt::from(2).pow(251));
//pub static RANGE_CHECK_BOUND: Lazy<BigInt> = Lazy::new(|| BigInt::from(2).pow(128));
