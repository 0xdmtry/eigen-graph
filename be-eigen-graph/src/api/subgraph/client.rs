use reqwest::{Client, Url};

#[derive(Clone, Debug)]
pub struct SubgraphClient {
    pub http: Client,
    pub endpoint: Url,
}

impl SubgraphClient {
    pub fn new(endpoint: Url) -> Self {
        let http = Client::builder()
            .user_agent("operators-snapshot/0.1")
            .build()
            .expect("subgraph client");
        Self { http, endpoint }
    }
}
