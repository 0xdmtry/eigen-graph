use crate::models::operator::OperatorRiskRow;
use crate::models::operators_aggr::{BarItem, GraphEdge, Outliers, TableRow};
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatesQuery {
    pub source: Option<String>,
    pub first: Option<i32>,
    pub skip: Option<i32>,
    pub top_n: Option<usize>,
    pub hhi_threshold: Option<f64>,
    pub min_tvl_atomic: Option<String>,
    pub operator_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatesResponse {
    pub meta: AggregatesMeta,
    pub table: Vec<TableRow>,
    pub bar: Vec<BarItem>,
    pub donut: serde_json::Value,
    pub graph: Vec<GraphEdge>,
    pub outliers: Outliers,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatesMeta {
    pub source: String,
    pub first: i32,
    pub skip: i32,
    pub count: usize,
}
