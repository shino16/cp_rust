#!/bin/bash
cargo install -q --git https://github.com/shino16/cargo-mod-deps.git --branch main
cd test
cargo-mod-deps --extract lib --target "$(basename "$1" | cut -f 1 -d '.')"
