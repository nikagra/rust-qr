# TODO

## In Progress
- [ ] Reed-Solomon codeword generation — `encode` function, polynomial division in GF(256)

## Up Next

### Encoder core
- [ ] Mode representation — `enum Mode { Byte, Kanji }` + detection logic (is input valid Shift-JIS kanji?)
- [ ] Data encoding — Byte mode (UTF-8 bytes) and Kanji mode (13-bit packing)
- [ ] Version selection — pick smallest version that fits data at given EC level
- [ ] Codeword structuring — interleave data and EC blocks per spec tables

### Matrix construction
- [ ] `QrMatrix` type — 2D grid of modules with state (data / function / unknown)
- [ ] Function patterns — finder patterns (3 corners), separators, timing patterns
- [ ] Alignment patterns — version-dependent placement
- [ ] Format and version info — encode EC level + mask into reserved areas
- [ ] Data placement — zigzag column placement of data codewords

### Masking
- [ ] Implement all 8 mask patterns
- [ ] Penalty scoring — evaluate all 4 penalty rules, pick best mask

### Output
- [ ] Add `image` crate — render `QrMatrix` to PNG with configurable scale
- [ ] Add `clap` crate — CLI with `<CONTENT>`, `--output`, `--scale`, `--ec-level` flags
- [ ] **User exercise:** wire up `--output` / `-o` flag with default `qrcode.png`
- [ ] **User exercise:** wire up `--scale` / `-s` flag with default `10`

### Error handling
- [ ] Define error types with `thiserror` — invalid input, version overflow, etc.

## Future / Advanced
- [ ] Numeric and Alphanumeric encoding modes
- [ ] Versions 11–40
- [ ] Segment optimization (mixed-mode encoding)
- [ ] Color themes (custom foreground/background colors)
- [ ] SVG output format
- [ ] Read content from stdin (`echo "hello" | qrcode`)

## Done
- [x] Initial commit (README, LICENSE, .gitignore)
- [x] `cargo init` — Cargo.toml, edition 2024, binary crate named `qrcode`
- [x] GF(256) arithmetic module — multiply, divide, log/exp tables, tests
- [x] Reed-Solomon generator polynomial — `generator()`, root verification tests
