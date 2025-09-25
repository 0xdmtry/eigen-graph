use crate::models::ids::TokenId;
use crate::models::operator::OperatorRiskRow;
use crate::models::operators_snapshot::{OperatorsSnapshotData, OperatorsSnapshotVars};

pub trait OperatorsSnapshotFetcher {
    fn fetch(
        &self,
        vars: OperatorsSnapshotVars,
    ) -> futures::future::BoxFuture<'_, Result<OperatorsSnapshotData, anyhow::Error>>;
}

#[derive(Clone, Debug)]
pub struct OperatorRiskParams {
    pub first: i32,
    pub skip: i32,
    pub has_slashing: i32,
    pub min_avs: Option<u32>,
    pub concentration_max: Option<f64>,
    pub token_filter: Option<TokenId>,
    pub min_tvl_atomic: Option<String>,
    pub order: OperatorRiskOrder,
}

#[derive(Clone, Debug)]
pub struct OperatorRiskPage {
    pub rows: Vec<OperatorRiskRow>,
    pub next_skip: i32,
    pub has_more: bool,
}

#[derive(Clone, Debug)]
pub struct OperatorRiskOrder {
    pub field: OperatorRiskOrderField,
    pub direction: SortDir,
}

#[derive(Clone, Debug)]
pub enum OperatorRiskOrderField {
    Tvl,
    Hhi,
    AvsCount,
    StrategyCount,
    SlashingCount,
    LastSlashAt,
    LastUpdateBlockTs,
}

#[derive(Clone, Copy, Debug)]
pub enum SortDir {
    Asc,
    Desc,
}
