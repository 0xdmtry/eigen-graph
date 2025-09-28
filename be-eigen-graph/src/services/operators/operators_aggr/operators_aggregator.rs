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

pub fn to_graph_edges(aggr: &[OperatorAggregate]) -> Vec<GraphEdge> {
    let mut edges = Vec::new();
    for a in aggr {
        for s in &a.strategy_breakdown {
            edges.push(GraphEdge {
                operator_id: a.operator_id.clone(),
                strategy_id: s.strategy_id.clone(),
                weight_atomic: s.tvl_atomic.clone(),
            });
        }
    }
    edges
}

pub fn detect_outliers(
    aggr: &[OperatorAggregate],
    hhi_threshold: f64,
    recent_window_s: i64,
    now_ts: i64,
) -> Outliers {
    let high_concentration = aggr
        .iter()
        .filter(|a| a.hhi_strategy >= hhi_threshold)
        .map(|a| a.operator_id.clone())
        .collect();

    let zero_share = aggr
        .iter()
        .filter(|a| a.zero_share_flag)
        .map(|a| a.operator_id.clone())
        .collect();

    let cutoff = now_ts.saturating_sub(recent_window_s);
    let recent_slashes = aggr
        .iter()
        .filter(|a| a.last_slash_at.is_some_and(|ts| ts >= cutoff))
        .map(|a| a.operator_id.clone())
        .collect();

    Outliers {
        high_concentration,
        zero_share,
        recent_slashes,
    }
}

fn aggregate_one(op: &UniformOperator) -> OperatorAggregate {
    let mut slices: Vec<(String, BigUint)> = Vec::with_capacity(op.positions.len());
    let mut zero_share_flag = false;

    for p in &op.positions {
        if p.total_shares == "0" {
            zero_share_flag = true;
        }

        let rate = if p.exchange_rate.is_empty() || p.exchange_rate == "0" {
            "1000000000000000000"
        } else {
            &p.exchange_rate
        };

        let tvl = mul_u256(&p.total_shares, rate).unwrap_or_else(|_| BigUint::zero());
        slices.push((p.strategy_id.clone(), tvl));
    }

    let total: BigUint = slices.iter().fold(BigUint::zero(), |acc, (_, v)| acc + v);

    let total_f = to_f64(&total);
    let breakdown: Vec<StrategySlice> = if total.is_zero() || total_f == 0.0 {
        slices
            .into_iter()
            .map(|(sid, amt)| StrategySlice {
                strategy_id: sid,
                tvl_atomic: amt.to_string(),
                share: 0.0,
            })
            .collect()
    } else {
        let mut v: Vec<StrategySlice> = slices
            .into_iter()
            .map(|(sid, amt)| {
                let share = to_f64(&amt) / total_f;
                StrategySlice {
                    strategy_id: sid,
                    tvl_atomic: amt.to_string(),
                    share,
                }
            })
            .collect();

        v.sort_by(|a, b| {
            let aa = parse_u256(&a.tvl_atomic).unwrap_or_else(|_| BigUint::zero());
            let bb = parse_u256(&b.tvl_atomic).unwrap_or_else(|_| BigUint::zero());
            match bb.cmp(&aa) {
                Ordering::Equal => a.strategy_id.cmp(&b.strategy_id),
                o => o,
            }
        });
        v
    };

    let top_strategy_share = breakdown.first().map(|s| s.share).unwrap_or(0.0);
    let hhi_strategy = breakdown
        .iter()
        .fold(0.0_f64, |acc, s| acc + s.share * s.share);

    OperatorAggregate {
        operator_id: op.operator_id.clone(),
        avs_count: op.avs_count,
        strategy_count: op.strategy_count,
        slashing_count: op.slashing_count,
        last_slash_at: op.last_slash_at,
        last_update_block_ts: op.last_update_block_ts,

        tvl_total_atomic: total.to_string(),
        nonzero_strategy_count: op
            .positions
            .iter()
            .filter(|p| p.total_shares != "0")
            .count() as i32,

        strategy_breakdown: breakdown,
        top_strategy_share,
        hhi_strategy,
        zero_share_flag,
    }
}

fn parse_u256(s: &str) -> Result<BigUint, ()> {
    BigUint::parse_bytes(s.as_bytes(), 10).ok_or(())
}

fn mul_u256(a: &str, b: &str) -> Result<BigUint, ()> {
    let x = parse_u256(a)?;
    let y = parse_u256(b)?;
    Ok(x * y)
}

fn to_f64(x: &BigUint) -> f64 {
    x.to_f64().unwrap_or(0.0)
}
