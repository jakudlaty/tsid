#!/bin/bash

#Used for local development for quick checks
cargo test-all-features
cargo fmt --check
cargo msrv verify -- cargo check --all-features