use crate::api::subgraph::client::SubgraphClient;
use crate::models::subgraph::{GraphQLRequest, GraphQLResponse};
use serde::Deserialize;

const Q_TOKENS_BY_SYMBOL: &str = r#"
query TokensBySymbol($sym: String!, $first: Int!) {
  tokens(
    first: $first
    where: { symbol_contains_nocase: $sym }
    orderBy: lastUpdateBlockTimestamp
    orderDirection: desc
  ) { id symbol decimals }
}
"#;

const Q_DEPOSITS_BY_TOKEN: &str = r#"
query DepositsByToken($tokenId: String, $token: String, $since: BigInt!, $first: Int!, $skip: Int!) {
  deposits(
    first: $first
    skip: $skip
    orderBy: blockTimestamp
    orderDirection: asc
    where: {
      blockTimestamp_gte: $since
      token_: { id: $tokenId }
      token: $token
    }
  ) {
    id
    token { id symbol }
    staker { id }
    strategy { id }
    shares
    blockNumber
    blockTimestamp
    transactionHash
  }
}
"#;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenLite {
    pub id: String,
    pub symbol: String,
    pub decimals: Option<i32>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokensData {
    tokens: Vec<TokenLite>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositDto {
    pub id: String,
    pub token: Option<TokenLite>,
    pub staker: StakerLite,
    pub strategy: StrategyLite,
    pub shares: String,
    pub block_number: String,
    pub block_timestamp: String,
    pub transaction_hash: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakerLite {
    pub id: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategyLite {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DepositsData {
    deposits: Vec<DepositDto>,
}

pub async fn resolve_token_id(
    client: &SubgraphClient,
    symbol_or_addr: &str,
) -> Result<(String, String), anyhow::Error> {
    if symbol_or_addr.starts_with("0x") && symbol_or_addr.len() >= 42 {
        return Ok((symbol_or_addr.to_string(), symbol_or_addr.to_string()));
    }
    let body = GraphQLRequest {
        query: Q_TOKENS_BY_SYMBOL,
        variables: Some(serde_json::json!({ "sym": symbol_or_addr, "first": 10 })),
    };
    let resp = client
        .http
        .post(client.endpoint.clone())
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json::<GraphQLResponse<TokensData>>()
        .await?;
    let data = resp
        .data
        .ok_or_else(|| anyhow::anyhow!("empty token data"))?;

    if let Some(ex) = data
        .tokens
        .iter()
        .find(|t| t.symbol.eq_ignore_ascii_case(symbol_or_addr))
    {
        return Ok((ex.id.clone(), ex.symbol.clone()));
    }
    data.tokens
        .first()
        .map(|t| (t.id.clone(), t.symbol.clone()))
        .ok_or_else(|| anyhow::anyhow!("token not found by symbol: {symbol_or_addr}"))
}

pub async fn fetch_deposits_since(
    client: &SubgraphClient,
    _token_key: &str,
    token_id_or_addr: &str,
    since_ts: i64,
    page_size: i32,
) -> Result<Vec<DepositDto>, anyhow::Error> {
    let (token_id, token_field_val, use_id) = if token_id_or_addr.starts_with("0x") {
        (token_id_or_addr.to_string(), serde_json::Value::Null, true)
    } else {
        (
            String::new(),
            serde_json::Value::String(token_id_or_addr.to_string()),
            false,
        )
    };

    let mut all = Vec::new();
    let mut skip = 0;
    loop {
        let vars = if use_id {
            serde_json::json!({ "tokenId": token_id, "since": since_ts.to_string(), "first": page_size, "skip": skip })
        } else {
            serde_json::json!({ "token": token_field_val, "since": since_ts.to_string(), "first": page_size, "skip": skip })
        };
        let body = GraphQLRequest {
            query: Q_DEPOSITS_BY_TOKEN,
            variables: Some(vars),
        };

        let resp = client
            .http
            .post(client.endpoint.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<GraphQLResponse<DepositsData>>()
            .await?;

        if let Some(errs) = resp.errors {
            return Err(anyhow::anyhow!("graphql errors: {errs:?}"));
        }
        let data = resp
            .data
            .ok_or_else(|| anyhow::anyhow!("empty deposits data"))?;
        let n = data.deposits.len();
        all.extend(data.deposits);
        if n < page_size as usize {
            break;
        }
        skip += page_size;
        if skip >= 5000 {
            break;
        }
    }
    Ok(all)
}
