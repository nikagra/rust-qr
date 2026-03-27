# qrcode

A command-line QR code generator written in Rust.

## Usage

```
qrcode [OPTIONS] <CONTENT>

Arguments:
  <CONTENT>  The text or URL to encode

Options:
  -o, --output <FILE>  Output image file [default: qrcode.png]
  -s, --scale <N>      Scale factor for the image [default: 10]
  -h, --help           Print help
  -V, --version        Print version
```

**Example:**

```sh
qrcode "https://example.com" -o my_qr.png
```

## Build

Requires [Rust](https://rustup.rs/) (edition 2021 or later).

```sh
git clone https://github.com/<you>/rust-qr.git
cd rust-qr
cargo build --release
./target/release/qrcode "hello world"
```

## License

MIT — see [LICENSE](LICENSE).
