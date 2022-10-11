#!/bin/bash

# See https://blog.logrocket.com/comparing-rust-supply-chain-safety-tools/

set -e

cargo install cargo-audit
cargo install cargo-deny
cargo install cargo-duplicates
cargo install cargo-license
cargo install cargo-outdated

