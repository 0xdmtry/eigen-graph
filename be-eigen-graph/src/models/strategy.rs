use super::ids::StrategyId;
use super::token::{AtomicAmount, TokenRef};

#[derive(Clone, Debug, PartialEq)]
pub struct OperatorStrategyPosition {
    pub strategy_id: StrategyId,
    pub token: TokenRef,
    pub total_shares_atomic: AtomicAmount,
    pub exchange_rate_atomic: AtomicAmount,
}
