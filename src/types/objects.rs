use crate::types::defined_types::{AssetIdType, IndexType};
use num_bigint::BigInt;
use crate::types::defined_types::TimeType;

#[derive(Debug, Clone, PartialEq)]
pub struct FundingIndex {
    pub asset_id: AssetIdType,
    // funding_index in fxp 32.32 format.
    pub funding_index: IndexType,
}

// Funding indices and their timestamp.
#[derive(Debug, Clone, PartialEq)]
pub struct FundingIndicesInfo {
    pub funding_indices: Vec<FundingIndex>,
    pub funding_timestamp: TimeType, // TODO: rename this field
}

// Represents a single asset's Oracle Price in internal representation (Refer to the documentation of
// AssetOraclePrice for the definition of internal representation).
#[derive(Debug, Clone)]
pub struct OraclePrice {
    pub asset_id: AssetIdType,
    // # 32.32 fixed point.
    pub price: BigInt,
}

#[derive(Debug, Clone)]
pub struct OraclePrices {
    pub data: Vec<OraclePrice>,
}
