# `batch-create-guard`

Batch invoice creation guard — validates a batch of invoice-creation
requests against a maximum batch size and a maximum total amount.

This crate is a **pure Rust helper** with no external dependencies. It is
intentionally dependency-free so it can be used both on-chain (inside a
Soroban contract via `no_std` + `alloc`) and off-chain (in the Express
backend, CLI tools, or tests) without pulling in the Soroban SDK or any
cryptographic primitives.

This crate is **not** wired into the Next.js or Express MVP demos in this
PR. It is purely the validation primitive that future invoice batching
work will call into.

## What it does

Given a list of invoice amounts (in **stroops**, Stellar's smallest
on-chain unit; `1 XLM == 10_000_000` stroops) and two policy parameters:

- `max_count` — maximum number of invoices allowed in a single batch
- `max_total` — maximum sum of all invoice amounts in the batch (stroops)

it returns `Ok(())` if the batch passes all checks, or a [`BatchError`]
indicating the first violation detected.

## Detection order

`validate_batch` checks conditions in a fixed order. The first failure is
returned:

1. **Invalid max_count** — `max_count` is zero.
2. **Invalid max_total** — `max_total` is negative.
3. **Empty batch** — the amounts slice is empty.
4. **Count exceeded** — batch length exceeds `max_count`.
5. **Negative amount** — any individual amount is negative.
6. **Zero amount** — any individual amount is zero.
7. **Total exceeded** — sum of all amounts exceeds `max_total`.

The order is part of the public contract so consumers can rely on
deterministic error precedence in diagnostics and tests.

## Public API

| Item | Type | Meaning |
|------|------|---------|
| `validate_batch(amounts, max_count, max_total)` | `fn(&[i128], u32, i128) -> BatchResult` | Primary validation entry point. |
| `validate_batch_with_config(amounts, config)` | `fn(&[i128], &BatchConfig) -> BatchResult` | Convenience wrapper using a pre-validated config. |
| `is_valid_batch(amounts, max_count, max_total)` | `fn(&[i128], u32, i128) -> bool` | Boolean convenience wrapper. |
| `BatchConfig::new(max_count, max_total)` | `fn(u32, i128) -> Result<BatchConfig, BatchError>` | Construct a validated config object. |
| `BatchError` | `enum` | Stable error codes with `#[repr(u32)]` discriminants. |

### Function signatures

```rust
pub fn validate_batch(amounts: &[i128], max_count: u32, max_total: i128) -> Result<(), BatchError>

pub fn validate_batch_with_config(amounts: &[i128], config: &BatchConfig) -> Result<(), BatchError>

pub fn is_valid_batch(amounts: &[i128], max_count: u32, max_total: i128) -> bool

pub struct BatchConfig {
    pub max_count: u32,
    pub max_total: i128,
}
impl BatchConfig {
    pub fn new(max_count: u32, max_total: i128) -> Result<Self, BatchError>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum BatchError {
    EmptyBatch = 1,
    CountExceeded = 2,
    TotalExceeded = 3,
    NegativeAmount = 4,
    ZeroAmount = 5,
    InvalidMaxCount = 6,
    InvalidMaxTotal = 7,
}
impl BatchError {
    pub fn message(&self) -> &'static str;
}
```

## Usage

```rust
use batch_create_guard::{validate_batch, BatchError, is_valid_batch, BatchConfig};

// 3 invoices at 10 XLM each (stroops), max 5 invoices, max 100 XLM total
let amounts = [10_000_000_000i128; 3];

// Primary API — returns detailed error on failure
let result = validate_batch(&amounts, 5, 100_000_000_000);
assert!(result.is_ok());

// Count exceeded
let too_many = [1_000_000_000i128; 6];
assert_eq!(
    validate_batch(&too_many, 5, 100_000_000_000),
    Err(BatchError::CountExceeded)
);

// Boolean convenience wrapper
assert!(is_valid_batch(&amounts, 5, 100_000_000_000));
assert!(!is_valid_batch(&too_many, 5, 100_000_000_000));

// Config object for repeated validation with same policy
let config = BatchConfig::new(5, 100_000_000_000).unwrap();
assert!(batch_create_guard::validate_batch_with_config(&amounts, &config).is_ok());
```

## Errors

`BatchError` is a `#[repr(u32)]` enum with stable discriminants for
off-chain matching:

| Variant | Code | Cause |
|---------|------|-------|
| `EmptyBatch` | `1` | The batch contains no invoice amounts. |
| `CountExceeded` | `2` | Batch size exceeds the configured maximum count. |
| `TotalExceeded` | `3` | Batch total exceeds the configured maximum total amount. |
| `NegativeAmount` | `4` | Batch contains a negative amount. |
| `ZeroAmount` | `5` | Batch contains a zero amount. |
| `InvalidMaxCount` | `6` | Maximum count must be greater than zero. |
| `InvalidMaxTotal` | `7` | Maximum total must be non-negative. |

Codes are stable so off-chain consumers can match on the integer.

## Scope and non-goals

In scope:

- Validating a batch of amounts against count and sum limits.
- Rejecting negative and zero amounts (Stellar payments must be positive).
- Using saturating addition to prevent overflow in the total sum.
- Providing a deterministic detection order for errors.

Out of scope:

- Asset validation (XLM vs USDC vs other) — the validator operates on
  raw `i128` stroop values; asset interpretation is the caller's concern.
- On-chain state validation (trustlines, balances, sequence numbers).
- Persistence or storage — this is a stateless validator.
- Authorization or authentication — no `require_auth` or identity checks.
- Wiring into the Next.js or Express MVP demos — that happens in a
  separate PR.

## Running the tests

This crate has no external dependencies, so it only needs a standard Rust
toolchain (1.74+ recommended). Install from <https://rustup.rs> if missing:

```bash
cd contracts/batch_create_guard
cargo test
cargo clippy --all-targets -- -D warnings
```

The integration tests in `tests/` cover:

- **Happy path** — single/multiple invoices, exact boundaries, config reuse.
- **Edge/boundary cases** — empty batch, off-by-one, zero limits, `i128::MAX`.
- **Failure/error cases** — every error variant, detection order, discriminants.
- **Additional scenarios** — freelancer milestones, USDC batches, mixed assets, client/server workflows, overflow safety.

## Build for Soroban (no_std + alloc)

To use this crate inside a Soroban contract, enable the `std` feature
off and depend on `alloc`:

```toml
batch-create-guard = { path = "../batch_create_guard", default-features = false, features = ["alloc"] }
```

The crate is written to be compatible with `no_std` + `alloc` when the
default `std` feature is disabled (the core logic uses only `core` and
`alloc` primitives). The test suite requires `std` and is gated behind
`#[cfg(test)]`.

## License

MIT — see `LICENSE` at the repository root.