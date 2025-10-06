use axum::{
    Router,
    extract::{MatchedPath, Request},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
};
use once_cell::sync::Lazy;
use prometheus::{
    Encoder, HistogramVec, IntCounterVec, Registry, TextEncoder, histogram_opts, opts,
};
use std::time::Instant;

static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);
static HTTP_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    let v = IntCounterVec::new(
        opts!("http_requests_total", "total http requests"),
        &["method", "path", "status"],
    )
    .unwrap();
    REGISTRY.register(Box::new(v.clone())).ok();
    v
});
static HTTP_HIST: Lazy<HistogramVec> = Lazy::new(|| {
    let v = HistogramVec::new(
        histogram_opts!(
            "http_request_duration_seconds",
            "http request duration seconds"
        ),
        &["method", "path", "status"],
    )
    .unwrap();
    REGISTRY.register(Box::new(v.clone())).ok();
    v
});

pub async fn track_http_metrics(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().as_str().to_string();
    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| req.uri().path().to_string());
    let resp = next.run(req).await;
    let status = resp.status().as_u16().to_string();
    HTTP_COUNTER
        .with_label_values(&[&method, &path, &status])
        .inc();
    HTTP_HIST
        .with_label_values(&[&method, &path, &status])
        .observe(start.elapsed().as_secs_f64());
    resp
}

pub async fn export() -> impl IntoResponse {
    let mf = REGISTRY.gather();
    let mut buf = Vec::new();
    TextEncoder::new().encode(&mf, &mut buf).ok();
    axum::http::Response::builder()
        .header(
            axum::http::header::CONTENT_TYPE,
            TextEncoder::new().format_type(),
        )
        .body(String::from_utf8(buf).unwrap_or_default())
        .unwrap()
}

pub fn routes() -> Router {
    Router::new().route("/metrics", get(export))
}
