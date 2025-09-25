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
