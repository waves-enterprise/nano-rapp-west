# Ledger Waves Enterprise Nano App

This is a Ledger devices wallet app for Waves Enterprise.

This application uses the official libraries [ledger-nanos-sdk](https://github.com/LedgerHQ/ledger-nanos-sdk) and [ledger-nanos-ui](https://github.com/LedgerHQ/ledger-nanos-ui).

### **!!!Important!!!**

Forks of official libraries are used at the moment, since the official libraries have flaws. This application is under development. Please see the list of what has already been implemented and what remains to be implemented.

- [x] Waves cryptography
    - [x] Signing transactions
- [ ] Transaction support (partially)
- [ ] UI
    - [ ] Menu
    - [x] Transaction view
- [ ] Tests
    - [x] Unit tests
    - [ ] Integration tests
- [ ] Documentation

## Building

### Prerequisites

This project will try to build [nanos-secure-sdk](https://github.com/LedgerHQ/nanos-secure-sdk), so you will need:

#### Linux

1. A standard ARM gcc (`sudo apt-get install gcc-arm-none-eabi binutils-arm-none-eabi`)
2. Cross compilation headers (`sudo apt-get install gcc-multilib`)
2. Python3 (`sudo apt-get install python3`)
3. Pip3 (`sudo apt-get install python3-pip`)

#### Windows

1. install [Clang](http://releases.llvm.org/download.html)
2. install an [ARM GCC toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads)
3. [Python](https://www.python.org/)


Other things you will need:
- [Cargo-ledger](https://github.com/LedgerHQ/cargo-ledger.git)
- [Speculos](https://github.com/LedgerHQ/speculos) (make sure you add speculos.py to your PATH by running `export PATH=/path/to/speculos:$PATH`)
- The correct target for rustc: `rustup target add thumbv6m-none-eabi`

You can build on either Windows or Linux with a simple `cargo build-nanos`.
It currently builds on stable.

## Loading

You can use [cargo-ledger](https://github.com/LedgerHQ/cargo-ledger.git) which builds, outputs a `hex` file and a manifest file for `ledgerctl`, and loads it on a device in a single `cargo ledger nanos --load` command in your app directory.

Some options of the manifest file can be configured directly in `Cargo.toml` under a custom section:

```yaml
[package.metadata.nanos]
curve = "secp256k1"
flags = "0x40"
icon = "btc.gif"
icon_small = ""
path = ""
```

## Testing

One can for example use [speculos](https://github.com/LedgerHQ/speculos)

`cargo run-nanos` defaults to running speculos on the generated binary with the appropriate flags, if `speculos.py` is in your `PATH`.

There is a small test script that sends some of the available commands in `test/test_cmds.py`, or raw APDUs that can be used with `ledgerctl`.

To run unit tests, use the `cargo test-nanos` command.
