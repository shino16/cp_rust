#!/bin/bash
cd test
cargo-mod-deps --extract lib --target "$(basename "$1" | cut -f 1 -d '.')"
