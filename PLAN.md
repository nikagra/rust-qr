# PLAN.md — rust-qr: QR code generator from scratch

## Goal
A CLI (`qrcode [OPTIONS] <CONTENT>`) that encodes text into a QR code and writes a PNG,
with the entire encoding pipeline implemented by hand (no `qrcode` crate in the main code).

This file is the roadmap and does not change often. Where things stand, which decisions were made
and what comes next all go in PROGRESS.md.

## Ground rules
- Allowed dependencies: `clap`, `anyhow`/`thiserror`, `image` (PNG output only).
- Dev-dependencies for verification: `rqrr` (decoder), optionally `qrcode` as a test oracle.
- Every step ends with passing `cargo test`, clean `cargo clippy`, and a git commit.
- Scope: Byte mode, versions 1–10, all four EC levels (L/M/Q/H). Everything else is a later step or a stretch goal.
- Reference: ISO/IEC 18004 concepts as explained in Thonky's QR Code Tutorial.
  Ask for the relevant part of the spec to be explained before each step.

## Pipeline overview
text → bit stream → codewords → split into blocks
→ Reed–Solomon EC per block → interleave → place in matrix
→ choose mask → write format/version info → render PNG

## Module layout
- main.rs     – CLI only
- lib.rs      – public `encode(text, ec_level) -> Result<QrCode>`
- error.rs    – error type (`thiserror`)
- gf256.rs    – Galois field arithmetic
- rs.rs       – Reed–Solomon encoder
- bits.rs     – BitBuffer
- mode.rs     – encoding modes (Byte only for now)
- version.rs  – version/capacity/block tables, version selection
- blocks.rs   – block splitting and interleaving (or inside rs.rs)
- matrix.rs   – module grid, function patterns, data placement
- mask.rs     – mask patterns, penalty scoring
- format.rs   – format & version info (BCH codes)
- render.rs   – PNG output

---

## Step 1 — GF(256) arithmetic
Build: field over primitive polynomial 0x11D, generator α = 2; exp/log tables;
add (XOR), multiply, divide, power.
Rust focus: `impl` blocks, arrays, tests. Optional refactor: newtype `struct Gf(u8)` with
`Add`/`Mul`/`Div` traits, `const fn` tables.
Done when: field properties hold in tests (a·b/b = a, a·1 = a, division by zero panics).

## Step 2 — Reed–Solomon encoding
2a. Generator polynomial ∏(x − αⁱ) for i in 0..n.
2b. Polynomial division producing n EC codewords (`encode`).
Rust focus: `Vec<u8>` polynomials, slices, avoiding unnecessary clones, coefficient order.
Done when: EC codewords for a known worked example match exactly
(Thonky's "HELLO WORLD" 1-M example).

## Step 3 — Library skeleton and errors
Build: `lib.rs` declaring the modules (moves `mod` lines out of `main.rs`, which clears the
dead-code warnings), an error enum with `thiserror` (input too long, invalid input),
empty stubs for the remaining modules.
Rust focus: crates (lib + bin in one package), modules and visibility, `Result` and `?`.
Done when: `cargo build` has no warnings and `cargo test` still passes.

## Step 4 — BitBuffer
Build: `BitBuffer` with `push_bits(value: u32, count: u8)`, `len()`,
padding to byte boundary, conversion to `Vec<u8>`.
Rust focus: structs, ownership vs borrowing (`&self`/`&mut self`), bit operations,
`debug_assert!`, unit tests.
Done when: tests cover pushing values of various widths and byte conversion,
including edge cases (0 bits, 32 bits, value too wide for count).

## Step 5 — Byte-mode data encoding
Build: `enum Mode { Byte }`, `enum EcLevel { L, M, Q, H }`; mode indicator (0100),
character count indicator (8 bits for v1–9, 16 bits for v10+), data bytes (UTF-8),
terminator (up to 4 zero bits), pad to byte boundary, pad codewords 0xEC/0x11 up to capacity.
Rust focus: enums, `match`, `&str` vs `String` vs `&[u8]`, iterators.
Done when: for a known input and capacity, the codeword sequence matches a worked example.

## Step 6 — Version tables and version selection (v1–10)
Build: tables of total codewords, EC codewords per block, and block structure
(group 1/2 block counts and sizes) for versions 1–10 × L/M/Q/H. Choose the smallest version that fits.
Rust focus: `const` arrays, a `struct` per table row, `Option`, the "data too long" error.
Done when: version selection is tested at capacity boundaries
(max length fits in version N, one more byte needs N+1, too long for v10 is an error).

## Step 7 — Blocks and interleaving
Build: split data codewords into blocks per the table, compute EC per block,
interleave data codewords column-wise, then EC codewords column-wise; append remainder bits
(0 for v1, 7 for v2–6, 0 for v7–10).
Rust focus: iterator adapters (`chunks`, `zip`, `flat_map`), uneven block sizes.
Done when: the final codeword stream for a multi-block version (e.g. 5-Q) matches a worked example.

## Step 8 — Matrix and function patterns
Build: grid of size 17 + 4·version; finder patterns with separators, timing patterns,
alignment patterns (v2+, from position table, skipping overlaps with finders),
dark module at (4v + 9, 8), reserved areas for format info (and version info for v7+).
Track which modules are function (reserved) vs data.
Rust focus: flat `Vec` with `(row, col)` indexing vs `Vec<Vec<_>>`, `enum Module`,
`usize` arithmetic, a `Display` impl that prints the grid as text.
Done when: printing an empty v1 and v7 grid shows correct patterns (compare to the tutorial).

## Step 9 — Data placement
Build: zigzag placement from bottom-right, two columns at a time, upward/downward,
skipping column 6 and all reserved modules.
Rust focus: iterators producing coordinates, separating "walk" from "write".
Done when: every non-reserved module is filled exactly once.

## Step 10 — Masking and penalty scoring
Build: the 8 mask conditions applied only to data modules; the 4 penalty rules
(runs ≥5, 2×2 blocks, finder-like 1:1:3:1:1 patterns, dark/light balance);
choose the mask with the lowest score.
Rust focus: closures / `fn(usize, usize) -> bool` arrays, iterator chains, `min_by_key`.
Done when: penalty scores for a known example match published values.

## Step 11 — Format and version information
Build: format info = 2-bit EC level + 3-bit mask, BCH(15,5) with generator 0x537, XOR with 0x5412,
placed in both copies. Version info (v7+) = BCH(18,6) with generator 0x1F25, placed in both blocks.
Rust focus: bit manipulation, small pure functions.
Done when: computed format strings for all 32 combinations match the known table.

## Step 12 — Rendering
Build: PNG output via `image` with scale factor and a 4-module quiet zone.
Rust focus: using an external crate API, generics in `image`, file I/O, error context.
Done when: generated PNGs scan with a phone camera for several inputs and versions.

## Step 13 — CLI
Build: `clap` derive struct matching README: `<CONTENT>`, `-o/--output` (default `qrcode.png`),
`-s/--scale` (default 10), `-e/--ec-level` (default M); `main` returns `anyhow::Result<()>`.
User exercises: wire up `--output` and `--scale` yourself.
Rust focus: derive macros, `ValueEnum`, `anyhow` vs `thiserror`.
Done when: `cargo run -- "hi" -o x.png -s 5 -e H` writes a scannable PNG; `--help` matches README.

## Step 14 — Verification and polish
Build: integration tests that render to a buffer and decode with `rqrr` for many inputs
(short, long, Unicode, capacity boundaries, every EC level); optional comparison against the
`qrcode` crate's matrix. Doc comments, README update.
Rust focus: integration tests (`tests/`), `#[cfg(test)]`, rustdoc, clippy.
Done when: round-trip tests pass for all EC levels and versions 1–10.

## Step 15 — Versions 11–40
Build: extend tables, alignment positions and capacity tests to all 40 versions;
remainder bits for v14+ (3, 4, 3, 0).
Done when: round-trip tests from Step 14 pass up to v40.

## Stretch goals
- Numeric and alphanumeric modes; mixed-segment optimization
- Kanji mode (Shift-JIS detection, 13-bit packing)
- Read content from stdin (`echo "hello" | qrcode`)
- Custom foreground/background colors; SVG output; terminal output mode
- WebAssembly build; benchmarks with `criterion`
- Decoding (much harder; a project of its own)

---

## Session protocol
1. Start: "Let's do step N" (+ paste the relevant current code). One chat per step or sub-step.
2. Claude explains the concept, gives hints and signatures; I write the code.
3. Claude reviews my code: correctness, idiomatic Rust, tests. Paste full compiler errors.
4. End: Claude writes a PROGRESS.md entry (done, decisions, open questions, next step), and I commit.
