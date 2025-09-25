use crate::models::ids::{OperatorId, StrategyId, TokenId};
use crate::models::operator::OperatorRiskRow;
use crate::models::operators_snapshot::{OperatorDto, OperatorsSnapshotData};
use crate::models::strategy::OperatorStrategyPosition;
use crate::models::time::BlockTimestamp;
use crate::models::token::{AtomicAmount, TokenRef, TvlByToken};

pub fn map_operators_snapshot(data: &OperatorsSnapshotData) -> Vec<OperatorRiskRow> {
    data.operators.iter().map(map_operator).collect()
}

fn map_operator(o: &OperatorDto) -> OperatorRiskRow {
    let operator_id = OperatorId(o.id.clone());

    let last_slash_at = o
        .slashings
        .get(0)
        .and_then(|s| s.block_timestamp.parse::<i64>().ok())
        .map(BlockTimestamp);

    let positions: Vec<OperatorStrategyPosition> = o
        .strategies
        .iter()
        .map(|link| {
            let token = TokenRef {
                id: TokenId(link.strategy.token.id.clone()),
                symbol: link.strategy.token.symbol.clone(),
                decimals: link.strategy.token.decimals as u8,
            };

            OperatorStrategyPosition {
                strategy_id: StrategyId(link.strategy.id.clone()),
                token,
                total_shares_atomic: AtomicAmount(link.total_shares.clone()),
                exchange_rate_atomic: AtomicAmount(link.strategy.exchange_rate.clone()),
            }
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

fn compute_tvl_by_token(p0: &Vec<OperatorStrategyPosition>) -> _ {
    todo!()
}

fn compute_hhi_over_shares(p0: &Vec<OperatorStrategyPosition>) -> _ {
    todo!()
}
