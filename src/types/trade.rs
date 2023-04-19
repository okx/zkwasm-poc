use super::limit_order::LimitOrder;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct Trade {
    pub party_a_order: LimitOrder,
    pub party_b_order: LimitOrder,
    pub actual_collateral: BigInt,
    pub actual_synthetic: BigInt,
    pub actual_a_fee: BigInt,
    pub actual_b_fee: BigInt,
}