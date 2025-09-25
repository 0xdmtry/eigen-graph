use crate::models::operator::OperatorRiskRow;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotQuery {
    pub(crate) first: Option<i32>,
    pub(crate) skip: Option<i32>,
    pub(crate) has_slashing: Option<i32>,
    pub(crate) order_by: Option<String>,
    pub(crate) order_direction: Option<String>,
}

#[derive(Serialize)]
pub struct OperatorRiskItemView {
    pub operator_id: String,
    pub avs_count: u32,
    pub strategy_count: u32,
    pub slashing_count: u32,
    pub last_slash_at: Option<i64>,
    pub hhi: f64,
    pub tvl: Vec<TvlView>,
}

#[derive(Serialize)]
pub struct TvlView {
    pub token: String,
    pub symbol: String,
    pub amount_atomic: String,
}

impl From<OperatorRiskRow> for OperatorRiskItemView {
    fn from(r: OperatorRiskRow) -> Self {
        Self {
            operator_id: r.operator_id.0,
            avs_count: r.avs_count,
            strategy_count: r.strategy_count,
            slashing_count: r.slashing_count,
            last_slash_at: r.last_slash_at.map(|t| t.0),
            hhi: r.hhi,
            tvl: r
                .tvl_by_token
                .into_iter()
                .map(|t| TvlView {
                    token: t.token.id.0,
                    symbol: t.token.symbol,
                    amount_atomic: t.amount_atomic.0,
                })
                .collect(),
        }
    }
}
