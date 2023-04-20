use crate::types::constants::FXP_32_ONE;
use crate::types::constants::SHIFT_32;
use crate::types::constants::TOTAL_RISK_UPPER_BOUND;
use crate::types::constants::{TOTAL_VALUE_LOWER_BOUND, TOTAL_VALUE_LOWER_BOUND_SHIFT_32, TOTAL_VALUE_LOWER_BOUND_SHIFT_63};
use crate::types::constants::{TOTAL_VALUE_UPPER_BOUND, TOTAL_VALUE_UPPER_BOUND_SHIFT_32, TOTAL_VALUE_UPPER_BOUND_SHIFT_63};
use crate::types::config::GeneralConfig;
use crate::types::objects::OraclePrices;
use crate::types::perp_error::PerpError;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};

use super::position::Position;
use super::position::PositionAsset;

fn position_get_status_inner(
    collateral_balance: BigInt,
    assets: &Vec<PositionAsset>,
    oracle_prices: &OraclePrices,
    general_config: &GeneralConfig,
) -> Result<(BigInt, BigInt), PerpError> {
    let mut total_value_rep: BigInt = collateral_balance;
    let mut total_risk_rep: BigInt = BigInt::zero();
    let mut value_rep: BigInt = BigInt::zero();
    let mut risk_rep: BigInt = BigInt::zero();
    for asset in assets.iter() {
        let curr_asset_id = &asset.asset_id;
        let balance = asset.balance.clone();
        for oracle_price in oracle_prices.data.iter() {
            if oracle_price.asset_id == curr_asset_id.clone() {
                // Signed (96.32) fixed point.
                value_rep = balance.clone() * oracle_price.price.clone();
                total_value_rep += value_rep.clone();
            }
        }
        for synthetic_asset_info in general_config.synthetic_assets_info.iter() {
            if synthetic_asset_info.asset_id == curr_asset_id.clone() {
                let abs_value_rep = value_rep.clone().abs();
                // value_rep is a (96.32) fixed point so risk_rep is a (128.64) fixed point.
                risk_rep = abs_value_rep * synthetic_asset_info.risk_factor.clone();
                total_risk_rep += risk_rep;
            }
        }
    }
    return Ok((total_value_rep, total_risk_rep));
}

// return total_value & total_risk
pub fn position_get_status(
    position: &Position,
    oracle_prices: &OraclePrices,
    general_config: &GeneralConfig,
) -> Result<(BigInt, BigInt), PerpError> {
    let (total_value_rep, total_risk_rep) = match position_get_status_inner(
        &position.collateral_balance * FXP_32_ONE,
        &position.assets,
        oracle_prices,
        general_config,
    ) {
        Ok((tv, tr)) => (tv, tr),
        Err(e) => return Err(e),
    };

    //let (total_value_rep2, total_risk_rep2) = (total_value_rep.to_i128().unwrap(), total_risk_rep.to_i128().unwrap());

    let total_value_lower_bound_rep = BigInt::from(TOTAL_VALUE_LOWER_BOUND_SHIFT_32);
    let total_value_upper_bound_rep = BigInt::from(TOTAL_VALUE_UPPER_BOUND_SHIFT_32);
    if total_value_rep < total_value_lower_bound_rep
        || total_value_rep >= total_value_upper_bound_rep
    {
        return Err(PerpError::OutOfRangeTotalValue);
    }

    //#[allow(arithmetic_overflow)]
    //let a: i128 = 1<<64;
    //std::u128::MAX;
    //let a: u128 = -1;
    let tr_upper_bound_rep = BigInt::from(TOTAL_RISK_UPPER_BOUND); //* a;
    //println!("tr_upper_bound_rep: {}", tr_upper_bound_rep);
    if total_risk_rep >= tr_upper_bound_rep {
        return Err(PerpError::OutOfRangeTotalRisk);
    }

    Ok((total_value_rep, total_risk_rep))
}
