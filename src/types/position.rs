use crate::types::defined_types::{IndexType, PositionIdType};
use crate::types::packed_public_key::PublicKeyType;
use crate::types::constants::{BALANCE_LOWER_BOUND, BALANCE_UPPER_BOUND};
use crate::types::{defined_types::AssetIdType, perp_error::PerpError};
use num_bigint::BigInt;
use num_traits::Zero;
use crate::types::defined_types::TimeType;

#[derive(Debug, Default, Clone)]
pub struct PositionAsset {
    pub balance: BigInt,
    pub asset_id: AssetIdType,
    // A snapshot of the funding index at the last time that funding was applied (fxp 32.32).
    pub cached_funding_index: IndexType,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub public_key: PublicKeyType,
    pub collateral_balance: BigInt,
    pub assets: Vec<PositionAsset>,
    pub funding_timestamp: TimeType,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            public_key: PublicKeyType::default(),
            collateral_balance: BigInt::default(),
            assets: Vec::new(),
            funding_timestamp: 0,
        }
    }
}

const STORE_LEN: usize = 64;

#[derive(Debug)]
pub struct PositionDictAccess {
    store: [Position; STORE_LEN],
}


impl PositionDictAccess {
    pub fn new() -> Self {
        Self {
            store: core::array::from_fn(|_i| Position::default()),
        }
    }

    pub fn get_position(&self, position_id: &PositionIdType) -> Result<Position, PerpError> {
       if let Some(v) = self.store.get(*position_id as usize % STORE_LEN) {
           Ok(v.clone())
       } else {
           Ok(Position::default())
       }

    }

    pub fn update(
        &mut self,
        position_id: &PositionIdType,
        new_value: &Position,
    ) -> Result<BigInt, PerpError> {
        let old = self.get_position(position_id)?.collateral_balance;
        self.store[*position_id as usize % STORE_LEN] = new_value.clone();

        Ok(old)
    }
}

pub fn position_new(
    public_key: &PublicKeyType,
    collateral_balance: BigInt,
    assets: &Vec<PositionAsset>,
    funding_timestamp: &TimeType,
) -> Position {
    return Position {
        public_key: public_key.clone(),
        collateral_balance,
        assets: assets.clone(),
        funding_timestamp: *funding_timestamp,
    };
}

pub fn position_add_collateral(
    position: &Position,
    delta: &BigInt,
    public_key: &PublicKeyType,
) -> Result<Position, PerpError> {
    let final_position = create_maybe_empty_position(
        public_key,
        position.collateral_balance.clone() + delta,
        &position.assets,
        &position.funding_timestamp,
    );

    check_valid_balance(final_position.collateral_balance.clone())?;
    Ok(final_position)
}

// Gets the balance of a specific asset in the position.
pub fn position_get_asset_balance(position: &Position, asset_id: &AssetIdType) -> BigInt {
    for ass in position.assets.iter() {
        if ass.asset_id == asset_id.clone() {
            return ass.balance.clone();
        }
    }
    return BigInt::zero();
}

// Checks that value is in the range [BALANCE_LOWER_BOUND, BALANCE_UPPER_BOUND)
pub fn check_valid_balance(balance: BigInt) -> Result<(), PerpError> {
    if BigInt::zero() <= balance.clone() - BALANCE_LOWER_BOUND
        && balance.clone() - BALANCE_LOWER_BOUND
        <= BigInt::from(BALANCE_UPPER_BOUND - BALANCE_LOWER_BOUND - 1)
    {
        return Ok(());
    }
    Err(PerpError::OutOfRangeBalance)
}

// Creates a position with given arguments.
// If the position is empty (collateral_balance == n_assets == 0) the public_key is ignored
// and an empty position is returned.
// The public_key must be non-zero.
pub fn create_maybe_empty_position(
    public_key: &PublicKeyType,
    collateral_balance: BigInt,
    assets: &Vec<PositionAsset>,
    funding_timestamp: &TimeType,
) -> Position {
    // TODO public key
    // if public_key == 0 {
    //     // If public_key == 0 add an unsatisfiable requirement.
    //     // public_key = 1
    // }

    let empty_assets: Vec<PositionAsset> = Vec::new();

    if collateral_balance == BigInt::zero() && assets.len() == 0 {
        // TODO
        return position_new(
            &PublicKeyType::default(),
            BigInt::zero(),
            &empty_assets,
            &0,
        );
    }

    return position_new(public_key, collateral_balance, &assets, funding_timestamp);
}

// Checks that the public key supplied in a request to change the position is valid.
// The public key is valid if it matches the position's public key or if the position is empty
// (public key is zero).
// The supplied key may not be zero.
// Return 0 if the check passed, otherwise returns an error code that describes the failure.
pub fn check_request_public_key(
    position_public_key: &PublicKeyType,
    request_public_key: &PublicKeyType,
) -> Result<(), PerpError> {
    if request_public_key.eq(&PublicKeyType::default()) {
        // Invalid request_public_key.
        return Err(PerpError::InvalidPublicKey);
    }
    if position_public_key.eq(&PublicKeyType::default()) {
        // Initial position is empty.
        return Ok(());
    }
    if position_public_key.eq(request_public_key) {
        // Matching keys.
        return Ok(());
    }
    // Mismatching keys.
    Err(PerpError::InvalidPublicKey)
}