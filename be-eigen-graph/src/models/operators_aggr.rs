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
