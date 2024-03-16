<h1 align="center">NTgCalls-rs</h1>

<p align="center">
  <b>NTgCalls-rs is a Rust binding for the <a href="https://github.com/pytgcalls/ntgcalls">NTgCalls</a> C API. It provides a safe interface for using the NTgCalls library in Rust.</b></br>
</p>

<div align="center">

[![Build Status](https://github.com/YouKnow-sys/ntgcalls-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/YouKnow-sys/ntgcalls-rs/actions?workflow=Rust%20CI)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/YouKnow-sys/ntgcalls-rs/blob/master/LICENSE-MIT)

</div>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#usage">Usage</a> •
  <a href="#contributing">Contributing</a> •
  <a href="#license">License</a>
</p>

## Features

- Safe Rust interface for NTgCalls
- Automatic linking of NTgCalls shared library
- Cross-platform support

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
ntgcalls = { git = "https://github.com/YouKnow-sys/ntgcalls-rs.git" } # not yet released to crates.io
```

Then you can use the NTgCalls API:

```rust
use ntgcalls::NTgCall;

let ntgcalls = NTgCall::new();
// use NTgCalls API...
```

## Contributing

Contributions are welcome! Please open an issue or PR.

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE-MIT) for more details.
