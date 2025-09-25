use crate::models::ids::TokenId;
use crate::models::operator::OperatorRiskRow;
use crate::models::operators_snapshot::OperatorOrderBy::LastUpdateBlockTimestamp;
use crate::models::operators_snapshot::OrderDirection::{Asc, Desc};
use crate::models::operators_snapshot::{OperatorsSnapshotData, OperatorsSnapshotVars};
use crate::services::operators::operators_mapper::map_operators_snapshot;
use num_bigint::BigUint;
use num_traits::Zero;

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

pub async fn list_operator_risk<F: OperatorsSnapshotFetcher>(
    fetcher: &F,
    params: OperatorRiskParams,
) -> Result<OperatorRiskPage, anyhow::Error> {
    let vars = OperatorsSnapshotVars {
        first: params.first,
        skip: params.skip,
        order_by: LastUpdateBlockTimestamp,
        order_direction: match params.direction_hint() {
            SortDir::Asc => Asc,
            SortDir::Desc => Desc,
        },
        has_slashing: params.has_slashing,
    };
    let data = fetcher.fetch(vars).await?;

    let mut rows = map_operators_snapshot(&data);

    if let Some(min) = params.min_avs {
        rows.retain(|r| r.avs_count >= min);
    }
    if let Some(max_hhi) = params.concentration_max {
        rows.retain(|r| r.hhi <= max_hhi);
    }
    if let (Some(token), Some(min_atomic)) = (&params.token_filter, &params.min_tvl_atomic) {
        let min = parse_biguint(min_atomic);
        rows.retain(|r| match (tvl_atomic_for_token(r, token), &min) {
            (Some(amt), Some(th)) => amt >= *th,
            (Some(_), None) => true,
            (None, _) => false,
        });
    }

    rows.sort_by(|a, b| {
        use OperatorRiskOrderField::*;
        let ord = match params.field_key() {
            Tvl => {
                let token = match &params.token_filter {
                    Some(t) => t,
                    None => return std::cmp::Ordering::Equal,
                };
                let av = tvl_atomic_for_token(a, token);
                let bv = tvl_atomic_for_token(b, token);
                cmp_biguint_opt(&av, &bv)
            }
            Hhi => a
                .hhi
                .partial_cmp(&b.hhi)
                .unwrap_or(std::cmp::Ordering::Equal),
            AvsCount => a.avs_count.cmp(&b.avs_count),
            StrategyCount => a.strategy_count.cmp(&b.strategy_count),
            SlashingCount => a.slashing_count.cmp(&b.slashing_count),
            LastSlashAt => a.last_slash_at.cmp(&b.last_slash_at),
            LastUpdateBlockTs => std::cmp::Ordering::Equal,
        };
        match params.order.direction {
            SortDir::Asc => ord,
            SortDir::Desc => ord.reverse(),
        }
    });

    let has_more = data.operators.len() as i32 == params.first;
    let next_skip = params.skip + params.first;

    Ok(OperatorRiskPage {
        rows,
        next_skip,
        has_more,
    })
}

fn tvl_atomic_for_token(row: &OperatorRiskRow, token: &TokenId) -> Option<BigUint> {
    let mut sum = BigUint::zero();
    let mut any = false;
    for b in &row.tvl_by_token {
        if &b.token.id == token {
            if let Some(v) = parse_biguint(&b.amount_atomic.0) {
                sum += v;
                any = true;
            }
        }
    }
    if any { Some(sum) } else { None }
}

fn parse_biguint(s: &str) -> Option<BigUint> {
    if s.is_empty() {
        return None;
    }
    BigUint::parse_bytes(s.as_bytes(), 10)
}

fn cmp_biguint_opt(a: &Option<BigUint>, b: &Option<BigUint>) -> std::cmp::Ordering {
    match (a, b) {
        (Some(x), Some(y)) => x.cmp(y),
        (Some(_), None) => std::cmp::Ordering::Greater,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (None, None) => std::cmp::Ordering::Equal,
    }
}

impl OperatorRiskParams {
    pub fn direction_hint(&self) -> SortDir {
        self.order.direction
    }
    pub fn field_key(&self) -> OperatorRiskOrderField {
        self.order.field.clone()
    }
}
