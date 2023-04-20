use num_bigint::BigInt;
use crate::types;
use crate::types::packed_public_key::PublicKeyType;
use crate::types::position::Position;
use crate::types::objects::OraclePrice;
use crate::types::objects::{FundingIndex,FundingIndicesInfo, OraclePrices};
use crate::types::order::OrderDictAccess;
use crate::types::position::PositionDictAccess;
use crate::types::defined_types::PositionIdType;
use crate::types::state::CarriedState;

pub(crate) fn make_state() -> CarriedState {
    let btc_asset_id = 0;
    let eth_asset_id = 1;

    let funding_indices = vec![
        FundingIndex {
            asset_id: btc_asset_id.clone(),
            funding_index: 1,
        },
        FundingIndex {
            asset_id: eth_asset_id.clone(),
            funding_index: 100,
        },
    ];

    let global_funding_indices = FundingIndicesInfo {
        funding_indices,
        funding_timestamp: 0,
    };

    let data = vec![
        OraclePrice {
            asset_id: btc_asset_id,
            price: BigInt::from(1073741824000i64),
        }, // BTC
        OraclePrice {
            asset_id: eth_asset_id,
            price: BigInt::from(1009900000000000i64),
        },
    ];
    let oracle_prices = OraclePrices { data };

    let mut positions_dict = PositionDictAccess::new();

    let party_a_position_id: PositionIdType = 10000;

    let party_a_public_key: PublicKeyType
        = hex::decode("df84035a8f7be2bc8d8a7f2d4a0be6c1e774f0a4c16aa0b112e64eb62c09698a")
        .unwrap().try_into().unwrap();


    let party_a_position = Position {
        public_key: party_a_public_key,
        collateral_balance: BigInt::from(10000000000i64), // 1w
        assets: vec![],
        funding_timestamp: 0,
    };

    let _ = positions_dict.update(&party_a_position_id, &party_a_position);

    let party_b_position_id: PositionIdType = 10001;
    let party_b_public_key : PublicKeyType
        = hex::decode("f5705bf1a2e8688ba804744fecc915371896aa7c39521966a9a61945dcda5219")
        .unwrap().try_into().unwrap();

    let party_b_position = Position {
        public_key: party_b_public_key,
        collateral_balance: BigInt::from(10000000000i64), // 1w
        assets: vec![],
        funding_timestamp: 0,
    };

    let _ = positions_dict.update(&party_b_position_id, &party_b_position);

    CarriedState{
        positions_dict,
        orders_dict: OrderDictAccess::new(),
        global_funding_indices,
        oracle_prices,
        system_time: 0,
    }
}