#!/bin/sh
# Generates the Rust documentation taking into account the private datatypes.
# Taken from https://github.com/rust-lang/cargo/issues/1520
cargo rustdoc --open -- --no-defaults --passes collapse-docs --passes unindent