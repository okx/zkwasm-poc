use super::position::{position_new, Position, PositionAsset};
use crate::types::defined_types::IndexType;
use crate::types::constants::BALANCE_UPPER_BOUND;
use crate::types::perp_error::PerpError;
use crate::types::constants::FXP_32_ONE;
use crate::types::objects::FundingIndicesInfo;
use num_bigint::BigInt;
use std::ops::Div;

// Computes the total_funding for a given position and updates the cached funding indices.
// The funding per asset is computed as:
//   (global_funding_index - cached_funding_index) * balance.
//
// Arguments:
// assets_before - a pointer to PositionAsset array.
// global_funding_indices - a pointer to a FundingIndicesInfo.
// current_collateral_fxp - Current collateral as signed (.32) fixed point.
// assets_after - a pointer to an output array, which will be filled with
// the same assets as assets_before but with an updated cached_funding_index.
//
// Returns:
// collateral_fxp - The colleteral after the funding was applied as signed (.32) fixed point.
//
// Assumption: current_collateral_fxp does not overflow. It is a sum of 95 bit values, and overflow
//   happens at 251 bits.
// Prover assumption: The assets in assets_before are a subset of the assets in
// global_funding_indices.
fn apply_funding_inner(
    assets_before: &Vec<PositionAsset>,
    global_funding_indices: &FundingIndicesInfo,
    mut current_collateral_fxp: BigInt,
) -> Result<(BigInt, Vec<PositionAsset>), PerpError> {
    let mut assets_after: Vec<PositionAsset> = Vec::new();
    let mut global_funding_index: IndexType;
    for asset in assets_before {
        let current_asset: &PositionAsset = asset;
        let asset_id = &current_asset.asset_id;

        if let Some(index) = global_funding_indices
            .funding_indices
            .iter()
            .find(|&index| index.asset_id == asset_id.clone())
        {
            global_funding_index = index.funding_index;
        } else {
            return Err(PerpError::MissingGlobalFundingIndex);
        }

        // Compute fixed point fxp_delta_funding := delta_funding_index * balance.
        let balance = current_asset.balance.clone();
        let delta_funding_index = global_funding_index - current_asset.cached_funding_index;
        let fxp_delta_funding = delta_funding_index * balance.clone();

        // Copy asset to assets_after with an updated cached_funding_index.
        assets_after.push(PositionAsset {
            balance,
            asset_id: asset_id.clone(),
            cached_funding_index: global_funding_index,
        });
        current_collateral_fxp -= fxp_delta_funding;
    }

    Ok((current_collateral_fxp, assets_after))
}

// Change the cached funding indices in the position into the updated funding indices and update the
// collateral balance according to the funding diff.
pub fn position_apply_funding(
    position: &Position,
    global_funding_indices: &FundingIndicesInfo,
) -> Result<Position, PerpError> {
    let (collateral_fxp, new_assets) = match apply_funding_inner(
        &position.assets,
        global_funding_indices,
        position.collateral_balance.clone() * FXP_32_ONE.clone(),
    ) {
        Ok((collateral, assets)) => (collateral, assets),
        Err(e) => return Err(e),
    };

    // The collateral changes due to funding over all positions always sum up to 0
    // (Assuming no rounding). Therefore the collateral delta is rounded down to make sure funding
    // does not make collateral out of thin air.
    // For example if we have 3 users a, b and c and the computed funding is as follows:
    // a = -0.5, b = -0.5, c = 1, we round the funding down to a = -1, b = -1 and c = 1 and therefore
    // we lose 1 collateral in the system from funding
    // (If instead we rounded up we would've created 1).
    let new_collateral_balance = collateral_fxp.div(FXP_32_ONE.clone());

    if BigInt::from(-BALANCE_UPPER_BOUND) > new_collateral_balance
        && new_collateral_balance >= BigInt::from(BALANCE_UPPER_BOUND)
    {
        return Err(PerpError::InvalidCollateralBalance);
    }

    return Ok(position_new(
        &position.public_key,
        new_collateral_balance,
        &new_assets,
        &global_funding_indices.funding_timestamp,
    ));
}
