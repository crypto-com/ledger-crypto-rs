# Rust library for Ledger Crypto app
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![CircleCI](https://circleci.com/gh/Zondax/ledger-crypto-rs.svg?style=shield&circle-token=f2dc016ba835e5e4a8e48193a8cd1c6b96d22436)](https://circleci.com/gh/Zondax/ledger-crypto-rs)

This package provides a basic Rust client library to communicate with the Crypto App running in a Ledger Nano S/X devices

## Build

- Install rust using the instructions [here](https://www.rust-lang.org/tools/install)
- To build run:
```shell script
cargo build
```

## Run Tests
To run the tests

- Initialize your device with the test mnemonic. More info [here](https://github.com/zondax/ledger-crypto#how-to-prepare-your-development-device)
- run tests using: 
```shell script
cargo test --all
```
