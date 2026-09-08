//! Typed closed request domain models for SSI REST endpoints.

use serde::Serialize;

/// SSI Market identifier accepted across market-data endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub enum Market {
    /// Ho Chi Minh Stock Exchange.
    #[serde(rename = "HOSE")]
    Hose,
    /// Hanoi Stock Exchange.
    #[serde(rename = "HNX")]
    Hnx,
    /// Unlisted Public Company Market.
    #[serde(rename = "UPCOM")]
    Upcom,
    /// Derivatives Market.
    #[serde(rename = "DER")]
    Der,
    /// Bond Market.
    #[serde(rename = "BOND")]
    Bond,
}

/// SSI Market identifier accepted for securities endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub enum SecuritiesMarket {
    /// Ho Chi Minh Stock Exchange.
    #[serde(rename = "HOSE")]
    Hose,
    /// Hanoi Stock Exchange.
    #[serde(rename = "HNX")]
    Hnx,
    /// Unlisted Public Company Market.
    #[serde(rename = "UPCOM")]
    Upcom,
    /// Derivatives Market.
    #[serde(rename = "DER")]
    Der,
}

/// SSI Exchange accepted for index endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub enum IndexExchange {
    /// Ho Chi Minh Stock Exchange.
    #[serde(rename = "HOSE")]
    Hose,
    /// Hanoi Stock Exchange.
    #[serde(rename = "HNX")]
    Hnx,
}

/// Query sorting direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub enum OrderDirection {
    /// Ascending order.
    #[serde(rename = "asc")]
    Asc,
    /// Descending order.
    #[serde(rename = "desc")]
    Desc,
}
