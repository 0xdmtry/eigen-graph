use crate::models::operators_aggr::{PageMeta, UniformOperator, UniformPage, UniformPosition};
use crate::models::operators_snapshot::OperatorsSnapshotData;

pub fn from_subgraph_adapt(page: &OperatorsSnapshotData, first: i32, skip: i32) -> UniformPage {
    let operators = page
        .operators
        .iter()
        .map(|op| {
            let last_slash_at = op
                .slashings
                .first()
                .and_then(|s| s.block_timestamp.parse::<i64>().ok());

            let last_update_block_ts = op.last_update_block_timestamp.parse::<i64>().unwrap_or(0);

            let positions = op
                .strategies
                .iter()
                .filter_map(|link| {
                    let token = match &link.strategy.token {
                        Some(t) => t,
                        None => return None,
                    };
                    Some(UniformPosition {
                        strategy_id: link.strategy.id.clone(),
                        token_id: token.id.clone(),
                        token_symbol: token.symbol.clone(),
                        token_decimals: token.decimals,
                        total_shares: link.total_shares.clone(),
                        exchange_rate: link.strategy.exchange_rate.clone(),
                    })
                })
                .collect();

            UniformOperator {
                operator_id: op.id.clone(),
                avs_count: op.avs_count,
                strategy_count: op.strategy_count,
                slashing_count: op.slashing_count,
                last_slash_at,
                last_update_block_ts,
                positions,
            }
        })
        .collect();

    UniformPage {
        operators,
        page_meta: PageMeta { first, skip },
    }
}
