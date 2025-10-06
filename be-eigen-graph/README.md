### Commands

```bash
cargo check
```

```bash
cargo fmt -- --check
```

```bash 
cargo clippy --all-targets --all-features -- -D warnings
```

Allows unused code

```bash
cargo clippy --all-targets --all-features -- -D warnings -A unused -A dead_code
```


```bash
psql -d eigen-graph -U root
```

```bash
\dt
```

```bash
docker compose -f be.yml up --build
```

Prometheus
http://localhost:9090
```text
sum by (result)(rate(subgraph_requests_total[5m]))
sum by (op,result)(rate(cache_ops_total[5m]))
histogram_quantile(0.95, sum by (le,op)(rate(db_query_duration_seconds_bucket[5m])))
sum by (kind)(rate(app_errors_total[5m]))
```