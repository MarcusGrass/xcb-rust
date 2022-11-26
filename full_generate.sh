#!/bin/sh
set -e
cargo run -p xcb-xsd-parser
cargo run -p xcb-code-generator
cargo fmt --all
cargo test -p xcb-rust-protocol --features all
cargo test -p xcb-rust-connection