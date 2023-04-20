use crate::types::perp_error::PerpError;
use crate::types::position::{Position, PositionAsset};
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use std::ops::Mul;

// Inner function for check_smaller_in_synthetic_holdings_inner. Checks a single asset and then
// recursively checks the rest.
fn check_smaller_in_synthetic_holdings_inner(
    updated_position_assets: &Vec<PositionAsset>,
    initial_position_assets: &Vec<PositionAsset>,
) -> Result<(), PerpError> {
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < initial_position_assets.len() || j < updated_position_assets.len() {
        if i == initial_position_assets.len() {
            return Err(PerpError::IllegalPositionTransitionEnlargingSyntheticHoldings);
        }
        if updated_position_assets[j].asset_id != initial_position_assets[i].asset_id {
            // Because the asset ids are sorted, we can assume that the initial position's asset id
            // doesn't exist in the updated position. (If that isn't true then we will eventually have
            // n_initial_position_assets == 0).
            // This means that the initial position's asset has updated balance 0 and we can skip it.
            i += 1;
            if i == initial_position_assets.len() {
                return Err(PerpError::IllegalPositionTransitionEnlargingSyntheticHoldings);
            }
            continue;
        }

        let updated_balance = &updated_position_assets[j].balance.clone();
        let initial_balance = &initial_position_assets[i].balance.clone();

        // Check that updated_balance and initial_balance have the same sign.
        // They cannot be zero at this point.
        if updated_balance.sign() != initial_balance.sign() {
            return Err(PerpError::IllegalPositionTransitionEnlargingSyntheticHoldings);
        }

        // Check that abs(updated_balance) <= abs(initial_balance) using
        // See the assumption in check_smaller_in_synthetic_holdings.
        if updated_balance.abs() > initial_balance.abs() {
            return Err(PerpError::IllegalPositionTransitionEnlargingSyntheticHoldings);
        }

        i += 1;
        j += 1;
    }

    Ok(())
}

// Checks that updated_position is as safe as the initial position.
// This means that the balance of each asset did not change sign, and its absolute value
// did not increase.
// Returns 1 if the check passes, 0 otherwise.
//
// Assumption:
//    All the asset balances are in the range [BALANCE_LOWER_BOUND, BALANCE_UPPER_BOUND).
//    The position's assets are sorted by asset id.
//    max(BALANCE_LOWER_BOUND**2, (BALANCE_UPPER_BOUND - 1)**2) < range_check_builtin.bound.
pub fn check_smaller_in_synthetic_holdings(
    updated_position: &Position,
    initial_position: &Position,
) -> Result<(), PerpError> {
    // %{
    //     assert
    //     max(
    //         ids.BALANCE_LOWER_BOUND * *2, (ids.BALANCE_UPPER_BOUND - 1) * *2) < range_check_builtin.bound
    //         %
    // }
    check_smaller_in_synthetic_holdings_inner(&updated_position.assets, &initial_position.assets)
}
