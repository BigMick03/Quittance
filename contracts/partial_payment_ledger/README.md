# partial-payment-ledger

Tracks how much of a Quittance invoice has been paid and refuses a payment that would go past the total.

## Signatures

```rust
pub fn record_payment(
    total: i128,
    paid_so_far: i128,
    new_payment: i128,
) -> Result<(i128, bool), PaymentError>;
```

The success value is `(new_paid_so_far, is_fully_paid)`. `is_fully_paid` is true only when the new amount equals `total`. An amount that would exceed `total` returns `PaymentError::Overpayment` and is not capped. Negative inputs return `PaymentError::NegativeAmount`.

`PaymentProgress` stores the same total and running amount. A failed `record` leaves `paid_so_far` unchanged.

This crate is not wired into the invoice flow.

## Build and test

```bash
cargo test --manifest-path contracts/partial_payment_ledger/Cargo.toml
cargo clippy --manifest-path contracts/partial_payment_ledger/Cargo.toml --all-targets -- -D warnings
```
