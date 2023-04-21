use crate::types::add_assets::position_add_asset;
use crate::types::funding::position_apply_funding;
use crate::types::position::{position_add_collateral, Position, PositionDictAccess};
use crate::types::validate_state_transition::check_valid_transition;
use crate::types::defined_types::{AssetIdType, PositionIdType};
use crate::types::packed_public_key::PublicKeyType;
use crate::types::config::GeneralConfig;
use crate::types::objects::{FundingIndicesInfo, OraclePrices};
use crate::types::perp_error::PerpError;
use crate::types::position::check_request_public_key;
use num_bigint::BigInt;
use num_traits::Zero;

pub const NO_SYNTHETIC_DELTA_ASSET_ID: i64 = -1;

fn is_asset_id_tradable(
    synthetic_asset_id: &AssetIdType,
    synthetic_delta: BigInt,
    global_funding_indices: &FundingIndicesInfo,
    oracle_prices: &OraclePrices,
) -> Result<(), PerpError> {
    if synthetic_asset_id.clone() == NO_SYNTHETIC_DELTA_ASSET_ID.clone() {
        if synthetic_delta == BigInt::zero() {
            return Ok(());
        }
        return Err(PerpError::Error);
    }
    let mut oracle_found = false;
    for v in oracle_prices.data.iter() {
        if v.asset_id == synthetic_asset_id.clone() {
            oracle_found = true;
            break;
        }
    }
    if !oracle_found {
        return Err(PerpError::MissingOraclePrice);
    }

    let mut funding_found = false;
    for v in global_funding_indices.funding_indices.iter() {
        if v.asset_id == synthetic_asset_id.clone() {
            funding_found = true;
            break;
        }
    }
    if !funding_found {
        return Err(PerpError::MissingGlobalFundingIndex);
    }
    Ok(())
}

// Updates the position with collateral_delta and synthetic_delta and returns the updated position.
// Checks that the transition is valid.
// If the transition is invalid or a failure occured, returns the funded position and a return code
// reporting the problem.
// If the given public key is 0, skip the public key validation and validate instead that the
// position's public key isn't 0. It can be 0 if both synthetic_delta and collateral_delta are 0.
// Returns the initial position, the updated position and the initial position after funding was
// applied.
// TODO: it is confusing that return funded_position even if failed
pub fn update_position(
    initial_position: &Position,
    request_public_key: &PublicKeyType,
    collateral_delta: &BigInt,
    synthetic_asset_id: &AssetIdType,
    synthetic_delta: &BigInt,
    global_funding_indices: &FundingIndicesInfo,
    oracle_prices: &OraclePrices,
    general_config: &GeneralConfig,
) -> Result<(Position, Position), (Position, PerpError)> {
    let funded_position = position_apply_funding(initial_position, global_funding_indices)
        .map_err(|e| (initial_position.clone(), e))?;

    is_asset_id_tradable(
        synthetic_asset_id,
        synthetic_delta.clone(),
        global_funding_indices,
        oracle_prices,
    )
        .map_err(|e| (funded_position.clone(), e))?;

    let mut public_key = &PublicKeyType::default(); // TODO check public key

    // Verify public_key.
    if request_public_key.eq(&PublicKeyType::default()) {
        // If request_public_key = 0, We'll take the request public key from the current position.
        if initial_position.public_key.eq(&PublicKeyType::default()) {
            // The current position is empty and we can't take its public key. We need to assert that
            // the new position is also empty because only in that case we don't need the public key.
            if synthetic_delta.is_zero() {
                // println!(
                //     "synthetic_delta: {}, initial position : {:?}",
                //     synthetic_delta, initial_position
                // );
                return Err((funded_position, PerpError::InvalidPublicKey));
            }
            if collateral_delta.is_zero() {
                // println!(
                //     "collateral_delta: {}, initial position : {:?}",
                //     collateral_delta, initial_position
                // );
                return Err((funded_position, PerpError::InvalidPublicKey));
            }
            // There is no change to the position. We can return.
            return Ok((funded_position.clone(), funded_position));
        }
        public_key = &initial_position.public_key;
    } else {
        check_request_public_key(&initial_position.public_key, request_public_key)
            .map_err(|e| (funded_position.clone(), e))?;
        public_key = &request_public_key;
    }

    let mut updated_position =
        position_add_collateral(&funded_position, collateral_delta, public_key)
            .map_err(|e| (funded_position.clone(), e))?;

    let updated_position = position_add_asset(
        &mut updated_position,
        global_funding_indices,
        synthetic_asset_id,
        synthetic_delta.clone(),
        &public_key,
    )
        .map_err(|e| (funded_position.clone(), e))?;

    let final_position = updated_position;

    check_valid_transition(
        &final_position,
        &funded_position,
        oracle_prices,
        general_config,
    )
        .map_err(|e| (funded_position.clone(), e))?;

    Ok((final_position, funded_position))
}

// return updated position, funded position
pub fn update_position_in_dict(
    position_dict: &mut PositionDictAccess,
    position_id: &PositionIdType,
    request_public_key: &PublicKeyType,
    collateral_delta: &BigInt,
    synthetic_asset_id: &AssetIdType,
    synthetic_delta: &BigInt,
    global_funding_indices: &FundingIndicesInfo,
    oracle_prices: &OraclePrices,
    general_config: &GeneralConfig,
) -> Result<(Position, Position), PerpError> {
    let initial_position = position_dict.get_position(position_id)?;
    let (updated_position, funded_position) = update_position(
        &initial_position,
        request_public_key,
        collateral_delta,
        synthetic_asset_id,
        synthetic_delta,
        global_funding_indices,
        oracle_prices,
        general_config,
    )
        .map_err(|e| e.1)?;

    position_dict.update(position_id, &updated_position)?;
    Ok((funded_position, updated_position))
}
