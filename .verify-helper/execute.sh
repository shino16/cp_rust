#!/bin/bash
cargo run -q --release --manifest-path test/Cargo.toml --bin "$(basename "$1" | cut -f 1 -d '.')"