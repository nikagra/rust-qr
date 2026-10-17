# PROGRESS.md

Roadmap: PLAN.md. This file tracks where things stand. Newest entry on top.

## Current state
- **Next step:** Step 3, library skeleton (`lib.rs`) and error type.
- **Done:** Step 1 (GF(256)), Step 2a (generator polynomial), Step 2b (Reed–Solomon `encode`).
- **Repo:** `master` at bc9d766. 8 tests pass, including Thonky's HELLO WORLD 1-M EC codewords.
  Dead-code warnings remain until Step 3 moves the modules into `lib.rs`.

## Decisions
- Encoder first: GF/RS before anything else, the library skeleton (`lib.rs`) right after RS encode,
  the CLI near the end.
- Byte mode only; Kanji, numeric and alphanumeric modes are stretch goals.
- BitBuffer is its own step, before data encoding.
- Versions 1–10 first; 11–40 is a separate later step.
- `--ec-level` is added together with the other CLI flags (Step 13).
- `rqrr` round-trip tests are the final verification step.
- `Gf256` is a struct holding runtime-built log/exp tables, passed by `&Gf256`.
  The `Gf(u8)` newtype with operator traits is an optional refactor, not required.
- Polynomials are stored highest degree first (`generator(2)` is `[1, 3, 2]`, i.e. x² + 3x + 2),
  matching the order of the data codewords.

## Open questions / review points
- `main.rs` still prints a placeholder and declares the modules; Step 3 moves them to `lib.rs`.

## Log
### 2026-10-16
- Done: RS `encode` by polynomial division; generator switched to highest degree first.
  Tests: a hand-worked `[5, 9]` → `[0, 12]` case and HELLO WORLD 1-M.
- Next: Step 3.

### 2026-10-15
- Done: GF(256) module (add/mul/div/pow, tests), RS generator with a root-check test.
- Next: Step 2b.
