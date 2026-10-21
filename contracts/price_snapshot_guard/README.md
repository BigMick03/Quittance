# price-snapshot-guard

Stores a price quote as a rate plus a timestamp and checks whether that quote is still fresh.

## Signatures

```rust
pub fn is_fresh(snapshot_at: u64, now: u64, max_age_secs: u64) -> bool;
```

The snapshot is fresh when `now >= snapshot_at` and `now - snapshot_at <= max_age_secs`. An age equal to `max_age_secs` is still fresh. One second later is stale. A `now` before the snapshot is not fresh, including when the subtraction would wrap.

`PriceSnapshot` stores a positive `rate` and `snapshot_at`. A rate of zero or less is rejected.

This crate is not wired into the invoice flow.

## Build and test

```bash
cargo test --manifest-path contracts/price_snapshot_guard/Cargo.toml
cargo clippy --manifest-path contracts/price_snapshot_guard/Cargo.toml --all-targets -- -D warnings
```
