use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorSnapshotVars {
    pub first: i32,
    pub skip: i32,
    pub order_by: OperatorOrderBy,
    pub order_direction: OrderDirection,
    pub has_slashing: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OperatorOrderBy {
    Id,
    AvsCount,
    StrategyCount,
    SlashingCount,
    LastUpdateBlockTimestamp,
}

#[derive(Serialize)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
