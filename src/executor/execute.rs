use crate::types::defined_types::HashType;
use crate::types::constants::POSITIVE_AMOUNT_LOWER_BOUND;
use crate::types::exchange::AMOUNT_UPPER_BOUND;
use crate::types::config::GeneralConfig;
use crate::types::perp_error::PerpError;
use crate::types::limit_order::validate_limit_order_fairness;
use crate::types::order::validate_order_and_update_fulfillment;
use crate::types::limit_order::LimitOrder;
use crate::types::state::CarriedState;
use crate::types::config::BatchConfig;
use crate::executor::update_position::{update_position_in_dict, NO_SYNTHETIC_DELTA_ASSET_ID};
use num_bigint::BigInt;
use num_traits::Zero;
use std::ops::Neg;
use crate::types::trade::Trade;
use num_traits::Num;

pub fn execute_limit_order(
    carried_state: &mut CarriedState,
    batch_config: &BatchConfig,
    limit_order: &LimitOrder,
    actual_collateral: BigInt,
    actual_synthetic: BigInt,
    actual_fee: BigInt,
) -> Result<(), PerpError> {
    let general_config: &GeneralConfig = &batch_config.general_config;

    if limit_order.position_id == general_config.fee_position_info.position_id {
        return Err(PerpError::InvalidPositionID);
    }

    // Check that asset_id_collateral is collateral.
    if limit_order.asset_id_collateral != general_config.collateral_asset_info.asset_id {
        return Err(PerpError::InvalidCollateralAssetID);
    }

    // 0 < limit_order.amount_collateral < AMOUNT_UPPER_BOUND.
    // 0 <= limit_order.amount_fee < AMOUNT_UPPER_BOUND.
    // Note that limit_order.amount_synthetic is checked by validate_order_and_update_fulfillment.
    if limit_order.amount_collateral < BigInt::from(POSITIVE_AMOUNT_LOWER_BOUND)
        || limit_order.amount_collateral >= BigInt::from(AMOUNT_UPPER_BOUND)
    {
        return Err(PerpError::OutOfRangePositiveAmount);
    }

    if limit_order.amount_fee < BigInt::zero()
        || limit_order.amount_fee > BigInt::from(AMOUNT_UPPER_BOUND - 1)
    {
        return Err(PerpError::OutOfRangePositiveAmount);
    }

    // actual_synthetic > 0. To prevent replay.
    // Note that actual_synthetic < AMOUNT_UPPER_BOUND is checked in
    // validate_order_and_update_fulfillment.
    if BigInt::from(POSITIVE_AMOUNT_LOWER_BOUND) > actual_synthetic {
        return Err(PerpError::OutOfRangePositiveAmount);
    }

    validate_limit_order_fairness(
        limit_order,
        &actual_collateral,
        &actual_synthetic,
        &actual_fee,
    )?;

    // TODO: use real hash
    let message_hash = {
        if limit_order.is_buying_synthetic {
            HashType::from_str_radix("15311d0f75e0f3d33022a87bd83f29f20b983605c3369e242c1a833d74e45794", 16).unwrap()
        } else {
            HashType::from_str_radix("26bce0eb499758b86ceba719a1c059fa7d7b693a7e651f4dfb4e177b3f0b6158", 16).unwrap()
        }
    };


    //let message_hash: HashType = limit_order_hash(limit_order);

    validate_order_and_update_fulfillment(
        &mut carried_state.orders_dict,
        &message_hash,
        &limit_order.base,
        &batch_config.min_expiration_timestamp,
        actual_synthetic.clone(),
        limit_order.amount_synthetic.clone(),
    )?;
    let collateral_delta: BigInt;
    let synthetic_delta: BigInt;

    if limit_order.is_buying_synthetic {
        collateral_delta = actual_collateral.neg() - actual_fee.clone();
        synthetic_delta = actual_synthetic;
    } else {
        collateral_delta = actual_collateral - actual_fee.clone();
        synthetic_delta = actual_synthetic.neg();
    }

    update_position_in_dict(
        &mut carried_state.positions_dict,
        &general_config.fee_position_info.position_id,
        &general_config.fee_position_info.public_key,
        actual_fee,
        &NO_SYNTHETIC_DELTA_ASSET_ID,
        BigInt::zero(),
        &carried_state.global_funding_indices,
        &carried_state.oracle_prices,
        general_config,
    )?;

    update_position_in_dict(
        &mut carried_state.positions_dict,
        &limit_order.position_id,
        &limit_order.base.public_key,
        collateral_delta,
        &limit_order.asset_id_synthetic,
        synthetic_delta,
        &carried_state.global_funding_indices,
        &carried_state.oracle_prices,
        general_config,
    )?;

    Ok(())
}

pub fn execute_trade(
    carried_state: &mut CarriedState,
    batch_config: &BatchConfig,
    trade: &Trade,
) -> Result<(), PerpError> {
    if trade.actual_collateral >= BigInt::from(AMOUNT_UPPER_BOUND) {
        return Err(PerpError::Error);
    }

    if trade.actual_a_fee >= BigInt::from(AMOUNT_UPPER_BOUND) {
        return Err(PerpError::Error);
    }

    if trade.actual_b_fee >= BigInt::from(AMOUNT_UPPER_BOUND) {
        return Err(PerpError::Error);
    }

    let buyer: &LimitOrder = &trade.party_a_order;
    let seller: &LimitOrder = &trade.party_b_order;

    // check party_a_order is a buyer while party_b_order is a seller
    if !buyer.is_buying_synthetic || seller.is_buying_synthetic {
        return Err(PerpError::Error);
    }

    execute_limit_order(
        carried_state,
        batch_config,
        buyer,
        trade.actual_collateral.clone(),
        trade.actual_synthetic.clone(),
        trade.actual_a_fee.clone(),
    )?;

    if buyer.asset_id_synthetic != seller.asset_id_synthetic {
        return Err(PerpError::Error);
    }

    execute_limit_order(
        carried_state,
        batch_config,
        seller,
        trade.actual_collateral.clone(),
        trade.actual_synthetic.clone(),
        trade.actual_b_fee.clone(),
    )
}