# `seller-reputation-counter`

Seller reputation counter — view-only counter tracking successful vs.
disputed invoice counts per seller.

This crate is a **Soroban contract** for tracking seller reputation on-chain.
It maintains two counters per seller: successful invoices and disputed invoices.
The data is view-only (no authorization required for reads) and can be used
by frontends, marketplaces, or other contracts to display seller reputation.

This crate is **not** wired into the Next.js or Express MVP demos in this
PR. It is purely the reputation primitive that future marketplace work
will call into.

## What it does

- Tracks **successful invoices** and **disputed invoices** per seller address.
- Provides derived metrics: total invoices, success rate percentage.
- Admin-gated writes (record_success, record_dispute), public reads.
- Overflow-safe with saturating arithmetic.

## Public API

| Item | Type | Meaning |
|------|------|---------|
| `record_success(env, seller)` | `fn(&Env, &Address) -> ReputationResult` | Record a successful invoice. |
| `record_dispute(env, seller)` | `fn(&Env, &Address) -> ReputationResult` | Record a disputed invoice. |
| `score_of(env, seller)` | `fn(&Env, &Address) -> Option<ReputationScore>` | Get (successes, disputes). |
| `has_reputation(env, seller)` | `fn(&Env, &Address) -> bool` | Check if seller has record. |
| `total_invoices(env, seller)` | `fn(&Env, &Address) -> u32` | Get total invoice count. |
| `success_rate(env, seller)` | `fn(&Env, &Address) -> Option<u32>` | Get success rate (0-100). |
| `ReputationError` | `enum` | Stable error codes with `#[repr(u32)]` discriminants. |

### Function signatures

```rust
pub fn record_success(env: &Env, seller: &Address) -> Result<(), ReputationError>
pub fn record_dispute(env: &Env, seller: &Address) -> Result<(), ReputationError>
pub fn score_of(env: &Env, seller: &Address) -> Option<(u32, u32)>
pub fn has_reputation(env: &Env, seller: &Address) -> bool
pub fn total_invoices(env: &Env, seller: &Address) -> u32
pub fn success_rate(env: &Env, seller: &Address) -> Option<u32>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ReputationError {
    InvalidSeller = 1,
    InternalError = 2,
}
impl ReputationError {
    pub fn message(&self) -> &'static str;
}
```

## Storage Layout

Each seller uses one instance storage entry:

| Key | Value | Description |
|-----|-------|-------------|
| `REP_{seller_address}` | `(u32, u32)` | Tuple of (success_count, dispute_count) |

## Usage

```rust
use seller_reputation_counter::{record_success, record_dispute, score_of, total_invoices, success_rate, ReputationError};
use soroban_sdk::{Env, Address};

let env = Env::default();
let seller = Address::generate(&env);

// Record some outcomes
record_success(&env, &seller).unwrap();
record_success(&env, &seller).unwrap();
record_dispute(&env, &seller).unwrap();

// Query reputation
assert_eq!(score_of(&env, &seller), Some((2, 1)));
assert_eq!(total_invoices(&env, &seller), 3);
assert_eq!(success_rate(&env, &seller), Some(66)); // 2/3 = 66%
```

## Errors

`ReputationError` is a `#[repr(u32)]` enum with stable discriminants for
off-chain matching:

| Variant | Code | Cause |
|---------|------|-------|
| `InvalidSeller` | `1` | Seller address is invalid. |
| `InternalError` | `2` | Unexpected internal state (e.g., overflow). |

Codes are stable so off-chain consumers can match on the integer.

## Scope and non-goals

In scope:

- Tracking successful and disputed invoice counts per seller.
- Derived metrics: total invoices, success rate.
- Overflow-safe saturating arithmetic.
- Public read access, controlled write access.

Out of scope:

- Authorization framework (admin gating) — that belongs in a wrapper contract.
- Invoice metadata storage (amounts, dates, parties) — this is counters only.
- Dispute resolution logic — this only records outcomes.
- Wiring into the Next.js or Express MVP demos — that happens in a
  separate PR.

## Running the tests

This crate requires the Soroban SDK. Install Rust from <https://rustup.rs>
if missing, then:

```bash
cd contracts/seller_reputation_counter
cargo test
cargo clippy --all-targets -- -D warnings
```

The integration tests in `tests/` cover:

- **Happy path** — record success/dispute, query scores, multiple sellers.
- **Edge/boundary cases** — unknown sellers, zero counts, overflow protection.
- **Failure/error cases** — invalid sellers, overflow handling.
- **Additional scenarios** — success rate calculations, marketplace workflows.

## License

MIT — see `LICENSE` at the repository root.