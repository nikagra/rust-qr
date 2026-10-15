# PROGRESS.md

Roadmap: PLAN.md. This file tracks where things stand. Newest entry on top.

## Current state
- **Next step:** Step 2b, Reed–Solomon `encode` (polynomial division).
- **Done:** Step 1 (GF(256)), Step 2a (generator polynomial).
- **Repo:** `master` at d9f0ae5. 6 tests pass. 4 dead-code warnings, which go away in Step 3.

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

## Open questions / review points
- `generator()` stores coefficients constant-term first (`[2, 3, 1]` for n = 2, i.e. x² + 3x + 2).
  Thonky's division works highest-degree first, so decide the order before writing `encode`.
- `main.rs` still prints a placeholder and declares the modules; Step 3 moves them to `lib.rs`.

## Log
### 2026-10-15
- Done: GF(256) module (add/mul/div/pow, tests), RS generator with a root-check test.
- Next: Step 2b.
