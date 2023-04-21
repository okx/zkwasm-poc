pub mod types;
pub mod executor;
pub mod mock;

use wasm_bindgen::prelude::*;
use crate::types::trade::Trade;
use crate::types::limit_order::OrderType;
use crate::types::order::OrderBase;
use crate::types::limit_order::LimitOrder;
use crate::executor::execute;
use num_bigint::BigInt;
use crate::mock::make_state;
use crate::types::config::BatchConfig;
use crate::types::state::CarriedState;

#[wasm_bindgen]
pub fn zkmain() {
    let mut state = make_state();
    execute_trade(&mut state).unwrap();
}

pub fn execute_trade(state: &mut CarriedState) -> Result<(), types::perp_error::PerpError> {
    let trade = generate_trade_tx();
    let tx = types::transactions::Transaction::Trade(Box::new(trade));
    let config = BatchConfig::test_config();
    match tx {
        types::transactions::Transaction::Trade(trade) => {
            execute::execute_trade(state, &config, &trade)?;
        }
    }
    Ok(())
}

fn generate_trade_tx() -> Trade {
    let mut sig_a: [u8; 64] = [0; 64];
    let mut sig_b: [u8; 64] = [0; 64];
    hex::decode_to_slice("2ff1c4706c8eec9957357f188ca3b3cc4cac43eaccb4f1c17400ed0be3151706d97db8f7b52c9bb1bbcf0a5c8f40151748778f23af27e4afbe1e0234b8fdb201", &mut sig_a).unwrap();
    hex::decode_to_slice("b04e7cc7980a8ff3e1d4768103f89543c0dc1690c39b058146f8a36c03dc19adee7d810bc3d619a80b59437d93b49205c0f08d4ccfe1ca1a156053815caaeb05", &mut sig_b).unwrap();

    let mut pub_a: [u8; 32] = [0; 32];
    let mut pub_b: [u8; 32] = [0; 32];
    hex::decode_to_slice("df84035a8f7be2bc8d8a7f2d4a0be6c1e774f0a4c16aa0b112e64eb62c09698a", &mut pub_a).unwrap();
    hex::decode_to_slice("f5705bf1a2e8688ba804744fecc915371896aa7c39521966a9a61945dcda5219", &mut pub_b).unwrap();
    Trade{
        party_a_order: LimitOrder{
            base: OrderBase{
                nonce: 1,
                public_key: pub_a,
                expiration_timestamp: 3608164305,
                signature: sig_a,
            },
            amount_synthetic: BigInt::from(100000000),
            amount_collateral: BigInt::from(25000000000i64),
            amount_fee: BigInt::from(25000000),
            asset_id_synthetic: 0,
            asset_id_collateral: 7,
            position_id: 10000,
            is_buying_synthetic: true,
            order_type: OrderType::default(),
        },
        party_b_order: LimitOrder{
            base: OrderBase{
                nonce: 1,
                public_key: pub_b,
                expiration_timestamp: 3407305306,
                signature: sig_b,
            },
            amount_synthetic: BigInt::from(200000000),
            amount_collateral: BigInt::from(25000000000i64),
            amount_fee: BigInt::from(25000000),
            asset_id_synthetic: 0,
            asset_id_collateral: 7,
            position_id: 10001,
            is_buying_synthetic: false,
            order_type: OrderType::default(),
        },
        actual_collateral: BigInt::from(25000000000i64),
        actual_synthetic: BigInt::from(100000000),
        actual_a_fee: BigInt::from(25000000),
        actual_b_fee: BigInt::from(12500000),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_hash() {
    //     let message_hash_buy = hex::decode("15311d0f75e0f3d33022a87bd83f29f20b983605c3369e242c1a833d74e45794").unwrap();
    //     let message_hash = H256::from_slice(&message_hash_buy);
    //     let order_id = extract_order_id(&message_hash).unwrap();
    //     println!("{}", order_id);
    //
    //     let message_hash_bigint = h256_to_bigint(&message_hash);
    //     let message_hash_bigint = bn254_hash_size_to_pedersen_hash_size(&message_hash_bigint);
    //     println!("{}", message_hash_bigint);
    //     let a = BigInt::from_str_radix("15311d0f75e0f3d33022a87bd83f29f20b983605c3369e242c1a833d74e45794", 16).unwrap();
    //     println!("{}", a);
    //
    //     println!("{},{}", message_hash_bigint.to_string().len(), a.to_string().len());
    //
    // }

    #[test]
    fn test_trade() {
        let mut state = make_state();
        assert_eq!(BigInt::from(10000000000i64), state.positions_dict.get_position(&10000).unwrap().collateral_balance);
        assert_eq!(BigInt::from(10000000000i64), state.positions_dict.get_position(&10001).unwrap().collateral_balance);
        assert_eq!(0usize, state.positions_dict.get_position(&10000).unwrap().assets.len());
        assert_eq!(0usize, state.positions_dict.get_position(&10001).unwrap().assets.len());
        // println!("init order_a:{:?}", state.positions_dict.get_position(&10000).unwrap());
        // println!("init order_b:{:?}", state.positions_dict.get_position(&10001).unwrap());
        execute_trade(&mut state).unwrap();
        assert_eq!(BigInt::from(-15025000000i64), state.positions_dict.get_position(&10000).unwrap().collateral_balance);
        assert_eq!(BigInt::from(34987500000i64), state.positions_dict.get_position(&10001).unwrap().collateral_balance);
        // println!("after order_a:{:?}", state.positions_dict.get_position(&10000).unwrap());
        // println!("after order_b:{:?}", state.positions_dict.get_position(&10001).unwrap());
        assert_eq!(1usize, state.positions_dict.get_position(&10000).unwrap().assets.len());
        assert_eq!(BigInt::from(100000000i64), state.positions_dict.get_position(&10000).unwrap().assets[0].balance);
        assert_eq!(1usize, state.positions_dict.get_position(&10001).unwrap().assets.len());
        assert_eq!(BigInt::from(-100000000i64), state.positions_dict.get_position(&10001).unwrap().assets[0].balance);
    }
}
