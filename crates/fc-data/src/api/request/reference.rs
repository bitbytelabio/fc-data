use serde::Serialize;

use super::{IndexExchange, PageQuery, SecuritiesMarket, ValidationError, validation as validate};

pub(super) const SECURITIES_PATH: &str = "api/v2/Market/Securities";
pub(super) const SECURITIES_DETAILS_PATH: &str = "api/v2/Market/SecuritiesDetails";
pub(super) const INDEX_COMPONENTS_PATH: &str = "api/v2/Market/IndexComponents";
pub(super) const INDEX_LIST_PATH: &str = "api/v2/Market/IndexList";

/// Securities list request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<SecuritiesMarket>,
    #[serde(flatten)]
    page: PageQuery,
}

impl SecuritiesQuery {
    /// Parses an optional market and endpoint-specific pagination.
    pub fn new(market: Option<SecuritiesMarket>, page: PageQuery) -> Result<Self, ValidationError> {
        validate::securities_page_size(page.page_size)?;
        Ok(Self { market, page })
    }
}

/// Securities details request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesDetailsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<SecuritiesMarket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(flatten)]
    page: PageQuery,
}

impl SecuritiesDetailsQuery {
    /// Parses optional market and symbol filters with endpoint-specific pagination.
    pub fn new(
        market: Option<SecuritiesMarket>,
        symbol: Option<String>,
        page: PageQuery,
    ) -> Result<Self, ValidationError> {
        validate::optional(symbol.as_deref(), "symbol")?;
        validate::securities_page_size(page.page_size)?;
        Ok(Self {
            market,
            symbol,
            page,
        })
    }
}

/// Index components request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponentsQuery {
    index_code: String,
    #[serde(flatten)]
    page: PageQuery,
}

impl IndexComponentsQuery {
    /// Parses a required index code with validated pagination.
    pub fn new(index_code: String, page: PageQuery) -> Result<Self, ValidationError> {
        validate::required(&index_code, "indexCode")?;
        Ok(Self { index_code, page })
    }
}

/// Index list request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    exchange: Option<IndexExchange>,
    #[serde(flatten)]
    page: PageQuery,
}

impl IndexListQuery {
    /// Creates an index list query for an optional HOSE or HNX exchange with validated pagination.
    #[must_use]
    pub const fn new(exchange: Option<IndexExchange>, page: PageQuery) -> Self {
        Self { exchange, page }
    }
}
