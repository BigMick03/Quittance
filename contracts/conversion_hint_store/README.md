# `conversion-hint-store`

Non-authoritative conversion hint store — stores display-only currency
conversion hint rates with an explicit 'not authoritative' disclaimer flag.

This crate is a **Soroban contract primitive** for storing currency conversion
rates that are explicitly marked as **non-authoritative** — they are intended
for display purposes only (e.g., showing approximate fiat values in a UI) and
must not be used for financial settlement, accounting, or any authoritative
purpose.

This crate is **not** wired into the Next.js or Express MVP demos in this
PR. It is purely the storage primitive that future conversion display work
will call into.

## What it does

- Stores conversion hint rates in **e7 format** (7 decimal places) per currency pair.
- All hints are explicitly marked as **non-authoritative** via a disclaimer flag.
- **Admin-only write access** (set/remove), **public read access**.
- Currency pair validation (non-empty, length-limited symbols).

## Non-Authoritative Disclaimer

**All rates in this store are non-authoritative.** They are:

- **Display hints only** — for UI estimation, not settlement.
- **Not guaranteed accurate** — may be stale, approximate, or wrong.
- **Not for financial use** — never use for invoicing, payment calculation, or accounting.

The `is_non_authoritative` function always returns `true` for any hint
that exists, reinforcing this design intent.

## Public API

| Item | Type | Meaning |
|------|------|---------|
| `set_hint(env, caller, admin, from, to, rate_e7)` | `fn(&Env, &Address, &Address, &Symbol, &Symbol, i128) -> ConversionHintResult` | Admin-only: store a conversion hint. |
| `hint(env, from, to)` | `fn(&Env, &Symbol, &Symbol) -> Option<i128>` | Public read: get hint rate in e7. |
| `has_hint(env, from, to)` | `fn(&Env, &Symbol, &Symbol) -> bool` | Public read: check if hint exists. |
| `remove_hint(env, caller, admin, from, to)` | `fn(&Env, &Address, &Address, &Symbol, &Symbol) -> ConversionHintResult` | Admin-only: remove a hint. |
| `is_non_authoritative(env, from, to)` | `fn(&Env, &Symbol, &Symbol) -> bool` | Always `true` for existing hints (explicit disclaimer). |
| `ConversionHintError` | `enum` | Stable error codes with `#[repr(u32)]` discriminants. |

### Function signatures

```rust
pub fn set_hint(
    env: &Env,
    caller: &Address,
    admin: &Address,
    from: &Symbol,
    to: &Symbol,
    rate_e7: i128
) -> Result<(), ConversionHintError>

pub fn hint(env: &Env, from: &Symbol, to: &Symbol) -> Option<i128>

pub fn has_hint(env: &Env, from: &Symbol, to: &Symbol) -> bool

pub fn remove_hint(
    env: &Env,
    caller: &Address,
    admin: &Address,
    from: &Symbol,
    to: &Symbol
) -> Result<(), ConversionHintError>

pub fn is_non_authoritative(env: &Env, from: &Symbol, to: &Symbol) -> bool

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ConversionHintError {
    Unauthorized = 1,
    NotFound = 2,
    InvalidRate = 3,
    InvalidCurrency = 4,
    InternalError = 5,
}
impl ConversionHintError {
    pub fn message(&self) -> &'static str;
}
```

## Storage Layout

Each currency pair `(from, to)` uses two instance storage entries:

| Key | Value | Description |
|-----|-------|-------------|
| `HINT_{from}_{to}` | `i128` | Rate in e7 (e.g., 105000000 = 10.5) |
| `HINT_{from}_{to}_DISCLAIMER` | `bool` | Always `true` (non-authoritative flag) |

## Usage

```rust
use conversion_hint_store::{set_hint, hint, has_hint, remove_hint, is_non_authoritative, ConversionHintError};
use soroban_sdk::{Env, Address, Symbol, symbol_short};

let env = Env::default();
let admin = Address::generate(&env);
let caller = admin.clone(); // In practice, caller comes from auth context

// No hint initially
assert_eq!(hint(&env, &symbol_short!("USD"), &symbol_short!("XLM")), None);
assert!(!has_hint(&env, &symbol_short!("USD"), &symbol_short!("XLM")));

// Admin sets hint: 1 USD = 10.5 XLM (105000000 in e7)
set_hint(&env, &caller, &admin, &symbol_short!("USD"), &symbol_short!("XLM"), 105000000).unwrap();

// Read hint
assert_eq!(hint(&env, &symbol_short!("USD"), &symbol_short!("XLM")), Some(105000000));
assert!(has_hint(&env, &symbol_short!("USD"), &symbol_short!("XLM")));

// Explicit non-authoritative flag
assert!(is_non_authoritative(&env, &symbol_short!("USD"), &symbol_short!("XLM")));

// Admin removes hint
remove_hint(&env, &caller, &admin, &symbol_short!("USD"), &symbol_short!("XLM")).unwrap();
assert_eq!(hint(&env, &symbol_short!("USD"), &symbol_short!("XLM")), None);
```

## Errors

`ConversionHintError` is a `#[repr(u32)]` enum with stable discriminants for
off-chain matching:

| Variant | Code | Cause |
|---------|------|-------|
| `Unauthorized` | `1` | Caller is not the admin. |
| `NotFound` | `2` | Conversion hint not found for the pair. |
| `InvalidRate` | `3` | Rate must be positive (> 0). |
| `InvalidCurrency` | `4` | Currency symbol empty or too long (>12 chars). |
| `InternalError` | `5` | Unexpected internal state. |

Codes are stable so off-chain consumers can match on the integer.

## Scope and non-goals

In scope:

- Storing and retrieving non-authoritative conversion hints.
- Admin-gated writes, public reads.
- Explicit non-authoritative disclaimer flag on every hint.
- Currency symbol validation.

Out of scope:

- Authoritative price feeds or oracle integration.
- Settlement, invoicing, or payment calculation using hints.
- Historical rate tracking or TTL/expiry.
- Cross-contract calls.
- Wiring into the Next.js or Express MVP demos — that happens in a
  separate PR.

## Running the tests

This crate requires the Soroban SDK. Install Rust from <https://rustup.rs>
if missing, then:

```bash
cd contracts/conversion_hint_store
cargo test
cargo clippy --all-targets -- -D warnings
```

The integration tests in `tests/` cover:

- **Happy path** — set/get/remove hints, admin authorization, disclaimer flag.
- **Edge/boundary cases** — empty/long currencies, rate boundaries, Unicode.
- **Failure/error cases** — every error variant, unauthorized access, invalid inputs.
- **Additional scenarios** — multiple pairs, overwrite behavior, cross-pair isolation.

## License

MIT — see `LICENSE` at the repository root.