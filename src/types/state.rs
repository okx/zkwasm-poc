use crate::types::defined_types::HashType;
use crate::types::config::GeneralConfig;

use time::OffsetDateTime;

use crate::types::order::OrderDictAccess;
use crate::types::objects::{FundingIndicesInfo, OraclePrices};
use crate::types::position::PositionDictAccess;

// State carried through batch execution. Keeps the current pointer of all dicts.
#[derive(Debug)]
pub struct CarriedState {
    pub positions_dict: PositionDictAccess,
    pub orders_dict: OrderDictAccess,
    pub global_funding_indices: FundingIndicesInfo,
    pub oracle_prices: OraclePrices,
    // TODO: prev_execute_time maybe better name
    pub system_time: OffsetDateTime,
}

// State stored on the blockchain.
pub struct SharedState {
    pub positions_root: HashType,
    pub positions_tree_height: u64,
    pub orders_root: HashType,
    pub orders_tree_height: u64,
    pub global_funding_indices: FundingIndicesInfo,
    pub oracle_prices: OraclePrices,
    pub system_time: OffsetDateTime,
}

// Applies the updates from the squashed carried state on the initial shared state.
// Arguments:
// pedersen_ptr - Pointer to the hash builtin.
// shared_state - The initial shared state
// squashed_carried_state - The squashed carried state representing the updated state.
// general_config - The general config (It doesn't change throughout the program so it's both initial
//   and updated).
//
// Returns:
// pedersen_ptr - Pointer to the hash builtin.
// shared_state - The shared state that corresponds to the updated state.
pub fn shared_state_apply_state_updates(
    _shared_state: &SharedState,
    carried_state: &CarriedState,
    general_config: &GeneralConfig,
) -> SharedState {
    // Hash position updates.
    // let (hashed_position_updates_ptr) = hash_position_updates(squashed_carried_state.positions_dict);

    // Merkle update positions dict.
    let new_positions_root: HashType = HashType::default();
    // TODO commit position dict

    // Merkle update orders dict.
    let new_orders_root: HashType = HashType::default();
    // TODO commit order dict

    // Return SharedState.
    return SharedState {
        positions_root: new_positions_root,
        positions_tree_height: general_config.positions_tree_height,
        orders_root: new_orders_root,
        orders_tree_height: general_config.orders_tree_height,
        global_funding_indices: carried_state.global_funding_indices.clone(),
        oracle_prices: carried_state.oracle_prices.clone(),
        system_time: carried_state.system_time,
    };
}
