#! /usr/bin/env sh
cargo tarpaulin --engine llvm --all-targets --line --ignore-tests --force-clean --all-features --all --locked --out Lcov --outut-dir ./target/coverage
