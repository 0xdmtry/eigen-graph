use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniformPage {
    pub operators: Vec<UniformOperator>,
    pub page_meta: PageMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMeta {
    pub first: i32,
    pub skip: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniformOperator {
    pub operator_id: String,
    pub avs_count: i32,
    pub strategy_count: i32,
    pub slashing_count: i32,
    pub last_slash_at: Option<i64>,
    pub last_update_block_ts: i64,
    pub positions: Vec<UniformPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniformPosition {
    pub strategy_id: String,
    pub token_id: String,
    pub token_symbol: String,
    pub token_decimals: i32,
    pub total_shares: String,
    pub exchange_rate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorParams {
    pub top_n: usize,
    pub hhi_threshold: f64,
    pub recent_window_s: i64,
    pub min_tvl_atomic: Option<String>,
    pub focus_operator_id: Option<String>,
}

impl Default for AggregatorParams {
    fn default() -> Self {
        Self {
            top_n: 10,
            hhi_threshold: 0.2,
            recent_window_s: 7 * 24 * 3600,
            min_tvl_atomic: None,
            focus_operator_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorAggregate {
    pub operator_id: String,
    pub avs_count: i32,
    pub strategy_count: i32,
    pub slashing_count: i32,
    pub last_slash_at: Option<i64>,
    pub last_update_block_ts: i64,

    pub tvl_total_atomic: String,
    pub nonzero_strategy_count: i32,

    pub strategy_breakdown: Vec<StrategySlice>,
    pub top_strategy_share: f64,
    pub hhi_strategy: f64,
    pub zero_share_flag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategySlice {
    pub strategy_id: String,
    pub tvl_atomic: String,
    pub share: f64, // 0..1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub operator_id: String,
    pub avs_count: i32,
    pub strategy_count: i32,
    pub slashing_count: i32,
    pub last_slash_at: Option<i64>,
    pub last_update_block_ts: i64,
    pub tvl_total_atomic: String,
    pub hhi_strategy: f64,
    pub nonzero_strategy_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarItem {
    pub operator_id: String,
    pub tvl_total_atomic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Donut {
    pub operator_id: String,
    pub slices: Vec<StrategySlice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub operator_id: String,
    pub strategy_id: String,
    pub weight_atomic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outliers {
    pub high_concentration: Vec<String>,
    pub zero_share: Vec<String>,
    pub recent_slashes: Vec<String>,
}
