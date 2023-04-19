use num_bigint::BigInt;
use once_cell::sync::Lazy;

pub static AMOUNT_UPPER_BOUND: Lazy<BigInt> = Lazy::new(|| BigInt::from(2).pow(64));