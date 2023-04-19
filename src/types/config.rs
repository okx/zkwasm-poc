use crate::types::defined_types::PositionIdType;
use crate::types::defined_types::{AssetIdType};
use crate::types::packed_public_key::PublicKeyType;
use num_bigint::BigInt;
use time::OffsetDateTime;
use std::time::Duration;


// Information about the unique collateral asset of the system.
#[derive(Debug, Clone, Default)]
pub struct CollateralAssetInfo {
    pub asset_id: AssetIdType,
    // Resolution: Each unit of balance in the oracle is worth this much units in our system.
    // pub resolution: BigInt,
}

// Information about the unique fee position of the system. All fees are paid to it.
#[derive(Debug, Clone, Default)]
pub struct FeePositionInfo {
    pub position_id: PositionIdType,
    pub public_key: PublicKeyType,
}

// Information about a synthetic asset in the system.
#[derive(Debug, Clone)]
pub struct SyntheticAssetInfo {
    // Asset id.
    pub asset_id: AssetIdType,
    // Resolution: Each unit of balance in the oracle is worth this much units in our system.
    // pub resolution: BigInt,
    // 32.32 fixed point number indicating the risk factor of the asset. This is used in deciding if
    // a position is well leveraged.
    pub risk_factor: BigInt,
    // A list of IDs associated with the asset, on which the oracle price providers sign.
    pub oracle_price_signed_asset_ids: Vec<AssetIdType>,
    // The minimum amounts of signatures required to sign on a price.
    pub oracle_price_quorum: u64,
    // A list of oracle signer public keys.
    pub oracle_price_signers: Vec<PublicKeyType>,
}

// Configuration for timestamp validation.
#[derive(Debug, Clone, Default)]
pub struct TimestampValidationConfig {
    // we don't need a negative value, so we just using
    // std::time::Duration but not time::Duration.
    pub price_validity_period: Duration,
    pub funding_validity_period: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct GeneralConfig {
    // 32.32 fixed point number, indicating the maximum rate of change of a normalized funding index.
    // Units are (1) / (time * price)
    // pub max_funding_rate: BigInt,
    // See CollateralAssetInfo.
    pub collateral_asset_info: CollateralAssetInfo,
    // See FeePositionInfo.
    pub fee_position_info: FeePositionInfo,
    // Information about the synthetic assets in the system. See SyntheticAssetInfo.
    pub synthetic_assets_info: Vec<SyntheticAssetInfo>,
    // Height of the merkle tree in which positions are kept.
    pub positions_tree_height: u64,
    // Height of the merkle tree in which orders are kept.
    pub orders_tree_height: u64,
    // See TimestampValidationConfig.
    pub timestamp_validation_config: TimestampValidationConfig,
}

impl GeneralConfig {
    pub fn test_config() -> Self {
        let fee_pk = hex::decode("df84035a8f7be2bc8d8a7f2d4a0be6c1e774f0a4c16aa0b112e64eb62c09698a").unwrap().try_into().unwrap();
        Self {
            // max_funding_rate: BigInt::from(1120),
            collateral_asset_info: CollateralAssetInfo{
                asset_id: 7,
                // resolution: BigInt::from(1000000),
            },
            fee_position_info: FeePositionInfo{
                position_id: 11111,
                public_key: fee_pk,
            },
            synthetic_assets_info: vec![
                SyntheticAssetInfo{
                    asset_id: 0,
                    // resolution: BigInt::from(10000000000i64),
                    risk_factor: BigInt::from(214748365),
                    oracle_price_signed_asset_ids: vec![],
                    oracle_price_quorum: 1,
                    oracle_price_signers: vec![],
                },
                SyntheticAssetInfo{
                    asset_id: 1,
                    // resolution: BigInt::from(100000000),
                    risk_factor: BigInt::from(322122548),
                    oracle_price_signed_asset_ids: vec![],
                    oracle_price_quorum: 1,
                    oracle_price_signers: vec![],
                },
                SyntheticAssetInfo{
                    asset_id: 2,
                    // resolution: BigInt::from(10000000),
                    risk_factor: BigInt::from(429496730),
                    oracle_price_signed_asset_ids: vec![],
                    oracle_price_quorum: 1,
                    oracle_price_signers: vec![],
                },
            ],
            positions_tree_height: 64,
            orders_tree_height: 64,
            timestamp_validation_config: TimestampValidationConfig {
                price_validity_period: Duration::from_secs(31536000),
                funding_validity_period: Duration::from_secs(604800),
            },
        }
    }
}


#[derive(Debug, Clone)]
pub struct BatchConfig {
    pub general_config: GeneralConfig,
    // pub signed_min_oracle_prices: Vec<OraclePrice>,
    // pub signed_max_oracle_prices: Vec<OraclePrice>,
    pub min_expiration_timestamp: OffsetDateTime,
}

impl BatchConfig {
    pub fn test_config() -> Self {
        Self {
            general_config: GeneralConfig::test_config(),
            // signed_min_oracle_prices: vec![],
            // signed_max_oracle_prices: vec![],
            min_expiration_timestamp: OffsetDateTime::from_unix_timestamp(0).unwrap(),
        }
    }
}