# grace-period-guard

Checks whether a Quittance payment still falls inside the grace window after an invoice due time.

A payment at `due_at` is on time. Extra `grace_secs` after that instant are still on time, including the last second. One second later is late. This crate is not wired into the invoice flow.

## Signatures

```rust
pub fn is_within_grace(due_at: u64, grace_secs: u64, paid_at: u64) -> bool;
pub fn remaining_grace(due_at: u64, grace_secs: u64, paid_at: u64) -> i64;
```

`is_within_grace` is true when `paid_at <= due_at + grace_secs`. `remaining_grace` is the seconds left until that deadline, and it is negative when the payment is late. The sum uses `i128`, so a timestamp near `u64::MAX` does not wrap. A difference that does not fit in `i64` saturates.

`GraceWindow` stores the same two inputs and exposes `contains`.

## Build and test

```bash
cargo test --manifest-path contracts/grace_period_guard/Cargo.toml
cargo clippy --manifest-path contracts/grace_period_guard/Cargo.toml --all-targets -- -D warnings
```
