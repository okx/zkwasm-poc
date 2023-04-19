use std::fmt::{Display, Formatter};

#[repr(i32)]
#[derive(Debug, PartialEq)]
pub enum PerpError {
    IllegalPositionTransitionEnlargingSyntheticHoldings = 1,
    IllegalPositionTransitionNoRiskReducedValue = 2,
    IllegalPositionTransitionReducingTotalValueRiskRatio = 3,
    InvalidAssetOraclePrice = 4,
    InvalidCollateralAssetID = 5,
    InvalidFulfillmentAssetsRatio = 6,
    InvalidFulfillmentFeeRatio = 7,
    InvalidFulfillmentInfo = 8,
    InvalidFundingTickTimestamp = 9,
    InvalidPublicKey = 10,
    InvalidSignature = 11,
    MissingGlobalFundingIndex = 12,
    MissingOraclePrice = 13,
    MissingSyntheticAssetID = 14,
    OutOfRangeAmount = 15,
    OutOfRangeBalance = 16,
    OutOfRangeFundingIndex = 17,
    OutOfRangePositiveAmount = 18,
    OutOfRangeTotalRisk = 19,
    OutOfRangeTotalValue = 20,
    SamePositionID = 21,
    TooManySyntheticAssetsInPosition = 22,
    TooManySyntheticAssetsInSystem = 23,
    UndeleveragablePosition = 24,
    UnfairDeleverage = 25,
    UnliquidatablePosition = 26,
    UnsortedOraclePrices = 27,

    // Custom defined
    Error = 28,
    InvalidCollateralBalance = 29,
    ValidateFundingIndicesFailed = 30,
    ValidateAssetsConfigFailed = 31,
    UnknownTxType = 32,
    OutOfRangeOraclePrice = 33,
    OutOfRangeExteranlOraclePrice = 34,
    InvalidOraclePriceTickTimestamp = 35,
    OutOfRangeOraclePriceTickTimestamp = 36,
    InvalidOracleMedianPrice = 37,
    InvalidTimeStamp = 38,
    InvalidPositionID = 39,
}

impl Display for PerpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO: make display more readable
        f.write_fmt(format_args!("PerpError: {:?}", self))
    }
}
