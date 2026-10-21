# `payout-memo-binding`

Payout memo binding check — validates that a payout destination's required
memo (if any) matches the expected memo before release.

This crate is a **Soroban helper** for a single primitive: comparing an
optional expected memo against an optional provided memo. It is intentionally
minimal and dependency-light (only `soroban-sdk`).

This crate is **not** wired into the Next.js or Express MVP demos in this
PR. It is purely the validation primitive that future payout work will call
into.

## What it does

Given an optional expected memo (what the destination requires) and an
optional provided memo (what the payment includes), it returns `Ok(())` if
the binding is satisfied, or a [`MemoBindingError`] indicating the
violation.

## Rules

1. **No memo required** (`expected = None`): Any provided memo is accepted
   (including `None`). The destination doesn't care about memos.
2. **Memo required** (`expected = Some(memo)`): The provided memo must be
   `Some(expected_memo)` — exact match required.

## Detection order

`matches_required_memo` checks conditions in a fixed order:

1. **Missing required memo** — expected is `Some` but provided is `None`.
2. **Memo mismatch** — expected is `Some` and provided is `Some` but they
   differ.
3. **Satisfied** — all other cases (including `expected = None`).

The order is part of the public contract so consumers can rely on
deterministic error precedence in diagnostics and tests.

## Public API

| Item | Type | Meaning |
|------|------|---------|
| `matches_required_memo(env, expected, provided)` | `fn(&Env, Option<Symbol>, Option<Symbol>) -> MemoBindingResult` | Primary validation entry point. |
| `memo_binding_satisfied(env, expected, provided)` | `fn(&Env, Option<Symbol>, Option<Symbol>) -> bool` | Boolean convenience wrapper. |
| `encode_expected_memo(expected)` | `fn(Option<Symbol>) -> Symbol` | Encode for contract storage. |
| `decode_expected_memo(stored)` | `fn(Symbol) -> Option<Symbol>` | Decode from contract storage. |
| `MemoBindingError` | `enum` | Stable error codes with `#[repr(u32)]` discriminants. |

### Function signatures

```rust
pub fn matches_required_memo(
    env: &Env,
    expected: Option<Symbol>,
    provided: Option<Symbol>
) -> Result<(), MemoBindingError>

pub fn memo_binding_satisfied(
    env: &Env,
    expected: Option<Symbol>,
    provided: Option<Symbol>
) -> bool

pub fn encode_expected_memo(expected: Option<Symbol>) -> Symbol

pub fn decode_expected_memo(stored: Symbol) -> Option<Symbol>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum MemoBindingError {
    MissingRequiredMemo = 1,
    MemoMismatch = 2,
    InternalError = 3,
}
impl MemoBindingError {
    pub fn message(&self) -> &'static str;
}
```

## Usage

```rust
use payout_memo_binding::{matches_required_memo, memo_binding_satisfied, MemoBindingError, encode_expected_memo, decode_expected_memo};
use soroban_sdk::{Env, Symbol, symbol_short};

let env = Env::default();
let expected_memo = symbol_short!("INV-123");

// No memo required, no memo provided -> OK
assert!(matches_required_memo(&env, None, None).is_ok());

// No memo required, memo provided -> OK (extra memo allowed)
assert!(matches_required_memo(&env, None, Some(symbol_short!("extra"))).is_ok());

// Memo required, correct memo provided -> OK
assert!(matches_required_memo(&env, Some(expected_memo), Some(expected_memo)).is_ok());

// Memo required, no memo provided -> Error
assert_eq!(
    matches_required_memo(&env, Some(expected_memo), None),
    Err(MemoBindingError::MissingRequiredMemo)
);

// Memo required, wrong memo provided -> Error
assert_eq!(
    matches_required_memo(&env, Some(expected_memo), Some(symbol_short!("WRONG"))),
    Err(MemoBindingError::MemoMismatch)
);

// Boolean convenience wrapper
assert!(memo_binding_satisfied(&env, None, None));
assert!(!memo_binding_satisfied(&env, Some(expected_memo), None));

// Storage encoding/decoding
let stored = encode_expected_memo(Some(expected_memo));
assert_eq!(decode_expected_memo(stored), Some(expected_memo));
let stored_none = encode_expected_memo(None);
assert_eq!(decode_expected_memo(stored_none), None);
```

## Errors

`MemoBindingError` is a `#[repr(u32)]` enum with stable discriminants for
off-chain matching:

| Variant | Code | Cause |
|---------|------|-------|
| `MissingRequiredMemo` | `1` | The payout destination requires a memo but none was provided. |
| `MemoMismatch` | `2` | The provided memo does not match the expected memo. |
| `InternalError` | `3` | Internal error during memo binding check (should not occur). |

Codes are stable so off-chain consumers can match on the integer.

## Storage encoding

The crate provides `encode_expected_memo` / `decode_expected_memo` for
persisting the expected memo in contract storage. This makes the "no memo
required" state explicit (`NO_MEMO_EXPECTED` sentinel) rather than relying
on optional presence in storage.

## Scope and non-goals

In scope:

- Validating memo binding for payout destinations.
- Exact Symbol comparison (Soroban's `Symbol` is interned, so `==` is
  pointer/identity equality — fast and correct).
- Storage encoding helpers for contract persistence.

Out of scope:

- Memo format validation (length, character set) — that's a separate
  concern handled by `destination_guard` / `memo_validator`.
- On-chain state validation (trustlines, balances, sequence numbers).
- Authorization or authentication — no `require_auth` or identity checks.
- Wiring into the Next.js or Express MVP demos — that happens in a
  separate PR.

## Running the tests

This crate requires the Soroban SDK. Install Rust from <https://rustup.rs>
if missing, then:

```bash
cd contracts/payout_memo_binding
cargo test
cargo clippy --all-targets -- -D warnings
```

The integration tests in `tests/` cover:

- **Happy path** — no memo required, memo required with match, extra memo allowed.
- **Edge/boundary cases** — empty symbols, max-length symbols, sentinel values.
- **Failure/error cases** — every error variant, detection order, discriminant stability.
- **Additional scenarios** — contract storage round-trip, multiple destinations, batch validation.

## License

MIT — see `LICENSE` at the repository root.