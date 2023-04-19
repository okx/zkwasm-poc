use crate::types::packed_public_key::PublicKeyType;
use time::OffsetDateTime;
use primitive_types::H256;
use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};
use crate::types::perp_error::PerpError;
use crate::types::hash::{h256_to_bigint, bn254_hash_size_to_pedersen_hash_size};
use crate::types::constants::*;
use crate::types::exchange::AMOUNT_UPPER_BOUND;
pub type PositionIdType = u64;
pub type OrderIdType = u64;
pub type HashType = H256;
pub type PrivateKeyType = String;
pub type IndexType = i128;

#[derive(Debug, Clone, PartialEq)]
pub struct OrderBase {
    pub nonce: u64,
    pub public_key: PublicKeyType,
    pub expiration_timestamp: OffsetDateTime,
    pub signature: [u8; 64],
}

impl Default for OrderBase {
    fn default() -> Self {
        Self {
            nonce: 0,
            public_key: Default::default(),
            expiration_timestamp: OffsetDateTime::from_unix_timestamp(0).unwrap(),
            signature: [0; 64],
        }
    }
}

const STORE_LEN: usize = 64;

#[derive(Debug)]
pub struct OrderDictAccess {
    store: [BigInt; STORE_LEN],
}

impl OrderDictAccess {
    pub fn new() -> Self {
        Self {
            store: core::array::from_fn(|_i| BigInt::default()),
        }
    }

    pub fn get_filled_amount(&mut self, order_id: &OrderIdType) -> Result<BigInt, PerpError> {
        if let Some(v) = self.store.get(*order_id as usize % STORE_LEN) {
            Ok(v.clone())
        } else {
            Ok(BigInt::default())
        }
    }

    pub fn update(
        &mut self,
        order_id: &OrderIdType,
        new_value: &BigInt,
    ) -> Result<BigInt, PerpError> {
        let old_amount = self.get_filled_amount(order_id)?;
        self.store[*order_id as usize % STORE_LEN] = new_value.clone();

        Ok(old_amount)
    }
}

pub(crate) fn extract_order_id(message_hash: &HashType) -> Result<OrderIdType, PerpError> {
    // The 251-bit message_hash can be viewed as a packing of three fields:
    // +----------------+--------------------+----------------LSB-+
    // | order_id (64b) | middle_field (59b) | right_field (128b) |
    // +----------------+--------------------+--------------------+
    // .

    let message_hash_bigint = h256_to_bigint(&message_hash);
    let message_hash_bigint = bn254_hash_size_to_pedersen_hash_size(&message_hash_bigint);

    let order_id_shift = SIGNED_MESSAGE_BOUND.clone() / ORDER_ID_UPPER_BOUND.clone();
    let middle_field_bound = order_id_shift.clone() / RANGE_CHECK_BOUND.clone();

    let order_id = message_hash_bigint.clone() / order_id_shift.clone();
    let right_field = message_hash_bigint.clone() & (RANGE_CHECK_BOUND.clone() - BigInt::from(1));
    let middle_field = (message_hash_bigint.clone() / RANGE_CHECK_BOUND.clone())
        & (middle_field_bound.clone() - BigInt::from(1));

    if middle_field_bound.clone() & (middle_field_bound.clone() - 1) != BigInt::zero() {
        panic!("MIDDLE_FIELD_BOUND should be a power of 2")
    }

    let check_message_hash = order_id.clone() * order_id_shift.clone()
        + middle_field.clone() * RANGE_CHECK_BOUND.clone()
        + right_field.clone();
    if message_hash_bigint != check_message_hash {
        panic!("message_hash not match")
    }

    if right_field < BigInt::zero() || right_field >= RANGE_CHECK_BOUND.clone() {
        panic!("right_field not match")
    }
    if middle_field < BigInt::zero() || middle_field >= middle_field_bound {
        panic!("middle_field not match")
    }
    Ok(order_id.to_u64().unwrap())
}

fn update_order_fulfillment(
    order_dict: &mut OrderDictAccess,
    message_hash: &H256,
    update_amount: BigInt,
    full_amount: BigInt,
) -> Result<(), PerpError> {
    let order_id = extract_order_id(message_hash)?;

    let fulfilled_amount = order_dict.get_filled_amount(&order_id)?;
    let remaining_capacity = full_amount.clone() - fulfilled_amount.clone();
    if update_amount < BigInt::zero() || update_amount > remaining_capacity {
        return Err(PerpError::OutOfRangeAmount);
    }
    if full_amount >= AMOUNT_UPPER_BOUND.clone() {
        return Err(PerpError::OutOfRangeAmount);
    }
    let _ = order_dict.update(&order_id, &(fulfilled_amount + update_amount));
    Ok(())
}

pub fn validate_order_and_update_fulfillment(
    order_dict: &mut OrderDictAccess,
    message_hash: &H256,
    _order: &OrderBase,
    _min_expiration_timestamp: &OffsetDateTime,
    update_amount: BigInt,
    full_amount: BigInt,
) -> Result<(), PerpError> {
    // TODO verify signature

    // TODO verify timestamp

    // TODO verify nonce

    update_order_fulfillment(order_dict, message_hash, update_amount, full_amount)
}
