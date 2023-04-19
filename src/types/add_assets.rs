use super::position::{create_maybe_empty_position, Position};
use crate::types::defined_types::{AssetIdType, IndexType};
use crate::types::packed_public_key::PublicKeyType;
use crate::types::constants::POSITION_MAX_SUPPORTED_N_ASSETS;
use crate::types::objects::FundingIndex;
use crate::types::{objects::FundingIndicesInfo, perp_error::PerpError};
use crate::types::position::{check_valid_balance, PositionAsset};
use num_bigint::BigInt;
use num_traits::Zero;

// Fetches the balance and cached funding index of a position asset if found.
// Otherwise, returns 0 balance and fetches funding index from global_funding_indices.
fn get_old_asset(
    asset_ptr: &PositionAsset,
    asset_found: bool,
    global_funding_indices: &FundingIndicesInfo,
    asset_id: &AssetIdType,
) -> Result<(BigInt, IndexType), PerpError> {
    if asset_found {
        // Asset found.
        return Ok((asset_ptr.balance.clone(), asset_ptr.cached_funding_index));
    }

    // Previous asset missing => initial balance is zero.
    // Find funding index.
    let mut i: usize = 0;
    let mut found_funding_index: FundingIndex = FundingIndex {
        asset_id: asset_id.clone(),
        funding_index: 0,
    };
    while i < global_funding_indices.funding_indices.len() {
        if global_funding_indices.funding_indices[i].asset_id == asset_id.clone() {
            found_funding_index = global_funding_indices.funding_indices[i].clone();
            break;
        }
        i += 1;
    }

    if i >= global_funding_indices.funding_indices.len() {
        return Err(PerpError::Error);
    }

    return Ok((BigInt::zero(), found_funding_index.funding_index));
}

// Builds the result position assets array after adding delta to the original assets array at
// asset_id. if new balance is zero then delete the asset
fn add_asset_inner(
    assets_ptr: &mut Vec<PositionAsset>,
    global_funding_indices: &FundingIndicesInfo,
    asset_id: &AssetIdType,
    delta: BigInt,
) -> Result<usize, PerpError> {
    // Split original assets array, around asset_id.
    // left_end_ptr is the pointer before current asset.
    let mut left_end_ptr = &PositionAsset {
        balance: Default::default(),
        asset_id: asset_id.clone(),
        cached_funding_index: 0,
    };
    let mut found: bool = false;
    let mut i: usize = 0;
    while i < assets_ptr.len() {
        if assets_ptr[i].asset_id == asset_id.clone() {
            left_end_ptr = &assets_ptr[i];
            found = true;
            break;
        }
        i += 1;
    }

    let (balance, funding_index) =
        get_old_asset(left_end_ptr, found, global_funding_indices, asset_id)?;

    // Check new balance validity.
    let new_balance = balance + delta;
    check_valid_balance(new_balance.clone())?;

    // Don't write new asset if new balance is 0.
    if new_balance.clone() == BigInt::zero() {
        // Copy right portion.
        assets_ptr.remove(i);
        return Ok(assets_ptr.len());
    }
    if found {
        assets_ptr[i].balance = new_balance;
    } else {
        assets_ptr.push(PositionAsset {
            balance: new_balance,
            asset_id: asset_id.clone(),
            cached_funding_index: funding_index,
        });
    }

    // Copy right portion.
    Ok(assets_ptr.len())
}

// Changes an asset balance of a position by delta. delta may be negative. Handles non existing and
// empty assets correctly. If the position is empty, the new position will have the given public key.
// Assumption: Either public_key matches the position, or position is empty.
pub fn position_add_asset(
    position: &mut Position,
    global_funding_indices: &FundingIndicesInfo,
    asset_id: &AssetIdType,
    delta: BigInt,
    public_key: &PublicKeyType,
) -> Result<Position, PerpError> {
    // Allow invalid asset_id when delta == 0.
    if delta == BigInt::zero() {
        return Ok(position.clone());
    }

    // Call add_asset_inner.
    let res_n_assets = add_asset_inner(
        &mut position.assets,
        global_funding_indices,
        asset_id,
        delta,
    )?;

    // A single position may not contain more than POSITION_MAX_SUPPORTED_N_ASSETS assets. We may
    // assert that (res_n_assets != POSITION_MAX_SUPPORTED_N_ASSETS + 1) instead of
    // (res_n_assets <= POSITION_MAX_SUPPORTED_N_ASSETS) since each transaction adds at most one
    // asset to a position and therefore checking for inequality is equivalent to comparing.
    if res_n_assets as u64 == POSITION_MAX_SUPPORTED_N_ASSETS + 1 {
        return Err(PerpError::TooManySyntheticAssetsInPosition);
    }

    let pos = create_maybe_empty_position(
        public_key,
        position.collateral_balance.clone(),
        &position.assets,
        &position.funding_timestamp,
    );
    Ok(pos)
}
