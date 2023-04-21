use crate::types::defined_types::*;
use crate::types::order::OrderBase;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use crate::types::perp_error::PerpError;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LimitOrder {
    pub base: OrderBase,
    pub amount_synthetic: BigInt,
    pub amount_collateral: BigInt,
    pub amount_fee: BigInt,
    pub asset_id_synthetic: AssetIdType,
    pub asset_id_collateral: AssetIdType,
    pub position_id: PositionIdType,
    pub is_buying_synthetic: bool,
    pub order_type: OrderType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    LimitOrderWithFees,
}

impl Default for OrderType {
    fn default() -> Self {
        Self::LimitOrderWithFees
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmountInfo {
    pub order_id: OrderIdType,
    pub amount: BigInt,
}

pub fn validate_limit_order_fairness(
    limit_order: &LimitOrder,
    actual_collateral: &BigInt,
    actual_synthetic: &BigInt,
    actual_fee: &BigInt,
) -> Result<(), PerpError> {
    let amount_collateral = &limit_order.amount_collateral;

    // actual_fee / actual_collateral <= amount_fee / amount_collateral, thus
    // actual_fee * amount_collateral <= amount_fee * actual_collateral.
    if actual_fee * &limit_order.amount_collateral
        > &limit_order.amount_fee * actual_collateral
    {
        // println!(
        //     "INVALID_FULFILLMENT_FEE_RATIO limit order {:?}, actual_collateral {}, actual_fee {}",
        //     limit_order.clone(),
        //     actual_collateral.clone(),
        //     actual_fee.clone()
        // );
        return Err(PerpError::InvalidFulfillmentFeeRatio);
    }

    if limit_order.is_buying_synthetic {
        let actual_sold = actual_collateral;
        let actual_bought = actual_synthetic;
        let amount_sell = amount_collateral;
        let amount_buy = &limit_order.amount_synthetic;

        if actual_sold.is_zero() {
            return Ok(());
        }

        if (actual_sold - 1) * amount_buy >= amount_sell * actual_bought {
            // println!("INVALID_FULFILLMENT_ASSETS_RATIO ('actual_sold'-1):{}, amount_buy:{}, amount_sell:{}, actual_bought:{}", actual_sold - 1, amount_buy, amount_sell, actual_bought);
            return Err(PerpError::InvalidFulfillmentAssetsRatio);
        }
        return Ok(());
    }

    let actual_sold = actual_synthetic;
    let actual_bought = actual_collateral;
    let amount_sell = &limit_order.amount_synthetic;
    let amount_buy = amount_collateral;
    if actual_sold * amount_buy > amount_sell * (actual_bought + 1)
    {
        // println!("INVALID_FULFILLMENT_ASSETS_RATIO amount_sell:{}, (actual_bought+1):{}, actual_sold:{}, amount_buy:{}", amount_sell, (actual_bought + 1), actual_sold, amount_buy);
        return Err(PerpError::InvalidFulfillmentAssetsRatio);
    }
    Ok(())
}