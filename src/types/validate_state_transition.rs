use crate::types::constants::FXP_32_ONE;
use crate::types::perp_error::PerpError;
use crate::types::config::GeneralConfig;
use crate::types::objects::OraclePrices;
use crate::types::check_smaller_holdings::check_smaller_in_synthetic_holdings;
use crate::types::status::position_get_status;
use num_bigint::BigInt;
use num_traits::Zero;

use super::position::Position;

// Checks if a position update was legal.
// A position update is legal if
//   1. The result position is well leveraged, or
//   2. a. The result position is `smaller` than the original position, and
//      b. The ratio between the total_value and the total_risk in the result position is not
//         smaller than the same ratio in the original position, and
//      c. If the total risk of the original position is 0, the total value of the result
//         position is not smaller than the total value of the original position.
pub fn check_valid_transition(
    updated_position: &Position,
    initial_position: &Position,
    oracle_prices: &OraclePrices,
    general_config: &GeneralConfig,
) -> Result<(), PerpError> {
    let (updated_tv, updated_tr) =
        position_get_status(&updated_position, &oracle_prices, &general_config)?;

    // is well leveraged
    if updated_tr <= (updated_tv.clone() * FXP_32_ONE.clone()) {
        return Ok(());
    }

    let (initial_tv, initial_tr) =
        position_get_status(initial_position, oracle_prices, general_config)?;

    check_smaller_in_synthetic_holdings(updated_position, initial_position).map_err(|e| {
        // println!(
        //     "check_valid_transition initial position: {:?}, updated position: {:?}",
        //     initial_position, updated_position
        // );
        e
    })?;

    // total_value / total_risk must not decrease.
    // tv0 / tr0 <= tv1 / tr1 iff tv0 * tr1 <= tv1 * tr0.
    // tv is 96 bit.
    // tr is 128 bit.
    // tv*tr fits in 224 bits.
    // Since tv can be negative, adding 2**224 to each side.

    // TODO
    // let (success) = is_le_felt{range_check_ptr=range_check_ptr}(
    //     2**224 + initial_tv * updated_tr, 2**224 + updated_tv * initial_tr)
    if initial_tv.clone() * updated_tr.clone() > updated_tv.clone() * initial_tr.clone() {
        return Err(PerpError::IllegalPositionTransitionReducingTotalValueRiskRatio);
    }

    if initial_tr == BigInt::zero() {
        // Edge case: When the total risk is 0 the TV/TR ratio is undefined and we need to check that
        // initial_tv <= updated_tv. Note that because we passed
        // 'check_smaller_in_synthetic_holdings' and initial_tr == 0 we must have updated_tr == 0.
        if initial_tv > updated_tv {
            return Err(PerpError::IllegalPositionTransitionNoRiskReducedValue);
        }
    }

    Ok(())
}
