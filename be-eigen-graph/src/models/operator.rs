use super::ids::OperatorId;
use super::strategy::OperatorStrategyPosition;
use super::time::BlockTimestamp;
use super::token::TvlByToken;

#[derive(Clone, Debug, PartialEq)]
pub struct OperatorRiskRow {
    pub operator_id: OperatorId,
    pub avs_count: u32,
    pub strategy_count: u32,
    pub slashing_count: u32,
    pub last_slash_at: Option<BlockTimestamp>,
    pub hhi: f64,
    pub tvl_by_token: Vec<TvlByToken>,
    pub positions: Vec<OperatorStrategyPosition>,
}
