#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenRef {
    pub id: super::ids::TokenId,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtomicAmount(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct TvlByToken {
    pub token: TokenRef,
    pub amount_atomic: AtomicAmount,
}
