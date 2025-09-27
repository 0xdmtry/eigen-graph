use crate::models::operators_aggr::{
    AggregatorParams, BarItem, Donut, GraphEdge, OperatorAggregate, Outliers, StrategySlice,
    TableRow,
};
use crate::models::operators_aggr::{UniformOperator, UniformPage};
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};
use std::cmp::Ordering;

pub fn aggregate(
    page: &UniformPage,
    params: &AggregatorParams,
    _now_ts: i64,
) -> Vec<OperatorAggregate> {
    let min_tvl = params
        .min_tvl_atomic
        .as_ref()
        .and_then(|s| parse_u256(s).ok());

    let mut out = Vec::with_capacity(page.operators.len());
    for op in &page.operators {
        out.push(aggregate_one(op));
    }

    if let Some(min) = min_tvl {
        out.retain(|a| parse_u256(&a.tvl_total_atomic).is_ok_and(|t| t >= min));
    }

    out.sort_by(|a, b| {
        let at = parse_u256(&a.tvl_total_atomic).unwrap_or_else(|_| BigUint::zero());
        let bt = parse_u256(&b.tvl_total_atomic).unwrap_or_else(|_| BigUint::zero());
        match bt.cmp(&at) {
            Ordering::Equal => a.operator_id.cmp(&b.operator_id),
            o => o,
        }
    });

    out
}

pub fn to_table_rows(aggr: &[OperatorAggregate]) -> Vec<TableRow> {
    aggr.iter()
        .map(|a| TableRow {
            operator_id: a.operator_id.clone(),
            avs_count: a.avs_count,
            strategy_count: a.strategy_count,
            slashing_count: a.slashing_count,
            last_slash_at: a.last_slash_at,
            last_update_block_ts: a.last_update_block_ts,
            tvl_total_atomic: a.tvl_total_atomic.clone(),
            hhi_strategy: a.hhi_strategy,
            nonzero_strategy_count: a.nonzero_strategy_count,
        })
        .collect()
}

pub fn to_bar_series(aggr_sorted: &[OperatorAggregate], top_n: usize) -> Vec<BarItem> {
    aggr_sorted
        .iter()
        .take(top_n.min(aggr_sorted.len()).max(1))
        .map(|a| BarItem {
            operator_id: a.operator_id.clone(),
            tvl_total_atomic: a.tvl_total_atomic.clone(),
        })
        .collect()
}

pub fn to_donuts(aggr: &[OperatorAggregate], focus_operator_id: &Option<String>) -> Vec<Donut> {
    if let Some(id) = focus_operator_id {
        if let Some(a) = aggr.iter().find(|x| &x.operator_id == id) {
            return vec![Donut {
                operator_id: a.operator_id.clone(),
                slices: a.strategy_breakdown.clone(),
            }];
        }
        return vec![];
    }

    aggr.iter()
        .map(|a| Donut {
            operator_id: a.operator_id.clone(),
            slices: a.strategy_breakdown.clone(),
        })
        .collect()
}

