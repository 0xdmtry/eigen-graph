use reqwest::{Client, Url};

pub struct SubgraphClient {
    http: Client,
    endpoint: Url,
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
