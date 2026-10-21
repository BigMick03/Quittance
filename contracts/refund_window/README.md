# refund-window

Checks whether a Quittance refund is still allowed, measured in Stellar ledger sequences.

## Signatures

```rust
pub fn refund_allowed(paid_ledger: u32, current_ledger: u32, window: u32) -> bool;
```

The refund is allowed when `current_ledger >= paid_ledger` and `current_ledger - paid_ledger <= window`. The ledger equal to `paid_ledger + window` is still allowed. The next ledger is not. A window of `0` allows only the payment ledger. A current ledger before the payment ledger is not allowed, including when the subtraction would wrap.

`RefundWindow` stores the payment ledger and the window length.

This crate is not wired into the invoice flow.

## Build and test

```bash
cargo test --manifest-path contracts/refund_window/Cargo.toml
cargo clippy --manifest-path contracts/refund_window/Cargo.toml --all-targets -- -D warnings
```
