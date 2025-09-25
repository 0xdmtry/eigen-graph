use crate::models::ids::{OperatorId, StrategyId, TokenId};
use crate::models::operator::OperatorRiskRow;
use crate::models::operators_snapshot::{OperatorDto, OperatorsSnapshotData};
use crate::models::strategy::OperatorStrategyPosition;
use crate::models::time::BlockTimestamp;
use crate::models::token::{AtomicAmount, TokenRef, TvlByToken};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;

pub fn map_operators_snapshot(data: &OperatorsSnapshotData) -> Vec<OperatorRiskRow> {
    data.operators.iter().map(map_operator).collect()
}

fn map_operator(o: &OperatorDto) -> OperatorRiskRow {
    let operator_id = OperatorId(o.id.clone());

    let last_slash_at = o
        .slashings
        .first()
        .and_then(|s| s.block_timestamp.parse::<i64>().ok())
        .map(BlockTimestamp);

    let positions: Vec<OperatorStrategyPosition> = o
        .strategies
        .iter()
        .filter_map(|link| {
            let t = link.strategy.token.as_ref()?;

            Some(OperatorStrategyPosition {
                strategy_id: StrategyId(link.strategy.id.clone()),
                token: TokenRef {
                    id: TokenId(t.id.clone()),
                    symbol: t.symbol.clone(),
                    decimals: t.decimals as u8,
                },
                total_shares_atomic: AtomicAmount(link.total_shares.clone()),
                exchange_rate_atomic: AtomicAmount(link.strategy.exchange_rate.clone()),
            })
        })
        .collect();

    let hhi = compute_hhi_over_shares(&positions);

    let tvl_by_token = compute_tvl_by_token(&positions);

    OperatorRiskRow {
        operator_id,
        avs_count: o.avs_count as u32,
        strategy_count: o.strategy_count as u32,
        slashing_count: o.slashing_count as u32,
        last_slash_at,
        hhi,
        tvl_by_token,
        positions,
    }
}

fn compute_tvl_by_token(positions: &[OperatorStrategyPosition]) -> Vec<TvlByToken> {
    let mut acc: HashMap<String, (TokenRef, BigUint, u32)> = HashMap::new();

    for p in positions {
        let Some(shares) = parse_biguint(&p.total_shares_atomic.0) else {
            continue;
        };
        let Some(rate) = parse_biguint(&p.exchange_rate_atomic.0) else {
            continue;
        };

        if shares.is_zero() || rate.is_zero() {
            continue;
        }

        let decimals = p.token.decimals as u32;
        let scale = pow10(decimals);
        let num = &shares * &rate;
        let amount = &num / &scale;

        let entry = acc
            .entry(p.token.id.0.clone())
            .or_insert_with(|| (p.token.clone(), BigUint::zero(), decimals));
        entry.1 += amount;
    }

    let mut out = Vec::with_capacity(acc.len());
    for (_k, (token, sum_atomic, _dec)) in acc {
        out.push(TvlByToken {
            token,
            amount_atomic: AtomicAmount(sum_atomic.to_str_radix(10)),
        });
    }
    out
}

fn compute_hhi_over_shares(positions: &[OperatorStrategyPosition]) -> f64 {
    let mut shares: Vec<BigUint> = Vec::with_capacity(positions.len());
    for p in positions {
        if let Some(s) = parse_biguint(&p.total_shares_atomic.0) {
            if !s.is_zero() {
                shares.push(s);
            }
        }
    }
    if shares.is_empty() {
        return 0.0;
    }
    let sum: BigUint = shares
        .iter()
        .cloned()
        .fold(BigUint::zero(), |acc, x| acc + x);
    if sum.is_zero() {
        return 0.0;
    }

    let sum_f = biguint_to_f64(&sum);
    if sum_f == 0.0 || !sum_f.is_finite() {
        return 0.0;
    }

    let mut hhi = 0.0_f64;
    for s in shares {
        let wi = biguint_to_f64(&s) / sum_f;
        let wi = wi.clamp(0.0, 1.0);
        hhi += wi * wi;
    }
    hhi.clamp(0.0, 1.0)
}

fn parse_biguint(s: &str) -> Option<BigUint> {
    if s.is_empty() {
        return None;
    }
    BigUint::parse_bytes(s.as_bytes(), 10)
}

fn pow10(n: u32) -> BigUint {
    if n == 0 {
        return BigUint::one();
    }
    let ten = BigUint::from(10_u32);
    (0..n).fold(BigUint::one(), |acc, _| acc * &ten)
}

fn biguint_to_f64(n: &BigUint) -> f64 {
    let s = n.to_str_radix(10);
    s.parse::<f64>().unwrap_or(f64::INFINITY)
}
