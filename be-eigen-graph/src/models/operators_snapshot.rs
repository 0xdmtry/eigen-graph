use serde::{Deserialize, Serialize};

/* --- OperatorsSnapshot: variables --- */

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatorsSnapshotVars {
    pub first: i32,
    pub skip: i32,
    pub order_by: OperatorOrderBy,
    pub order_direction: OrderDirection,
    pub has_slashing: i32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OperatorOrderBy {
    Id,
    AvsCount,
    StrategyCount,
    SlashingCount,
    LastUpdateBlockTimestamp,
}

#[derive(Serialize, Debug)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

/* --- OperatorsSnapshot: response --- */

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorsSnapshotData {
    pub operators: Vec<OperatorDto>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatorDto {
    pub id: String,
    pub avs_count: i32,
    pub strategy_count: i32,
    pub slashing_count: i32,
    pub last_update_block_timestamp: String,
    #[serde(default)]
    pub slashings: Vec<OperatorSlashingDto>,
    #[serde(default)]
    pub strategies: Vec<OperatorStrategyLinkDto>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatorSlashingDto {
    pub block_timestamp: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatorStrategyLinkDto {
    pub total_shares: String,
    pub strategy: StrategyLiteDto,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StrategyLiteDto {
    pub id: String,
    pub exchange_rate: String,
    #[serde(default)]
    pub token: Option<TokenDto>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenDto {
    pub id: String,
    pub symbol: String,
    pub decimals: i32,
}
